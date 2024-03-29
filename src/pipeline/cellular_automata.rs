use std::borrow::Cow;
use bevy::app::{App, Plugin};
use bevy::asset::AssetServer;
use bevy::prelude::{Commands, FromWorld, Image, IntoSystemConfig, Res, Resource, World};
use bevy::render::render_asset::RenderAssets;
use bevy::render::{render_graph, RenderSet};
use bevy::render::render_graph::{NodeRunError, RenderGraphContext, SlotInfo};
use bevy::render::render_resource::*;
use bevy::render::renderer::{RenderContext, RenderDevice};
use crate::cellular_automata_image::CellularAutomataImage;
use crate::{CellularAutomataBuffers, SIMULATION_SIZE, WORKGROUP_SIZE};

pub struct CellularAutomataPipelinePlugin;
impl Plugin for CellularAutomataPipelinePlugin {
    fn build(&self, render_app: &mut App) {
        render_app
            .init_resource::<CellularAutomataPipeline>()
            .add_system(queue_bind_group.in_set(RenderSet::Queue));
    }
}

#[derive(Resource)]
pub struct CellularAutomataPipeline {
    init_pipeline: CachedComputePipelineId,
    update_pipeline: CachedComputePipelineId,
    bind_group_layout: BindGroupLayout,
}

impl FromWorld for CellularAutomataPipeline {
    fn from_world(world: &mut World) -> Self {
        let bind_group_layout = world.resource::<RenderDevice>()
            .create_bind_group_layout(&BindGroupLayoutDescriptor{
                label: Some("Cellular automata bind group layout"),
                entries: &[
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
                        ty: BindingType::StorageTexture {
                            access: StorageTextureAccess::ReadWrite,
                            format: TextureFormat::Rgba8Unorm,
                            view_dimension: TextureViewDimension::D2,
                        },
                        count: None,
                    }
                ],
            });

        let pipeline_cache = world.resource::<PipelineCache>();
        let shader = world
            .resource::<AssetServer>()
            .load("shaders/falling_sand.wgsl");

        let init_pipeline = pipeline_cache.queue_compute_pipeline(
            ComputePipelineDescriptor {
                label: Some(Cow::from("Falling sand init pipeline")),
                layout: vec![bind_group_layout.clone()],
                push_constant_ranges: vec![],
                shader: shader.clone(),
                shader_defs: vec![],
                entry_point: Cow::from("init")
            }
        );

        let update_pipeline= pipeline_cache.queue_compute_pipeline(
            ComputePipelineDescriptor {
                label: Some(Cow::from("Falling sand update pipeline")),
                layout: vec![bind_group_layout.clone()],
                push_constant_ranges: vec![],
                shader,
                shader_defs: vec![],
                entry_point: Cow::from("update"),
            }
        );

        CellularAutomataPipeline {
            bind_group_layout,
            init_pipeline,
            update_pipeline,
        }
    }
}

#[derive(Resource)]
pub(crate) struct CellularAutomataImageBindGroup(pub BindGroup);

pub fn queue_bind_group(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    pipeline: Res<CellularAutomataPipeline>,
    gpu_images: Res<RenderAssets<Image>>,
    cellular_automata_image: Res<CellularAutomataImage>,
    buffers: Res<CellularAutomataBuffers>,
) {
    let view = &gpu_images[&cellular_automata_image.0];
    let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
        label: Some("Cellular Automata Bind Group"),
        layout: &pipeline.bind_group_layout,
        entries: &[
            BindGroupEntry {
                binding: 0,
                resource: buffers.size_buffer.as_entire_binding(),
            },
            BindGroupEntry {
                binding: 1,
                resource: BindingResource::TextureView(&view.texture_view),
        }],
    });
    commands.insert_resource(CellularAutomataImageBindGroup(bind_group))
}

pub enum CellularAutomataState {
    Loading,
    Init,
    Update,
}

pub struct CellularAutomataNode {
    state: CellularAutomataState,
}

impl Default for CellularAutomataNode {
    fn default() -> Self {
        Self {
            state: CellularAutomataState::Loading,
        }
    }
}

impl render_graph::Node for CellularAutomataNode {
    fn update(&mut self, world: &mut World) {
        let pipeline = world.resource::<CellularAutomataPipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        match self.state {
            CellularAutomataState::Loading => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache.get_compute_pipeline_state(pipeline.init_pipeline) {
                    self.state = CellularAutomataState::Init;
                }
            }
            CellularAutomataState::Init => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache.get_compute_pipeline_state(pipeline.update_pipeline) {
                    self.state = CellularAutomataState::Update;
                }
            }
            CellularAutomataState::Update => {}
        }
    }

    fn run(
        &self,
        graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World
    ) -> Result<(), NodeRunError> {
        let texture_bind_group = &world.resource::<CellularAutomataImageBindGroup>().0;
        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = world.resource::<CellularAutomataPipeline>();

        let mut pass = render_context
            .command_encoder()
            .begin_compute_pass(&ComputePassDescriptor::default());

        pass.set_bind_group(0, texture_bind_group, &[]);

        match self.state {
            CellularAutomataState::Loading => {}
            CellularAutomataState::Init => {
                let init_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.init_pipeline)
                    .unwrap();
                pass.set_pipeline(init_pipeline);
                pass.dispatch_workgroups(
                    SIMULATION_SIZE.0 / WORKGROUP_SIZE,
                    SIMULATION_SIZE.1 / WORKGROUP_SIZE,
                    1,
                );
            }
            CellularAutomataState::Update => {
                let update_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.update_pipeline)
                    .unwrap();
                pass.set_pipeline(update_pipeline);
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