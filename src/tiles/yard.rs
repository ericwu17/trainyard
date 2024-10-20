use bevy::prelude::*;

use super::{connections::TileBorderState, construct_new_tile, TileConstructionInfo};
use crate::tiles::tile::Tile;
use crate::{NUM_COLS, NUM_ROWS};

#[derive(Component)]
pub struct Yard {
    pub tiles: Vec<Vec<Box<dyn Tile + Send + Sync>>>,
    pub borders: Vec<Vec<TileBorderState>>,
}

impl Yard {
    pub fn new(commands: &mut Commands) -> Self {
        let mut tiles: Vec<Vec<Box<dyn Tile + Send + Sync>>> = Vec::new();
        let border_states =
            vec![vec![TileBorderState::default(); NUM_COLS as usize]; NUM_ROWS as usize];

        for row in 0..NUM_ROWS {
            let mut row_vec: Vec<Box<dyn Tile + Send + Sync>> = Vec::new();
            for col in 0..NUM_COLS {
                let tile =
                    construct_new_tile(TileConstructionInfo::DrawableTile, row, col, commands);
                row_vec.push(tile);
            }
            tiles.push(row_vec);
        }

        let mut yard = Yard {
            tiles,
            borders: border_states,
        };
        yard.replace_tile(
            3,
            3,
            construct_new_tile(TileConstructionInfo::Rock, 3, 3, commands),
            commands,
        );

        return yard;
    }

    pub fn replace_tile(
        &mut self,
        row: usize,
        col: usize,
        tile: Box<dyn Tile + Send + Sync>,
        commands: &mut Commands,
    ) {
        self.tiles[row][col].despawn_entities_recursive(commands);
        self.tiles[row][col] = tile;
    }

    pub fn render(&mut self, commands: &mut Commands, asset_server: &Res<AssetServer>) {
        for row in &mut self.tiles {
            for tile in row {
                tile.render(commands, asset_server);
            }
        }
    }
}
