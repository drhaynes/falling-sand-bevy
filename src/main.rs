// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io::Cursor;

use winit::window::Icon;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::winit::WinitWindows;
use bevy::DefaultPlugins;
use bevy::render::extract_resource::ExtractResourcePlugin;

use falling_sand_game::GamePlugin;
use falling_sand_game::cellular_automata_image;
use falling_sand_game::cellular_automata_image::CellularAutomataImage;

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Falling Sand Game".to_string(),
                resolution: (800., 600.).into(),
                canvas: Some("#bevy".to_owned()),
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(setup)
        .add_plugin(GamePlugin)
        .add_system(set_window_icon.on_startup())
        .add_plugin(ExtractResourcePlugin::<CellularAutomataImage>::default())
        .run();
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let width: u32 = 800;
    let height: u32 = 600;
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

// Sets the icon on windows and X11
fn set_window_icon(
    windows: NonSend<WinitWindows>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    let primary_entity = primary_window.single();
    let primary = windows.get_window(primary_entity).unwrap();
    let icon_buf = Cursor::new(include_bytes!(
        "../build/macos/AppIcon.iconset/icon_256x256.png"
    ));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
}
