use bevy::prelude::*;

use super::tiles::{connections::TileBorderState, construct_new_tile, TileConstructionInfo};
use crate::level::{direction::Dir, tiles::tile::Tile, TrainCrashedEvent};
use crate::{NUM_COLS, NUM_ROWS, TILE_SIZE_PX};

#[derive(Component, Clone)]
pub struct Yard {
    pub tiles: Vec<Vec<Box<dyn Tile + Send + Sync>>>,
    pub borders: [[TileBorderState; NUM_COLS as usize]; NUM_ROWS as usize],
    pub base_entity: Entity,
}

#[derive(Component)]
pub struct TrainSprite;

#[derive(Component)]
pub struct YardEditedState(pub Yard);

#[derive(Event, Default)]
pub struct YardTickedEvent;

impl Yard {
    pub fn new(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Self {
        let base_entity = commands.spawn(SpatialBundle { ..default() }).id();
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
                commands
                    .entity(base_entity)
                    .push_children(&[tile.get_entity()]);
                row_vec.push(tile);
            }
            tiles.push(row_vec);
        }

        Yard {
            tiles,
            borders: border_states,
            base_entity,
        }
    }

    pub fn replace_tile(
        &mut self,
        row: usize,
        col: usize,
        tile: Box<dyn Tile + Send + Sync>,
        commands: &mut Commands,
    ) {
        self.tiles[row][col].despawn_entities_recursive(commands);
        commands
            .entity(self.base_entity)
            .push_children(&[tile.get_entity()]);
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
                        let train_entity = commands.spawn(bundle).id();
                        commands
                            .entity(self.base_entity)
                            .push_children(&[train_entity]);
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
                let mut t1 = outgoing_border_states[row][col].get_train(Dir::Up);
                let mut t2 = outgoing_border_states[row + 1][col].get_train(Dir::Down);
                if t1.is_some() && t2.is_some() {
                    let new_color = t1.unwrap().mix_with(t2.unwrap());
                    t1 = Some(new_color);
                    t2 = Some(new_color);
                }
                outgoing_border_states[row][col].set_train(t2, Dir::Up);
                outgoing_border_states[row + 1][col].set_train(t1, Dir::Down);
            }
        }
        for row in 0..(NUM_ROWS as usize) {
            for col in 0..((NUM_COLS - 1) as usize) {
                // horizontal swaps:
                let mut t1 = outgoing_border_states[row][col].get_train(Dir::Right);
                let mut t2 = outgoing_border_states[row][col + 1].get_train(Dir::Left);
                if t1.is_some() && t2.is_some() {
                    let new_color = t1.unwrap().mix_with(t2.unwrap());
                    t1 = Some(new_color);
                    t2 = Some(new_color);
                }
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
