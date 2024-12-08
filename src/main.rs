pub mod level;
pub mod ui;

use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;

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
    .add_systems(Update, despawn_empty_audio_sinks);

    app.run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
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
