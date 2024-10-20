pub mod connections;
pub mod drawable_tile;
pub mod rock_tile;
pub mod source_tile;
pub mod tile;
pub mod yard;

use crate::TILE_SIZE_PX;
use bevy::prelude::*;
use drawable_tile::DrawableTile;
use rock_tile::RockTile;
use tile::Tile;
use yard::Yard;

#[derive(Component)]
pub struct TilePosition {
    pub r: u8,
    pub c: u8,
}

#[derive(Component)]
pub struct TileComponent;

#[derive(Component)]
pub struct NonDrawableTile;

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_game_tiles)
            .add_systems(Update, render_yard);
    }
}

pub enum TileConstructionInfo {
    DrawableTile,
    Rock,
    Trainsource,
    Trainsink,
    Painter,
    Splitter,
}

pub fn construct_new_tile(
    tile_type: TileConstructionInfo,
    row: u8,
    col: u8,
    commands: &mut Commands,
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
        TileConstructionInfo::Trainsource => todo!(),
        TileConstructionInfo::Trainsink => todo!(),
        TileConstructionInfo::Rock => Box::new(RockTile::new(entity)),
        TileConstructionInfo::Painter => todo!(),
        TileConstructionInfo::Splitter => todo!(),
    }
}

fn spawn_game_tiles(mut commands: Commands) {
    let yard = Yard::new(&mut commands);
    commands.spawn(yard);

    // commands
    //     .get_entity(grid.tiles[3][3])
    //     .unwrap()
    //     .insert((NonDrawableTile, RockTile));
    // spawn_source_tile(
    //     commands,
    //     grid.tiles[3][4],
    //     Dir::Right,
    //     vec![
    //         TrainColor::Brown,
    //         TrainColor::Blue,
    //         TrainColor::Red,
    //         TrainColor::Yellow,
    //         TrainColor::Orange,
    //         TrainColor::Green,
    //         TrainColor::Purple,
    //     ],
    //     asset_server,
    // );
}

fn render_yard(
    mut commands: Commands,
    mut yard_query: Query<&mut Yard>,
    asset_server: Res<AssetServer>,
) {
    let yard = yard_query.single_mut().into_inner();

    yard.render(&mut commands, &asset_server);
}
