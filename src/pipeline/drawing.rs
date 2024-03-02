use std::borrow::Cow;
use bevy::app::{App, Plugin};
use bevy::math::Vec2;
use bevy::prelude::{AssetServer, Commands, FromWorld, Image, IntoSystemConfigs, Res, Resource, World};
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_phase::Draw;
use bevy::render::render_resource::{BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType, BufferBindingType, BufferSize, CachedComputePipelineId, CachedPipelineState, ComputePassDescriptor, ComputePipelineDescriptor, PipelineCache, PushConstantRange, ShaderStages, StorageTextureAccess, TextureFormat, TextureViewDimension};
use bevy::render::renderer::{RenderContext, RenderDevice};
use bevy::render::{render_graph, RenderSet};
use bevy::render::render_graph::{NodeRunError, RenderGraphContext, RenderLabel};
use bevy::render::RenderSet::Render;
use crate::cellular_automata_image::CellularAutomataImage;
use crate::input::DrawingParams;
use crate::{CellularAutomataBuffers, NUMBER_OF_CELLS, SIMULATION_SIZE, WORKGROUP_SIZE};
use super::cellular_automata::CellularAutomataImageBindGroup;

pub struct DrawingPipelinePlugin;
impl Plugin for DrawingPipelinePlugin {
    fn build(&self, render_app: &mut App) {
        render_app
            .init_resource::<DrawingPipeline>()
            .add_systems(Render, queue_drawing_bind_group.in_set(RenderSet::Queue));
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct DrawingPushConstants {
    draw_start: [f32; 2],
    draw_end: [f32; 2],
    draw_radius: f32,
}

impl DrawingPushConstants {
    pub fn new(draw_start: Vec2, draw_end: Vec2, draw_radius: f32) -> Self {
        Self {
            draw_radius,
            draw_start: draw_start.to_array(),
            draw_end: draw_end.to_array(),
        }
    }
}

#[derive(Resource)]
pub struct DrawingPipeline {
    drawing_pipeline: CachedComputePipelineId,
    drawing_bind_group_layout: BindGroupLayout,
}

impl FromWorld for DrawingPipeline {
    fn from_world(world: &mut World) -> Self {
        let pipeline_cache = world.resource::<PipelineCache>();
        
        let drawing_bind_group_layout = world
            .resource::<RenderDevice>()
            .create_bind_group_layout(
                "Drawing bind group layout",
                &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::COMPUTE,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: BufferSize::new((2 * std::mem::size_of::<u32>()) as _,),
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::COMPUTE,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Storage { read_only: false },
                            has_dynamic_offset: false,
                            min_binding_size: BufferSize::new((NUMBER_OF_CELLS * std::mem::size_of::<u32>()) as _,),
                        },
                        count: None,
                    },
                ],
            );
        
        let drawing_shader = world.resource::<AssetServer>().load("shaders/drawing.wgsl");
        
        let drawing_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some(Cow::from("Drawing pipeline")),
            layout: vec![drawing_bind_group_layout.clone()],
            push_constant_ranges: [PushConstantRange {
                stages: ShaderStages::COMPUTE,
                range: 0..std::mem::size_of::<DrawingPushConstants>() as u32,
            }].to_vec(),
            shader: drawing_shader,
            shader_defs: vec![],
            entry_point: Cow::from("draw"),
        });

        DrawingPipeline {
            drawing_pipeline,
            drawing_bind_group_layout,
        }
    }
}

#[derive(Resource)]
struct DrawingBindGroup(pub BindGroup);

pub fn queue_drawing_bind_group(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    pipeline: Res<DrawingPipeline>,
    buffers: Res<CellularAutomataBuffers>,
    parameters: Res<DrawingParams>,
) {
    // Ping-pong the buffers, alternating source and destination each frame
    let destination_buffer = if *parameters.frame_number.lock() % 2 == 0 {
        &buffers.simulation_buffers[1]
    } else {
        &buffers.simulation_buffers[0]
    };
    let drawing_bind_group = render_device.create_bind_group(
        "Drawing bind group",
        &pipeline.drawing_bind_group_layout,
        &[
            BindGroupEntry {
                binding: 0,
                resource: buffers.size_buffer.as_entire_binding(),
            },
            BindGroupEntry {
                binding: 1,
                resource: destination_buffer.as_entire_binding(),
            },
        ],
    );
    commands.insert_resource(DrawingBindGroup(drawing_bind_group))
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub struct DrawingLabel;

pub enum DrawingState {
    Loading,
    Update,
}

pub struct DrawingNode {
    state: DrawingState,
}

impl Default for DrawingNode {
    fn default() -> Self {
        Self {
            state: DrawingState::Loading,
        }
    }
}

impl render_graph::Node for DrawingNode {
    fn update(&mut self, _world: &mut World) {
        let pipeline_cache = _world.resource::<PipelineCache>();
        let pipeline = _world.resource::<DrawingPipeline>();

        match self.state {
            DrawingState::Loading => {
                if let CachedPipelineState::Ok(_) = pipeline_cache
                    .get_compute_pipeline_state(pipeline.drawing_pipeline) {
                    self.state = DrawingState::Update;
                }
            }
            DrawingState::Update => {}
        }
    }

    fn run(&self, graph: &mut RenderGraphContext, render_context: &mut RenderContext, world: &World) -> Result<(), NodeRunError> {
        let drawing_params = &world.resource::<DrawingParams>();

        if drawing_params.is_drawing {
            let texture_bind_group = &world.resource::<CellularAutomataImageBindGroup>().0;
            let drawing_bind_group = &world.resource::<DrawingBindGroup>().0;
            let pipeline_cache = world.resource::<PipelineCache>();
            let pipeline = world.resource::<DrawingPipeline>();

            let mut pass = render_context
                .command_encoder()
                .begin_compute_pass(&ComputePassDescriptor::default());

            pass.set_bind_group(0, texture_bind_group, &[]);

            match self.state {
                DrawingState::Loading => {}
                DrawingState::Update => {
                    let drawing_pipeline = pipeline_cache
                        .get_compute_pipeline(pipeline.drawing_pipeline)
                        .unwrap();

                    let brush_size: f32 = 10.0;
                    let push_constants =
                    DrawingPushConstants::new(drawing_params.canvas_position,
                                              drawing_params.previous_canvas_position,
                                              brush_size);

                    pass.set_pipeline(drawing_pipeline);
                    pass.set_bind_group(0, drawing_bind_group, &[]);
                    pass.set_push_constants(0, bytemuck::cast_slice(&[push_constants]));
                    pass.dispatch_workgroups(
                        SIMULATION_SIZE.0 / WORKGROUP_SIZE,
                        SIMULATION_SIZE.1 / WORKGROUP_SIZE,
                        1,
                    );
                }
            }
        }

        Ok(())
    }
}