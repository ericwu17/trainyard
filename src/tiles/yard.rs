use bevy::prelude::*;

use super::{connections::TileBorderState, construct_new_tile, TileConstructionInfo};
use crate::direction::Dir;
use crate::tiles::tile::Tile;
use crate::trains::TrainColor;
use crate::{NUM_COLS, NUM_ROWS};

#[derive(Component)]
pub struct Yard {
    pub tiles: Vec<Vec<Box<dyn Tile + Send + Sync>>>,
    pub borders: [[TileBorderState; NUM_COLS as usize]; NUM_ROWS as usize],
}

impl Yard {
    pub fn new(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Self {
        let mut tiles: Vec<Vec<Box<dyn Tile + Send + Sync>>> = Vec::new();
        let border_states: [[TileBorderState; NUM_COLS as usize]; NUM_ROWS as usize] =
            Default::default();

        for row in 0..NUM_ROWS {
            let mut row_vec: Vec<Box<dyn Tile + Send + Sync>> = Vec::new();
            for col in 0..NUM_COLS {
                let tile = construct_new_tile(
                    TileConstructionInfo::DrawableTile,
                    row,
                    col,
                    commands,
                    asset_server,
                );
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
            construct_new_tile(TileConstructionInfo::Rock, 3, 3, commands, asset_server),
            commands,
        );

        yard.replace_tile(
            3,
            4,
            construct_new_tile(
                TileConstructionInfo::SourceTile {
                    out: Dir::Right,
                    trains: vec![
                        TrainColor::Brown,
                        TrainColor::Blue,
                        TrainColor::Red,
                        TrainColor::Yellow,
                        TrainColor::Orange,
                        TrainColor::Green,
                        TrainColor::Purple,
                    ],
                },
                3,
                4,
                commands,
                asset_server,
            ),
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

    pub fn tick(&mut self) {
        let mut outgoing_border_states: [[TileBorderState; NUM_COLS as usize]; NUM_ROWS as usize] =
            Default::default();

        for row in 0..(NUM_ROWS as usize) {
            for col in 0..(NUM_COLS as usize) {
                let incoming_border_state = &self.borders[row][col];
                let tile = &mut self.tiles[row][col];

                let outgoing_border_state = tile.process_and_output(incoming_border_state.clone());
                outgoing_border_states[row][col] = outgoing_border_state;
            }
        }

        // swap borders, so outgoing becomes incoming:
        for row in 0..((NUM_ROWS - 1) as usize) {
            for col in 0..(NUM_COLS as usize) {
                // vertical swaps:
                let t1 = outgoing_border_states[row][col].get_train(Dir::Up);
                let t2 = outgoing_border_states[row + 1][col].get_train(Dir::Down);
                outgoing_border_states[row][col].set_train(t2, Dir::Up);
                outgoing_border_states[row + 1][col].set_train(t1, Dir::Down);
            }
        }
        for row in 0..(NUM_ROWS as usize) {
            for col in 0..((NUM_COLS - 1) as usize) {
                // horizontal swaps:
                let t1 = outgoing_border_states[row][col].get_train(Dir::Right);
                let t2 = outgoing_border_states[row][col + 1].get_train(Dir::Left);
                outgoing_border_states[row][col].set_train(t2, Dir::Right);
                outgoing_border_states[row][col + 1].set_train(t1, Dir::Left);
            }
        }
        self.borders = outgoing_border_states;
    }
}
