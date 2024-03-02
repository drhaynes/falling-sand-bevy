use bevy::app::{App, Plugin, Update};
use bevy::input::ButtonInput;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::math::Vec2;
use bevy::prelude::{Camera, EventReader, KeyCode, OrthographicProjection, Query, Res, Transform, With};
use bevy::time::Time;

const CAMERA_MOVE_SPEED: f32 = 500.0;
const CAMERA_SCALE_FACTOR: f32 = 1.05;
const CAMERA_MAX_ZOOM: f32 = 5.0;
const CAMERA_MIN_ZOOM: f32 = 0.15;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, camera_controller);
    }
}

fn camera_controller(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>
) {
    for (mut transform, mut ortho) in query.iter_mut() {
        let up = keyboard_input.pressed(KeyCode::KeyW);
        let down = keyboard_input.pressed(KeyCode::KeyS);
        let left = keyboard_input.pressed(KeyCode::KeyA);
        let right = keyboard_input.pressed(KeyCode::KeyD);

        let x_axis = right as i8 - left as i8;
        let y_axis = up as i8 - down as i8;
        let mut move_delta = Vec2::new(x_axis as f32, y_axis as f32);

        if move_delta != Vec2::ZERO {
            move_delta /= move_delta.length();

            let z = transform.translation.z;
            transform.translation += move_delta.extend(z) * CAMERA_MOVE_SPEED * time.delta_seconds();
            transform.translation.z = z;
        }

        for MouseWheel { x, y, unit, .. } in mouse_wheel_events.read() {
            let mut x_scroll = 0.0;
            let mut y_scroll = 0.0;

            match unit {
                MouseScrollUnit::Line => {
                    x_scroll += x;
                    y_scroll += y;
                }
                MouseScrollUnit::Pixel => {
                    const PIXELS_PER_LINE: f32 = 38.0;
                    x_scroll += x / PIXELS_PER_LINE;
                    y_scroll += x / PIXELS_PER_LINE;
                }
            }

            if x_scroll != 0.0 || y_scroll != 0.0 {
                if y_scroll < 0.0 {
                    ortho.scale *= CAMERA_SCALE_FACTOR;
                } else {
                    ortho.scale *= 1.0 / CAMERA_SCALE_FACTOR;
                }

                ortho.scale = ortho.scale.clamp(CAMERA_MIN_ZOOM, CAMERA_MAX_ZOOM);
            }
        }
    }
}