pub mod cursor;
pub mod direction;
pub mod level;
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
        //level::LevelPlugin,
        bevy_inspector_egui::quick::WorldInspectorPlugin::default()
            .run_if(input_toggle_active(false, KeyCode::Escape)),
    ))
    .add_systems(Startup, spawn_camera);

    app.run();
}

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}
