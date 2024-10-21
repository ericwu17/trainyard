pub mod cursor;
pub mod direction;
pub mod level;
pub mod tiles;
pub mod trains;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use level::LevelPlugin;

const NUM_ROWS: u8 = 7;
const NUM_COLS: u8 = 7;
const TILE_SIZE_PX: f32 = 96.0;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, LevelPlugin))
        .add_systems(Startup, spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}
