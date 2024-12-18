pub mod connections;
pub mod drawable_tile;
pub mod rock_tile;
pub mod sink_tile;
pub mod source_tile;
pub mod tile;
pub mod tile_animations;

use bevy::{prelude::*, ui::UiSystem};

use crate::{
    level::{
        direction::Dir,
        restore_yard_edited_state,
        trains::TrainColor,
        yard::YardEditedState,
        yard::{Yard, YardTickedEvent},
        LevelSet, LevelState,
    },
    ui::level::YardPlaceholderNode,
    NUM_COLS, NUM_ROWS, TILE_SIZE_PX,
};
use drawable_tile::DrawableTile;
use rock_tile::RockTile;
use sink_tile::SinkTile;
use source_tile::SourceTile;
use tile::Tile;

use super::{
    persistence::{GameLevelProgress, LevelProgress},
    yard::YardMidTickEvent,
    CurrentLevelName, LevelStateIsRunning, YardTickTimer,
};
pub struct TilePlugin;

#[derive(Component)]
pub struct YardComponent;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<YardTickedEvent>()
            .add_event::<YardMidTickEvent>()
            .add_plugins(tile_animations::TileAnimationPlugin)
            .add_systems(
                OnEnter(LevelState::None),
                (
                    restore_yard_edited_state,
                    persist_yard_and_despawn_game_tiles,
                )
                    .chain(),
            )
            .add_systems(OnEnter(LevelState::Editing), restore_yard_edited_state)
            .add_systems(
                Update,
                ((
                    render_yard,
                    render_yard_trains.run_if(in_state(LevelStateIsRunning::Running)),
                )
                    .chain(),)
                    .in_set(LevelSet),
            )
            .add_systems(
                PostUpdate,
                adjust_yard_position_to_match_placeholder
                    .in_set(LevelSet)
                    .after(UiSystem::Layout)
                    .before(TransformSystem::TransformPropagate),
            );
    }
}

pub enum TileConstructionInfo {
    DrawableTile,
    Rock,
    SourceTile {
        out: Dir,
        trains: Vec<TrainColor>,
    },
    SinkTile {
        ins: [bool; 4],
        trains: Vec<TrainColor>,
    },
    Painter,
    Splitter,
}

pub fn construct_new_tile(
    tile_type: TileConstructionInfo,
    row: u8,
    col: u8,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) -> Box<dyn Tile + Send + Sync> {
    let x = col as f32 * TILE_SIZE_PX + TILE_SIZE_PX / 2.0;
    let y = row as f32 * TILE_SIZE_PX + TILE_SIZE_PX / 2.0;

    let entity = commands
        .spawn((
            Transform::from_xyz(x, y, 0.0),
            Visibility::default(),
            Name::new(format!("Base entity at row {} column {}", row, col)),
        ))
        .id();

    match tile_type {
        TileConstructionInfo::DrawableTile => Box::new(DrawableTile::new(entity)),
        TileConstructionInfo::Rock => Box::new(RockTile::new(entity)),
        TileConstructionInfo::SourceTile { out, trains } => {
            Box::new(SourceTile::new(out, trains, entity, commands, asset_server))
        }
        TileConstructionInfo::SinkTile { ins, trains } => {
            Box::new(SinkTile::new(ins, trains, entity, commands, asset_server))
        }
        TileConstructionInfo::Painter => todo!(),
        TileConstructionInfo::Splitter => todo!(),
    }
}

fn persist_yard_and_despawn_game_tiles(
    mut commands: Commands,
    yard_query: Query<(Entity, &Yard)>,
    yard_edit_state_query: Query<Entity, With<YardEditedState>>,
    mut persistence: ResMut<GameLevelProgress>,
    curr_lvl_name: Res<CurrentLevelName>,
    lvl_state: Res<State<LevelState>>,
) {
    for (entity, yard) in yard_query.iter() {
        commands.entity(entity).despawn_recursive();
        if let Some(name) = curr_lvl_name.0.as_ref() {
            let mut has_won = *lvl_state.get() == LevelState::Won;
            let drawn_tracks = yard.get_progress();

            if persistence.0.remove(name).unwrap_or_default().has_won {
                has_won = true;
            }

            let progress = LevelProgress {
                has_won,
                drawn_tracks,
            };
            persistence.0.insert(name.to_string(), progress);
        }
    }
    for entity in yard_edit_state_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn render_yard(
    mut commands: Commands,
    mut yard_query: Query<&mut Yard>,
    asset_server: Res<AssetServer>,
) {
    let yard = yard_query.single_mut().into_inner();
    yard.render(&mut commands, &asset_server);
}

fn render_yard_trains(
    mut commands: Commands,
    mut yard_query: Query<&mut Yard>,
    asset_server: Res<AssetServer>,
    timer_q: Query<&YardTickTimer>,
) {
    if let Ok(yard) = yard_query.get_single_mut() {
        let yard = yard.into_inner();

        let mut time_within_tick = 0.0;
        if let Ok(yard_tick_timer) = timer_q.get_single() {
            time_within_tick = yard_tick_timer.timer.elapsed().as_micros() as f32 / 1000000.0;
        }
        yard.render_trains(&mut commands, &asset_server, time_within_tick);
    }
}

pub fn adjust_yard_position_to_match_placeholder(
    yard_query: Query<Entity, With<Yard>>,
    placeholder_query: Query<(&Transform, &Parent), With<YardPlaceholderNode>>,
    parent_query: Query<(&Transform, &Parent)>,
    mut commands: Commands,
    _window_query: Query<&Window, With<bevy::window::PrimaryWindow>>,
) {
    for yard_entity in yard_query.iter() {
        println!("adjusting yard position to match placeholder...");
        if let Ok((placeholder_transform, parent)) = placeholder_query.get_single() {
            let mut final_transform: Vec2 = Vec2::ZERO;

            // add up the transforms of all parent entities
            let mut curr_transform = placeholder_transform;
            let mut curr_parent = parent;
            final_transform += curr_transform.translation.truncate();
            loop {
                if let Ok((transform, parent)) = parent_query.get(curr_parent.get()) {
                    curr_transform = transform;
                    curr_parent = parent;
                    final_transform += curr_transform.translation.truncate();
                } else {
                    break;
                }
            }

            let x = final_transform.x - (NUM_COLS as f32 * TILE_SIZE_PX) / 2.0 + 120.0;
            let y = final_transform.y - (NUM_ROWS as f32 * TILE_SIZE_PX) / 2.0;
            commands
                .entity(yard_entity)
                .insert(Transform::from_xyz(x, y, 0.0));
        }
    }
}
