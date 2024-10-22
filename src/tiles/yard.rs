use bevy::prelude::*;

use super::{connections::TileBorderState, construct_new_tile, TileConstructionInfo};
use crate::direction::Dir;
use crate::level::TrainCrashedEvent;
use crate::tiles::tile::Tile;
use crate::trains::TrainColor;
use crate::{NUM_COLS, NUM_ROWS, TILE_SIZE_PX};

#[derive(Component, Clone)]
pub struct Yard {
    pub tiles: Vec<Vec<Box<dyn Tile + Send + Sync>>>,
    pub borders: [[TileBorderState; NUM_COLS as usize]; NUM_ROWS as usize],
}

#[derive(Component)]
pub struct TrainSprite;

#[derive(Event, Default)]
pub struct YardTickedEvent;

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
                    trains: vec![TrainColor::Blue],
                },
                3,
                4,
                commands,
                asset_server,
            ),
            commands,
        );
        yard.replace_tile(
            5,
            6,
            construct_new_tile(
                TileConstructionInfo::SourceTile {
                    out: Dir::Down,
                    trains: vec![TrainColor::Red],
                },
                5,
                6,
                commands,
                asset_server,
            ),
            commands,
        );

        yard.replace_tile(
            3,
            2,
            construct_new_tile(
                TileConstructionInfo::SinkTile {
                    ins: [false, false, false, true],
                    trains: vec![TrainColor::Purple, TrainColor::Purple],
                },
                3,
                2,
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

    pub fn reset_tile_inner_entities(&mut self, commands: &mut Commands) {
        for row in &mut self.tiles {
            for tile in row {
                tile.reset_inner_entities(commands);
            }
        }
    }

    pub fn render_trains(&mut self, commands: &mut Commands, asset_server: &Res<AssetServer>) {
        for r in 0..(NUM_ROWS as usize) {
            for c in 0..(NUM_COLS as usize) {
                for dir in Dir::all_dirs() {
                    let x = c as f32 * TILE_SIZE_PX + TILE_SIZE_PX / 2.0;
                    let y = r as f32 * TILE_SIZE_PX + TILE_SIZE_PX / 2.0;

                    let base_transform = Transform::from_xyz(x, y, 0.5);

                    if let Some(train_color) = self.borders[r][c].get_train(dir) {
                        let bundle = (
                            SpriteBundle {
                                transform: base_transform
                                    * Transform::from_rotation(Quat::from(dir.flip()))
                                    * Transform::from_xyz(0.0, -TILE_SIZE_PX / 2.0, 0.0),
                                texture: asset_server.load("sprites/Train.png"),
                                sprite: Sprite {
                                    color: Color::from(train_color),
                                    ..default()
                                },
                                ..default()
                            },
                            TrainSprite,
                            Name::new(format!("{} train", train_color.to_str())),
                        );
                        commands.spawn(bundle);
                    }
                }
            }
        }
    }

    pub fn tick(&mut self, crashed_event: &mut EventWriter<TrainCrashedEvent>) {
        let mut outgoing_border_states: [[TileBorderState; NUM_COLS as usize]; NUM_ROWS as usize] =
            Default::default();

        for row in 0..(NUM_ROWS as usize) {
            for col in 0..(NUM_COLS as usize) {
                let incoming_border_state = &self.borders[row][col];
                let tile = &mut self.tiles[row][col];

                let outgoing_border_state =
                    tile.process_and_output(incoming_border_state.clone(), crashed_event);
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

    // check if all source tiles are empty, all destination tiles are empty, and if all borders are empty.
    pub fn has_won(&self) -> bool {
        for row in 0..(NUM_ROWS as usize) {
            for col in 0..(NUM_COLS as usize) {
                if !self.tiles[row][col].has_no_remaining_trains() {
                    return false;
                }
                if !self.borders[row][col].is_empty() {
                    return false;
                }
            }
        }
        true
    }
}
