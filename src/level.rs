pub mod cursor;
pub mod direction;
pub mod loader;
pub mod persistence;
pub mod tiles;
pub mod trains;
pub mod yard;

use bevy::{audio::Volume, prelude::*};
use persistence::{GameLevelProgress, LevelProgress};
use trains::TrainColor;

use crate::{
    ui::{level::speed_slider::TrainSpeed, level_picker::StartLevelEvent},
    TILE_SIZE_PX,
};
use loader::StockLevelInfos;
use std::time::Duration;
use tiles::{
    tile::TileEvent,
    tile_animations::{FloatingFadingAnimationComponent, SrinkToNoneAnimationComponent},
    YardComponent,
};
use yard::{TileEventWithLocation, Yard, YardEditedState, YardMidTickEvent, YardTickedEvent};

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

#[derive(Default, Resource)]
pub struct CurrentLevelName(pub Option<String>);

#[derive(Component)]
pub struct YardTickTimer {
    timer: Timer,
    half_timer: Timer,
}

#[derive(Event, Default)]
pub struct WinLevelEvent;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct LevelSet;
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct LevelEditingSet;
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct LevelRunningSet;

#[derive(Component)]
pub struct EndTickEvent;

#[derive(Component)]
pub struct MidTickEvent;

#[derive(Component)]
pub struct StartTickEvent;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            cursor::CursorPlugin,
            tiles::TilePlugin,
            persistence::PersistencePlugin,
        ))
        .add_event::<WinLevelEvent>()
        .add_event::<TileEventWithLocation>()
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
        .add_systems(
            Update,
            (
                update_level_state_from_keypress,
                tick_yard_tick_timer.in_set(LevelRunningSet),
                win_event_handler.run_if(on_event::<WinLevelEvent>()),
            )
                .in_set(LevelSet),
        )
        .add_systems(
            Update,
            level_start_event_handler
                .after(LevelSet)
                .run_if(on_event::<StartLevelEvent>()),
        )
        .init_resource::<CurrentLevelName>();
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
    mut yard_edited_state_query: Query<(&mut YardEditedState, Entity)>,
) {
    if let Ok((mut old_yard, old_yard_entity)) = yard_edited_state_query.get_single_mut() {
        let yard = yard_query.single_mut().into_inner();
        old_yard
            .0
            .reset_tile_inner_entities_and_train_entities(&mut commands);
        yard.despawn_trains(&mut commands);

        *yard = old_yard.0.clone();

        commands.entity(old_yard_entity).despawn();
    }
}

pub fn spawn_timer(mut commands: Commands, timer_query: Query<Entity, With<YardTickTimer>>) {
    for entity in timer_query.iter() {
        commands.entity(entity).despawn();
    }

    let mut timer: Timer = Timer::new(Duration::from_secs(1), TimerMode::Repeating);
    let half_timer = Timer::new(Duration::from_millis(500), TimerMode::Once);
    timer.tick(Duration::from_micros(999999)); // make the timer just about to expire
    commands.spawn(YardTickTimer { timer, half_timer });
}

pub fn despawn_timer(
    mut commands: Commands,
    timer_query: Query<Entity, Or<(With<YardTickTimer>, With<MidTickEvent>, With<EndTickEvent>)>>,
) {
    for entity in timer_query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn handle_tile_event(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    event: &TileEventWithLocation,
    yard: &mut Yard,
    next_state: &mut ResMut<NextState<LevelState>>,
    has_crashed: &mut bool,
) {
    info!("handling event {:?}", event);
    match event.event {
        TileEvent::SinkReceivedTrain(train_color) => {
            play_color_sound(commands, asset_server, train_color);
        }
        TileEvent::MixColors(train_color, (dx, dy)) => {
            play_color_sound(commands, asset_server, train_color);
            spawn_sparkles(
                commands,
                asset_server,
                yard.base_entity,
                event.row,
                event.col,
                (dx, dy),
                train_color,
            );
        }
        TileEvent::CrashedOnEdge(train_color, dir) => {
            next_state.set(LevelState::RunningCrashed);
            *has_crashed = true;
            commands.spawn(AudioBundle {
                source: asset_server.load("audio/crash.ogg"),
                ..default()
            });
            spawn_smoke(
                commands,
                asset_server,
                yard.base_entity,
                event.row,
                event.col,
                dir.to_local_coords_of_edge(),
                train_color,
            );
        }
        TileEvent::ShrinkAwayInnerEntity(entity) => {
            commands
                .entity(entity)
                .insert(SrinkToNoneAnimationComponent(1.0));
        }
        TileEvent::SwitchActivePassive => {
            yard.switch_active_passive(event.row, event.col);
            commands.spawn(AudioBundle {
                source: asset_server.load("audio/switch_track.ogg"),
                ..default()
            });
        }
    };
}

pub fn play_color_sound(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    color: TrainColor,
) {
    let asset_path = match color {
        TrainColor::Brown => "audio/train_brown.ogg",
        TrainColor::Red => "audio/train_red.ogg",
        TrainColor::Blue => "audio/train_blue.ogg",
        TrainColor::Yellow => "audio/train_yellow.ogg",
        TrainColor::Purple => "audio/train_purple.ogg",
        TrainColor::Green => "audio/train_green.ogg",
        TrainColor::Orange => "audio/train_orange.ogg",
    };
    commands.spawn(AudioBundle {
        source: asset_server.load(asset_path),
        ..default()
    });
}

pub fn spawn_smoke(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    yard_entity: Entity,
    row: usize,
    col: usize,
    dir: (f32, f32),
    color: TrainColor,
) {
    let y = TILE_SIZE_PX * (row as f32) + TILE_SIZE_PX * dir.1;
    let x = TILE_SIZE_PX * (col as f32) + TILE_SIZE_PX * dir.0;

    let color: Color = color.into();

    for _ in 0..8 {
        let id = commands
            .spawn((
                SpriteBundle {
                    texture: asset_server.load("sprites/Smoke.png"),
                    transform: Transform::from_xyz(x, y, 5.0),
                    sprite: Sprite { color, ..default() },
                    ..default()
                },
                FloatingFadingAnimationComponent::new(),
            ))
            .id();
        commands.entity(yard_entity).push_children(&[id]);
    }
}

pub fn spawn_sparkles(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    yard_entity: Entity,
    row: usize,
    col: usize,
    dir: (f32, f32),
    color: TrainColor,
) {
    let y = TILE_SIZE_PX * (row as f32) + TILE_SIZE_PX * dir.1;
    let x = TILE_SIZE_PX * (col as f32) + TILE_SIZE_PX * dir.0;

    let color: Color = color.into();

    for _ in 0..5 {
        let id = commands
            .spawn((
                SpriteBundle {
                    texture: asset_server.load("sprites/Fire_small.png"),
                    transform: Transform::from_xyz(x, y, 5.0),
                    sprite: Sprite { color, ..default() },
                    ..default()
                },
                FloatingFadingAnimationComponent::new(),
            ))
            .id();
        commands.entity(yard_entity).push_children(&[id]);
    }
    for _ in 0..5 {
        let id = commands
            .spawn((
                SpriteBundle {
                    texture: asset_server.load("sprites/Fire.png"),
                    transform: Transform::from_xyz(x, y, 5.0),
                    sprite: Sprite { color, ..default() },
                    ..default()
                },
                FloatingFadingAnimationComponent::new(),
            ))
            .id();
        commands.entity(yard_entity).push_children(&[id]);
    }
}

pub fn tick_yard_tick_timer(
    mut commands: Commands,
    asset_server: Res<AssetServer>,

    mut q: Query<&mut YardTickTimer>,
    time: Res<Time>,
    level_state: ResMut<State<LevelState>>,
    mut next_state: ResMut<NextState<LevelState>>,
    mut yard_query: Query<&mut Yard>,
    mut event_yard_ticked: EventWriter<YardTickedEvent>,
    mut event_yard_mid_tick: EventWriter<YardMidTickEvent>,
    mut win_event: EventWriter<WinLevelEvent>,

    mid_tick_events_q: Query<(Entity, &TileEventWithLocation), With<MidTickEvent>>,
    end_tick_events_q: Query<(Entity, &TileEventWithLocation), With<EndTickEvent>>,

    train_speed: Res<TrainSpeed>,
) {
    let yard_tick_timer = q.single_mut().into_inner();

    let delta_ns = time.delta().as_nanos();
    let delta_ns_for_tick = (delta_ns as f32 * 10.0 * train_speed.0) as u64;
    yard_tick_timer
        .timer
        .tick(Duration::from_nanos(delta_ns_for_tick));
    yard_tick_timer
        .half_timer
        .tick(Duration::from_nanos(delta_ns_for_tick));

    let mut has_crashed = false;

    if yard_tick_timer.timer.just_finished() {
        let yard = yard_query.single_mut().into_inner();

        for (entity, ev) in end_tick_events_q.iter() {
            commands.entity(entity).despawn();
            handle_tile_event(
                &mut commands,
                &asset_server,
                ev,
                yard,
                &mut next_state,
                &mut has_crashed,
            );
        }

        let process_tick_results = yard.tick();

        for e in process_tick_results.start_tick_events {
            handle_tile_event(
                &mut commands,
                &asset_server,
                &e,
                yard,
                &mut next_state,
                &mut has_crashed,
            );
        }

        for e in process_tick_results.mid_tick_events {
            commands.spawn((MidTickEvent, e));
        }
        for e in process_tick_results.end_tick_events {
            commands.spawn((EndTickEvent, e));
        }

        event_yard_ticked.send_default();

        if !has_crashed && *level_state.get() == LevelState::RunningNotCrashed && yard.has_won() {
            win_event.send_default();
        }
        yard_tick_timer.half_timer.reset();
    } else if yard_tick_timer.half_timer.just_finished() {
        let yard = yard_query.single_mut().into_inner();
        for (entity, ev) in mid_tick_events_q.iter() {
            commands.entity(entity).despawn();
            handle_tile_event(
                &mut commands,
                &asset_server,
                ev,
                yard,
                &mut next_state,
                &mut has_crashed,
            );
        }

        event_yard_mid_tick.send_default();
    }
}

pub fn win_event_handler(
    mut win_event: EventReader<WinLevelEvent>,
    mut next_state: ResMut<NextState<LevelState>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut persistence: ResMut<GameLevelProgress>,
    curr_lvl_name: Res<CurrentLevelName>,
    yard_edit_state_query: Query<&YardEditedState>,
) {
    if win_event.read().count() > 0 {
        let yard = yard_edit_state_query.single();

        next_state.set(LevelState::Won);
        commands.spawn(AudioBundle {
            source: asset_server.load("audio/win_level.ogg"),
            settings: PlaybackSettings {
                volume: Volume::new(0.2),
                ..default()
            },
            ..default()
        });

        // persist current level progress
        if let Some(name) = curr_lvl_name.0.as_ref() {
            let name = name.to_string();
            let has_won = true;

            let drawn_tracks = yard.0.get_progress();

            let progress = LevelProgress {
                has_won,
                drawn_tracks,
            };
            persistence.0.insert(name, progress);
        }
    }
}

pub fn level_start_event_handler(
    mut commands: Commands,
    mut start_event_reader: EventReader<StartLevelEvent>,
    mut next_level_state: ResMut<NextState<LevelState>>,
    asset_server: Res<AssetServer>,
    levels: Res<StockLevelInfos>,
    persistence: Res<GameLevelProgress>,
    mut level_name: ResMut<CurrentLevelName>,
    yard_query: Query<Entity, With<Yard>>,
    yard_edit_state_query: Query<Entity, With<YardEditedState>>,
) {
    for entity in yard_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in yard_edit_state_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    for start_event in start_event_reader.read() {
        let mut found_level = false;
        for level in levels.0.iter() {
            if level.name == start_event.level_name {
                let yard_entity = level.to_yard(
                    &mut commands,
                    &asset_server,
                    persistence.0.get(&start_event.level_name),
                );

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
        level_name.0 = Some(start_event.level_name.clone());
    }
}
