pub mod connections;
pub mod drawable_tile;
pub mod rock_tile;
pub mod source_tile;
pub mod tile;
pub mod yard;

use crate::{NUM_COLS, NUM_ROWS, TILE_SIZE_PX};
use bevy::prelude::*;
use connections::TileBorderState;
use drawable_tile::DrawableTile;
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

fn spawn_game_tiles(mut commands: Commands) {
    let mut tiles: Vec<Vec<Box<dyn Tile + Send + Sync>>> = Vec::new();
    let border_states =
        vec![vec![TileBorderState::default(); NUM_COLS as usize]; NUM_ROWS as usize];

    for row in 0..NUM_ROWS {
        let mut row_vec: Vec<Box<dyn Tile + Send + Sync>> = Vec::new();
        for col in 0..NUM_COLS {
            let x = col as f32 * TILE_SIZE_PX + TILE_SIZE_PX / 2.0;
            let y = row as f32 * TILE_SIZE_PX + TILE_SIZE_PX / 2.0;
            let entity = commands
                .spawn((SpriteBundle {
                    transform: Transform::from_xyz(x, y, 0.0),
                    ..default()
                },))
                .id();

            let tile = DrawableTile::new(entity);

            row_vec.push(Box::new(tile));
        }
        tiles.push(row_vec);
    }

    let yard = Yard {
        tiles,
        borders: border_states,
    };

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
