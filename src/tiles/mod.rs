pub mod connections;
pub mod drawable_tile;
pub mod rock_tile;
pub mod sink_tile;
pub mod source_tile;
pub mod tile;
pub mod yard;

use crate::{
    direction::Dir,
    level::{restore_yard_edited_state, LevelSet, LevelState, YardEditedState},
    trains::TrainColor,
    ui::level::YardPlaceholderNode,
    NUM_COLS, NUM_ROWS, TILE_SIZE_PX,
};
use bevy::prelude::*;
use drawable_tile::DrawableTile;
use rock_tile::RockTile;
use sink_tile::SinkTile;
use source_tile::SourceTile;
use tile::Tile;
use yard::{TrainSprite, Yard, YardTickedEvent};

pub struct TilePlugin;

#[derive(Component)]
pub struct YardComponent;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<YardTickedEvent>()
            .add_systems(OnEnter(LevelState::None), despawn_game_tiles)
            .add_systems(
                OnEnter(LevelState::Editing),
                refresh_yard_trains.after(restore_yard_edited_state),
            )
            .add_systems(
                Update,
                (
                    (
                        render_yard,
                        refresh_yard_trains.run_if(on_event::<YardTickedEvent>()),
                    )
                        .chain(),
                    adjust_yard_position_to_match_placeholder,
                )
                    .in_set(LevelSet),
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
            SpatialBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            },
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

fn despawn_game_tiles(
    mut commands: Commands,
    yard_query: Query<Entity, With<Yard>>,
    yard_edit_state_query: Query<Entity, With<YardEditedState>>,
) {
    for entity in yard_query.iter() {
        commands.entity(entity).despawn_recursive();
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

fn refresh_yard_trains(
    mut commands: Commands,
    mut yard_query: Query<&mut Yard>,
    trains_query: Query<Entity, With<TrainSprite>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(yard) = yard_query.get_single_mut() {
        let yard = yard.into_inner();
        // despawn all train entities
        for entity in trains_query.iter() {
            commands.entity(entity).despawn_recursive();
        }

        // respawn all train entities
        yard.render_trains(&mut commands, &asset_server);
    }
}

pub fn adjust_yard_position_to_match_placeholder(
    yard_query: Query<Entity, With<Yard>>,
    placeholder_query: Query<&GlobalTransform, With<YardPlaceholderNode>>,
    mut commands: Commands,
) {
    if let Ok(yard_entity) = yard_query.get_single() {
        if let Ok(placeholder_transform) = placeholder_query.get_single() {
            let Vec2 { x, y } = placeholder_transform.translation().truncate();
            let x = x - (NUM_COLS as f32 * TILE_SIZE_PX) / 2.0;
            let y = y - (NUM_ROWS as f32 * TILE_SIZE_PX) / 2.0;
            commands.entity(yard_entity).insert(TransformBundle {
                local: Transform::from_xyz(x, y, 0.0),
                ..default()
            });
        }
    }
}
