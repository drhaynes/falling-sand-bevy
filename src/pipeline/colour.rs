use std::borrow::Cow;
use std::cell::Cell;
use bevy::app::App;
use bevy::asset::AssetServer;
use bevy::prelude::{Commands, FromWorld, Image, IntoSystemConfig, Plugin, Res, World};
use bevy::render::render_resource::{BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType, BufferBindingType, BufferSize, CachedComputePipelineId, CachedPipelineState, ComputePassDescriptor, ComputePipelineDescriptor, PipelineCache, ShaderStages, StorageTextureAccess, TextureFormat, TextureViewDimension};
use bevy::render::{render_graph, RenderSet};
use bevy::ecs::system::Resource;
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_graph::{NodeRunError, RenderGraphContext};
use bevy::render::renderer::{RenderContext, RenderDevice};
use crate::cellular_automata_image::CellularAutomataImage;
use crate::{CellularAutomataBuffers, NUMBER_OF_CELLS, SIMULATION_SIZE, WORKGROUP_SIZE};
use crate::input::DrawingParams;
use crate::pipeline::cellular_automata::CellularAutomataImageBindGroup;

pub struct ColourPipelinePlugin;
impl Plugin for ColourPipelinePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ColourPipeline>()
            .add_system(queue_colour_bind_group.in_set(RenderSet::Queue));
    }
}

#[derive(Resource)]
pub struct ColourPipeline {
    colour_pipeline: CachedComputePipelineId,
    colour_bind_group_layout: BindGroupLayout,
}

impl FromWorld for ColourPipeline {
    fn from_world(world: &mut World) -> Self {
        let pipeline_cache = world.resource::<PipelineCache>();

        let colour_bind_group_layout =
        world
            .resource::<RenderDevice>()
            .create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("Falling sand colour bind group layout"),
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::COMPUTE,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: BufferSize::new((2 * std::mem::size_of::<u32>()) as _,)
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::COMPUTE,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Storage {
                                read_only: false,
                            },
                            has_dynamic_offset: false,
                            min_binding_size: BufferSize::new(
                                (2 * NUMBER_OF_CELLS * std::mem::size_of::<u32>()) as _,
                            ),
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 2,
                        visibility: ShaderStages::COMPUTE,
                        ty: BindingType::StorageTexture {
                            access: StorageTextureAccess::ReadWrite,
                            format: TextureFormat::Rgba8Unorm,
                            view_dimension: TextureViewDimension::D2,
                        },
                        count: None,
                    },
                ],
            });

        let colour_shader = world.resource::<AssetServer>().load("shaders/colour.wgsl");

        let colour_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some(Cow::Borrowed("Falling sand colour pipeline")),
            layout: vec![colour_bind_group_layout.clone()],
            push_constant_ranges: vec![],
            shader: colour_shader,
            shader_defs: vec![],
            entry_point: Cow::from("colour"),
        });

        ColourPipeline {
            colour_pipeline,
            colour_bind_group_layout
        }
    }
}

#[derive(Resource)]
struct ColourBindGroup(pub BindGroup);

pub fn queue_colour_bind_group(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    buffers: Res<CellularAutomataBuffers>,
    pipeline: Res<ColourPipeline>,
    gpu_images: Res<RenderAssets<Image>>,
    cellular_automata_image: Res<CellularAutomataImage>,
    params: Res<DrawingParams>,
) {
    let view = &gpu_images[&cellular_automata_image.0];
    let colour_bind_group = render_device.create_bind_group(&BindGroupDescriptor {
        label: Some("Falling sand colour bind group"),
        layout: &pipeline.colour_bind_group_layout,
        entries: &[
            BindGroupEntry {
                binding: 0,
                resource: buffers.size_buffer.as_entire_binding(),
            },
            BindGroupEntry {
                binding: 1,
                resource: buffers.simulation_buffers[*params.frame_number.lock() % 2].as_entire_binding(),
            },
            BindGroupEntry {
                binding: 2,
                resource: BindingResource::TextureView(&view.texture_view),
            },
        ],
    });
    commands.insert_resource(ColourBindGroup(colour_bind_group));
}

pub enum ColourState {
    Loading,
    Update,
}

pub struct ColourNode {
    state: ColourState,
}

impl Default for ColourNode {
    fn default() -> Self {
        Self {
            state: ColourState::Loading
        }
    }
}

impl render_graph::Node for ColourNode {
    fn update(&mut self, world: &mut World) {
        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = world.resource::<ColourPipeline>();

        match self.state {
            ColourState::Loading => {
                if let CachedPipelineState::Ok(_) = pipeline_cache.get_compute_pipeline_state(pipeline.colour_pipeline) {
                    self.state = ColourState::Update;
                }
            }
            ColourState::Update => {}
        }
    }

    fn run(&self, graph: &mut RenderGraphContext, render_context: &mut RenderContext, world: &World) -> Result<(), NodeRunError> {
        let texture_bind_group = &world.resource::<CellularAutomataImageBindGroup>().0;
        let colour_bind_group = &world.resource::<ColourBindGroup>().0;
        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = world.resource::<ColourPipeline>();

        let mut pass = render_context
            .command_encoder()
            .begin_compute_pass(&(ComputePassDescriptor::default()));

        pass.set_bind_group(0, texture_bind_group, &[]);

        match self.state {
            ColourState::Loading => {}
            ColourState::Update => {
                let colour_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.colour_pipeline)
                    .unwrap();

                pass.set_pipeline(colour_pipeline);
                pass.set_bind_group(0, colour_bind_group, &[]);
                pass.dispatch_workgroups(
                    SIMULATION_SIZE.0 / WORKGROUP_SIZE,
                    SIMULATION_SIZE.1 / WORKGROUP_SIZE,
                    1,
                );
            }
        }
        Ok(())
    }
}
