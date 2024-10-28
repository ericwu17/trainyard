use bevy::prelude::*;

use super::persistence::LevelProgress;
use super::tiles::connections::TileConnections;
use super::tiles::tile::TileTrainActivity;
use super::tiles::{connections::TileBorderState, construct_new_tile, TileConstructionInfo};
use super::trains::TrainColor;
use crate::level::{direction::Dir, tiles::tile::Tile, TrainCrashedEvent};
use crate::{NUM_COLS, NUM_ROWS, TILE_SIZE_PX};

#[derive(Clone)]
pub struct TrainActivityWithLocation {
    row: usize,
    col: usize,
    activity: TileTrainActivity,
}

#[derive(Component, Clone)]
pub struct Yard {
    pub tiles: Vec<Vec<Box<dyn Tile + Send + Sync>>>,
    pub borders: [[TileBorderState; NUM_COLS as usize]; NUM_ROWS as usize],
    pub base_entity: Entity,
    pub train_entities: Vec<Entity>,
    pub train_activity: Vec<TrainActivityWithLocation>,
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
            train_entities: Vec::new(),
            train_activity: Vec::new(),
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

    pub fn reset_tile_inner_entities_and_train_entities(&mut self, commands: &mut Commands) {
        // restore inner entities
        for row in &mut self.tiles {
            for tile in row {
                tile.reset_inner_entities(commands);
            }
        }
        self.despawn_trains(commands);
    }

    pub fn despawn_trains(&mut self, commands: &mut Commands) {
        while let Some(entity) = self.train_entities.pop() {
            commands.entity(self.base_entity).remove_children(&[entity]);
            commands.entity(entity).despawn();
        }
    }

    pub fn render_trains(&mut self, commands: &mut Commands, asset_server: &Res<AssetServer>) {
        self.despawn_trains(commands);

        for train_activity in self.train_activity.iter() {
            let r = train_activity.row;
            let c = train_activity.col;
            let Some(dir) = train_activity.activity.to_dir else {
                continue;
            };
            let train_color = train_activity.activity.end_color;

            let x = c as f32 * TILE_SIZE_PX + TILE_SIZE_PX / 2.0;
            let y = r as f32 * TILE_SIZE_PX + TILE_SIZE_PX / 2.0;

            let base_transform = Transform::from_xyz(x, y, 0.5);

            let bundle = (
                SpriteBundle {
                    transform: base_transform
                        * Transform::from_rotation(Quat::from(dir))
                        * Transform::from_xyz(0.0, TILE_SIZE_PX / 2.0, 0.0),
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
            self.train_entities.push(train_entity);
        }
    }

    pub fn tick(&mut self, crashed_event: &mut EventWriter<TrainCrashedEvent>) {
        let mut outgoing_border_states: [[TileBorderState; NUM_COLS as usize]; NUM_ROWS as usize] =
            Default::default();
        self.train_activity = Vec::new();

        for row in 0..(NUM_ROWS as usize) {
            for col in 0..(NUM_COLS as usize) {
                let incoming_border_state = &self.borders[row][col];
                let tile = &mut self.tiles[row][col];

                let train_tile_activity =
                    tile.process_and_output(incoming_border_state.clone(), crashed_event);
                let mut outgoing_border_state = TileBorderState::new();

                for dir_u8 in 0..4 {
                    let out_dir = Dir::from(dir_u8);
                    let mut colors_to_mix: Vec<TrainColor> = Vec::with_capacity(2);
                    for train_coming_thru in train_tile_activity.iter() {
                        if train_coming_thru.to_dir == Some(out_dir) {
                            colors_to_mix.push(train_coming_thru.end_color);
                        }
                    }
                    if !colors_to_mix.is_empty() {
                        outgoing_border_state
                            .add_train(TrainColor::mix_many(colors_to_mix), out_dir);
                    }
                }
                for train_activity in train_tile_activity {
                    self.train_activity.push(TrainActivityWithLocation {
                        row,
                        col,
                        activity: train_activity,
                    });
                }
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

    pub fn apply_progress(&mut self, progress: &LevelProgress) {
        for row in 0..NUM_ROWS {
            for col in 0..NUM_COLS {
                let index = row * NUM_ROWS + col;

                let conns = TileConnections::from_data(progress.drawn_tracks[index as usize]);

                if !conns.get_active_conn().is_empty() {
                    let (d1, d2) = conns.get_passive_conn().get_dirs();
                    self.tiles[row as usize][col as usize].add_connection(d1, d2);
                }
                if !conns.get_active_conn().is_empty() {
                    let (d1, d2) = conns.get_active_conn().get_dirs();
                    self.tiles[row as usize][col as usize].add_connection(d1, d2);
                }
            }
        }
    }

    pub fn get_progress(&self) -> Vec<u8> {
        let mut res = Vec::new();
        for row in 0..NUM_ROWS {
            for col in 0..NUM_COLS {
                res.push(self.tiles[row as usize][col as usize].get_connection_data());
            }
        }
        res
    }
}
