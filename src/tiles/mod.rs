pub mod connections;
pub mod drawable_tile;
pub mod rock_tile;
pub mod sink_tile;
pub mod source_tile;
pub mod tile;
pub mod yard;

use crate::{direction::Dir, trains::TrainColor, TILE_SIZE_PX};
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use drawable_tile::DrawableTile;
use rock_tile::RockTile;
use sink_tile::SinkTile;
use source_tile::SourceTile;
use tile::Tile;
use yard::{despawn_train_entities, Yard};

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_game_tiles)
            .add_systems(Update, (despawn_train_entities, render_yard).chain())
            .add_systems(
                Update,
                update_yard.run_if(input_just_pressed(KeyCode::KeyN)),
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
        .spawn((SpriteBundle {
            transform: Transform::from_xyz(x, y, 0.0),
            ..default()
        },))
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

fn spawn_game_tiles(mut commands: Commands, asset_server: Res<AssetServer>) {
    let yard = Yard::new(&mut commands, &asset_server);
    commands.spawn(yard);
}

fn render_yard(
    mut commands: Commands,
    mut yard_query: Query<&mut Yard>,
    asset_server: Res<AssetServer>,
) {
    let yard = yard_query.single_mut().into_inner();

    yard.render(&mut commands, &asset_server);
    yard.render_trains(&mut commands, &asset_server);
}

fn update_yard(mut yard_query: Query<&mut Yard>) {
    let yard = yard_query.single_mut().into_inner();

    yard.tick();
}
