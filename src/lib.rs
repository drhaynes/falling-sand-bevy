mod actions;
mod audio;
mod loading;
mod menu;
pub mod cellular_automata_image;
mod pipeline;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::diagnostic::Diagnostics;
use bevy::prelude::*;
use bevy::render::extract_resource::ExtractResourcePlugin;
use bevy::render::{RenderApp, RenderSet};
use bevy::render::render_graph::RenderGraph;
use bevy::window::PrimaryWindow;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::cellular_automata_image::CellularAutomataImage;
use crate::pipeline::{CellularAutomataNode, CellularAutomataPipeline};

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

const SIMULATION_SIZE: (u32, u32) = (1280, 720);
const WORKGROUP_SIZE: u32 = 8;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_startup_system(setup)
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(ExtractResourcePlugin::<CellularAutomataImage>::default());

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default())
                .add_system(display_fps);
        }

        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .init_resource::<CellularAutomataPipeline>()
            .add_system(pipeline::queue_bind_group.in_set(RenderSet::Queue));

        let mut render_graph = render_app.world.resource_mut::<RenderGraph>();
        render_graph.add_node("cellular_automata", CellularAutomataNode::default());
        render_graph.add_node_edge(
            "cellular_automata",
            bevy::render::main_graph::node::CAMERA_DRIVER,
        );
    }
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let width: u32 = SIMULATION_SIZE.0;
    let height: u32 = SIMULATION_SIZE.1;
    let image = cellular_automata_image::create_image(width, height);
    let image = images.add(image);

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(width as f32, height as f32)),
            ..default()
        },
        texture: image.clone(),
        ..default()
    });

    commands.insert_resource(cellular_automata_image::CellularAutomataImage(image))
}

fn display_fps(diagnostics: Res<Diagnostics>, mut windows: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = windows.get_single_mut() {
        if let Some(fps_raw) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(fps_smoothed) = fps_raw.smoothed() {
                window.title = format!("Falling Sand Game ({fps_smoothed:.2} fps)")
            }
        }
    }
}
