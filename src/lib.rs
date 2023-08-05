mod actions;
mod audio;
mod loading;
mod menu;
pub mod cellular_automata_image;
mod pipeline;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::diagnostic::Diagnostics;
use bevy::prelude::*;
use bevy::render::RenderApp;
use bevy::window::PrimaryWindow;
use crate::pipeline::CellularAutomataPipeline;

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

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(InternalAudioPlugin);

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default())
                .add_system(display_fps);
        }

        let render_app = app.sub_app_mut(RenderApp);
        render_app.init_resource::<CellularAutomataPipeline>();
    }
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
