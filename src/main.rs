pub mod cursor;
pub mod direction;
pub mod level;
mod level_loader;
pub mod tiles;
pub mod trains;
pub mod ui;

use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

const NUM_ROWS: u8 = 7;
const NUM_COLS: u8 = 7;
const TILE_SIZE_PX: f32 = 96.0;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins,
        ui::TrainyardUIPlugin,
        level::LevelPlugin,
        level_loader::LevelLoaderPlugin,
        bevy_inspector_egui::quick::WorldInspectorPlugin::default()
            .run_if(input_toggle_active(false, KeyCode::Escape)),
    ))
    .add_systems(Startup, spawn_camera)
    .add_systems(
        Update,
        keep_camera_centered.run_if(on_event::<bevy::window::WindowResized>()),
    );

    app.run();
}

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

fn keep_camera_centered(
    mut commands: Commands,
    camera_query: Query<Entity, With<Camera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // this system keeps the camera center pointed at (window.width()/2, window.height()/2)
    // so that the bottom left corner of the screen is always (0, 0).

    let window = window_query.single();
    let camera_entity = camera_query.single();

    commands.entity(camera_entity).insert(Transform::from_xyz(
        window.width() / 2.0,
        window.height() / 2.0,
        0.0,
    ));
}
