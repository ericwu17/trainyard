use std::time::Duration;

use bevy::prelude::*;

use crate::{
    cursor::CursorPlugin,
    level_loader::StockLevelInfos,
    tiles::{
        yard::{Yard, YardTickedEvent},
        TilePlugin, YardComponent,
    },
    ui::level_picker::StartLevelEvent,
};

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum LevelState {
    #[default]
    None,
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
                    LevelRunningSet.run_if(in_state(LevelState::Running)),
                ),
            )
            .insert_state(LevelState::None)
            .add_systems(OnEnter(LevelState::Editing), restore_yard_edited_state)
            .add_systems(
                OnEnter(LevelState::Running),
                (spawn_timer, save_yard_edited_state),
            )
            .add_systems(OnExit(LevelState::Running), despawn_timer)
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
            LevelState::None => {
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
    mut yard_query: Query<&mut Yard>,
    mut event_yard_ticked: EventWriter<YardTickedEvent>,
    mut crashed_event: EventWriter<TrainCrashedEvent>,
    mut win_event: EventWriter<WinLevelEvent>,
) {
    let yard_tick_timer = q.single_mut().into_inner();
    yard_tick_timer.timer.tick(time.delta() * 2);

    if yard_tick_timer.timer.just_finished() {
        let yard = yard_query.single_mut().into_inner();
        yard.tick(&mut crashed_event);
        event_yard_ticked.send_default();
        if yard.has_won() {
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
        next_state.set(LevelState::Crashed);
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
