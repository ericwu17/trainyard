pub mod cursor;
pub mod direction;
pub mod tiles;
pub mod trains;
pub mod yard;

use bevy::prelude::*;
use std::time::Duration;

use crate::{
    level_loader::StockLevelInfos,
    ui::{level::speed_slider::TrainSpeed, level_picker::StartLevelEvent},
};
use cursor::CursorPlugin;
use tiles::{TilePlugin, YardComponent};
use yard::{Yard, YardEditedState, YardTickedEvent};

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum LevelState {
    #[default]
    None,
    Editing,
    RunningNotCrashed,
    RunningCrashed,
    Won,
}
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum LevelStateIsRunning {
    Running,
    NotRunning,
}
impl ComputedStates for LevelStateIsRunning {
    type SourceStates = LevelState;

    fn compute(sources: Self::SourceStates) -> Option<Self> {
        if sources == LevelState::RunningCrashed || sources == LevelState::RunningNotCrashed {
            Some(Self::Running)
        } else {
            Some(Self::NotRunning)
        }
    }
}

#[derive(Component)]
pub struct YardTickTimer {
    timer: Timer,
}

#[derive(Event, Default)]
pub struct TrainCrashedEvent;

#[derive(Event, Default)]
pub struct WinLevelEvent;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct LevelSet;
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct LevelEditingSet;
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct LevelRunningSet;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CursorPlugin, TilePlugin))
            .add_event::<TrainCrashedEvent>()
            .add_event::<WinLevelEvent>()
            .configure_sets(
                Update,
                (
                    LevelSet.run_if(not(in_state(LevelState::None))),
                    LevelEditingSet.run_if(in_state(LevelState::Editing)),
                    LevelRunningSet.run_if(in_state(LevelStateIsRunning::Running)),
                ),
            )
            .insert_state(LevelState::None)
            .add_computed_state::<LevelStateIsRunning>()
            .add_systems(
                OnEnter(LevelStateIsRunning::Running),
                (spawn_timer, save_yard_edited_state),
            )
            .add_systems(OnExit(LevelStateIsRunning::Running), despawn_timer)
            .add_systems(OnEnter(LevelState::Editing), restore_yard_edited_state)
            .add_systems(
                Update,
                (
                    update_level_state_from_keypress,
                    tick_yard_tick_timer.in_set(LevelRunningSet),
                    crashed_event_handler.run_if(on_event::<TrainCrashedEvent>()),
                    win_event_handler.run_if(on_event::<WinLevelEvent>()),
                )
                    .in_set(LevelSet),
            )
            .add_systems(
                Update,
                level_start_event_handler.run_if(on_event::<StartLevelEvent>()),
            );
    }
}

pub fn update_level_state_from_keypress(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    state: Res<State<LevelState>>,
    mut next_state: ResMut<NextState<LevelState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        toggle_level_state(&state, &mut next_state);
    }
}
pub fn toggle_level_state(
    state: &Res<State<LevelState>>,
    next_state: &mut ResMut<NextState<LevelState>>,
) {
    match state.get() {
        LevelState::Editing => {
            next_state.set(LevelState::RunningNotCrashed);
        }
        LevelState::RunningNotCrashed => {
            next_state.set(LevelState::Editing);
        }
        LevelState::RunningCrashed => {
            next_state.set(LevelState::Editing);
        }
        _ => {
            // do nothing
        }
    };
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
    if let Ok((old_yard, old_yard_entity)) = yard_edited_state_query.get_single() {
        let yard = yard_query.single_mut().into_inner();
        *yard = old_yard.0.clone();
        yard.reset_tile_inner_entities(&mut commands);

        commands.entity(old_yard_entity).despawn();
    }
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
    level_state: Res<State<LevelState>>,
    mut yard_query: Query<&mut Yard>,
    mut event_yard_ticked: EventWriter<YardTickedEvent>,
    mut crashed_event: EventWriter<TrainCrashedEvent>,
    mut win_event: EventWriter<WinLevelEvent>,
    train_speed: Res<TrainSpeed>,
) {
    let yard_tick_timer = q.single_mut().into_inner();

    let delta_ns = time.delta().as_nanos();
    let delta_ns_for_tick = (delta_ns as f32 * 10.0 * train_speed.0) as u64;
    yard_tick_timer
        .timer
        .tick(Duration::from_nanos(delta_ns_for_tick));

    if yard_tick_timer.timer.just_finished() {
        let yard = yard_query.single_mut().into_inner();
        yard.tick(&mut crashed_event);
        event_yard_ticked.send_default();

        if *level_state.get() == LevelState::RunningNotCrashed && yard.has_won() {
            win_event.send_default();
        }
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
        next_state.set(LevelState::RunningCrashed);
        commands.spawn(AudioBundle {
            source: asset_server.load("audio/crash.ogg"),
            ..default()
        });
    }
}

pub fn win_event_handler(
    mut win_event: EventReader<WinLevelEvent>,
    mut crashed_event: EventReader<TrainCrashedEvent>,
    mut next_state: ResMut<NextState<LevelState>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if win_event.read().count() > 0 && crashed_event.read().count() == 0 {
        next_state.set(LevelState::Won);
        commands.spawn(AudioBundle {
            source: asset_server.load("audio/win_level.ogg"),
            ..default()
        });
    }
}

pub fn level_start_event_handler(
    mut commands: Commands,
    mut start_event_reader: EventReader<StartLevelEvent>,
    mut next_level_state: ResMut<NextState<LevelState>>,
    asset_server: Res<AssetServer>,
    levels: Res<StockLevelInfos>,
) {
    for start_event in start_event_reader.read() {
        let mut found_level = false;
        for level in levels.0.iter() {
            if level.name == start_event.level_name {
                let yard_entity = level.to_yard(&mut commands, &asset_server);

                let yard_bundle = (YardComponent, Name::new("The Yard"));
                commands.entity(yard_entity).insert(yard_bundle);
                found_level = true;
                break;
            }
        }
        if !found_level {
            panic!(
                "could not find a level with name {}",
                start_event.level_name
            );
        }
        next_level_state.set(LevelState::Editing);
    }
}
