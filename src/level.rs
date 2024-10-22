use std::time::Duration;

use bevy::prelude::*;

use crate::{
    cursor::CursorPlugin,
    tiles::{
        yard::{Yard, YardTickedEvent},
        TilePlugin,
    },
};

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum LevelState {
    #[default]
    Editing,
    Running,
    Crashed,
    Won,
}

#[derive(Component)]
pub struct YardTickTimer {
    timer: Timer,
}

#[derive(Component)]
pub struct YardEditedState(Yard);

#[derive(Event, Default)]
pub struct TrainCrashedEvent;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CursorPlugin, TilePlugin))
            .add_event::<TrainCrashedEvent>()
            .insert_state(LevelState::Editing)
            .add_systems(Update, update_state_from_keypress)
            .add_systems(
                OnEnter(LevelState::Running),
                (spawn_timer, save_yard_edited_state),
            )
            .add_systems(
                OnExit(LevelState::Running),
                (despawn_timer, restore_yard_edited_state),
            )
            .add_systems(
                Update,
                tick_yard_tick_timer.run_if(in_state(LevelState::Running)),
            )
            .add_systems(
                Update,
                crashed_event_handler.run_if(on_event::<TrainCrashedEvent>()),
            );
    }
}

pub fn update_state_from_keypress(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    state: Res<State<LevelState>>,
    mut next_state: ResMut<NextState<LevelState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match state.get() {
            LevelState::Editing => {
                next_state.set(LevelState::Running);
            }
            LevelState::Running => {
                next_state.set(LevelState::Editing);
            }
            LevelState::Crashed => {
                next_state.set(LevelState::Editing);
            }
            LevelState::Won => {
                // do nothing
            }
        };
    }
}

pub fn save_yard_edited_state(mut commands: Commands, yard_query: Query<&Yard>) {
    let yard = yard_query.single();
    commands.spawn(YardEditedState(yard.clone()));
}

pub fn restore_yard_edited_state(
    mut commands: Commands,
    mut yard_query: Query<&mut Yard>,
    yard_edited_state_query: Query<(&YardEditedState, Entity)>,
) {
    let (old_yard, old_yard_entity) = yard_edited_state_query.single();

    let yard = yard_query.single_mut().into_inner();
    *yard = old_yard.0.clone();
    yard.reset_tile_inner_entities(&mut commands);

    commands.entity(old_yard_entity).despawn();
}

pub fn spawn_timer(mut commands: Commands, timer_query: Query<Entity, With<YardTickTimer>>) {
    for entity in timer_query.iter() {
        commands.entity(entity).despawn();
    }
    commands.spawn((YardTickTimer {
        timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
    },));
}

pub fn despawn_timer(mut commands: Commands, timer_query: Query<Entity, With<YardTickTimer>>) {
    for entity in timer_query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn tick_yard_tick_timer(
    mut q: Query<&mut YardTickTimer>,
    time: Res<Time>,
    mut yard_query: Query<&mut Yard>,
    mut event_yard_ticked: EventWriter<YardTickedEvent>,
    mut crashed_event: EventWriter<TrainCrashedEvent>,
) {
    let yard_tick_timer = q.single_mut().into_inner();
    yard_tick_timer.timer.tick(time.delta() * 2);

    if yard_tick_timer.timer.just_finished() {
        let yard = yard_query.single_mut().into_inner();
        yard.tick(&mut crashed_event);
        event_yard_ticked.send_default();
    }
}

pub fn crashed_event_handler(
    mut crashed_event: EventReader<TrainCrashedEvent>,
    mut next_state: ResMut<NextState<LevelState>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut did_crash = false;

    for _ in crashed_event.read() {
        did_crash = true;
    }

    if did_crash {
        next_state.set(LevelState::Crashed);
        commands.spawn(AudioBundle {
            source: asset_server.load("audio/crash.ogg"),
            ..default()
        });
    }
}
