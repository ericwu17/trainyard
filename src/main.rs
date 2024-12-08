pub mod level;
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
        DefaultPlugins.set(bevy::log::LogPlugin {
            //level: bevy::log::Level::TRACE,
            ..default()
        }),
        ui::TrainyardUIPlugin,
        level::LevelPlugin,
        level::loader::LevelLoaderPlugin,
        bevy_inspector_egui::quick::WorldInspectorPlugin::default()
            .run_if(input_toggle_active(false, KeyCode::Escape)),
    ))
    .add_systems(Startup, spawn_camera)
    .add_systems(
        Update,
        (
            despawn_empty_audio_sinks,
            keep_camera_centered.run_if(on_event::<bevy::window::WindowResized>),
        ),
    );

    app.run();
}

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single();
    commands.spawn((
        Camera2d,
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
    ));
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

fn despawn_empty_audio_sinks(
    mut commands: Commands,
    audio_sink_query: Query<(Entity, &AudioSink)>,
) {
    for (entity, audio_sink) in audio_sink_query.iter() {
        if audio_sink.empty() {
            commands.entity(entity).despawn();
        }
    }
}
