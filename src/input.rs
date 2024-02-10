use std::sync::{Arc};
use bevy::app::{App, Plugin};
use bevy::input::ButtonState;
use bevy::input::mouse::MouseButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{Camera, EventReader, GlobalTransform, MouseButton, Query, ResMut, Resource, With};
use bevy::render::extract_resource::ExtractResource;
use bevy::window::{PrimaryWindow, Window};
use winit::event::Event;

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DrawingParams>()
            .add_system(update_input_state);
    }
}

#[derive(Default, Resource, ExtractResource, Clone)]
pub struct DrawingParams {
    pub canvas_position: Vec2,
    pub is_drawing: bool,
    pub previous_canvas_position: Vec2,
    pub frame_number: Arc<parking_lot::Mutex<usize>>,
}

pub fn update_input_state(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut input_state: ResMut<DrawingParams>,
    camera_query: Query<(&Camera, &GlobalTransform), With<crate::MainCamera>>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
) {
    let primary_window = window_query.single();
    let (camera, camera_transform) = camera_query.single();

    for event in mouse_button_input_events.iter() {
        if event.button == MouseButton::Left {
            let is_drawing = event.state == ButtonState::Pressed;
            input_state.is_drawing = is_drawing;
        }
    }

    if let Some(world_position) = primary_window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        input_state.previous_canvas_position = input_state.canvas_position;
        input_state.canvas_position = world_position_to_canvas_position(world_position * Vec2::new(1.0, -1.0));
    }
}

fn world_position_to_canvas_position(world_position: Vec2) -> Vec2 {
    world_position + Vec2::new(
        crate::SIMULATION_SIZE.0 as f32 / 2.0,
        crate::SIMULATION_SIZE.1 as f32 / 2.0,
    )
}
