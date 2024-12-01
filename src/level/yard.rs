use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;

use super::persistence::LevelProgress;
use super::tiles::connections::TileConnections;
use super::tiles::tile::{TileEvent, TileTrainActivity};
use super::tiles::{connections::TileBorderState, construct_new_tile, TileConstructionInfo};
use super::trains::TrainColor;
use crate::level::{direction::Dir, tiles::tile::Tile};
use crate::{NUM_COLS, NUM_ROWS, TILE_SIZE_PX};

#[derive(Clone)]
pub struct TrainActivityWithLocation {
    row: usize,
    col: usize,
    activity: TileTrainActivity,
}

#[derive(Clone, Debug, Event)]
pub struct TileEventWithLocation {
    pub row: usize,
    pub col: usize,
    pub event: TileEvent,
}

#[derive(Clone)]
pub struct YardProcessTickResult {
    pub start_tick_events: Vec<TileEventWithLocation>,
    pub mid_tick_events: Vec<TileEventWithLocation>,
    pub end_tick_events: Vec<TileEventWithLocation>,
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

#[derive(Event, Default)]
pub struct YardMidTickEvent;

impl Yard {
    pub fn new(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Self {
        let base_entity = commands
            .spawn((Transform::default(), Visibility::default()))
            .id();
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
                    .add_children(&[tile.get_entity()]);
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
            .add_children(&[tile.get_entity()]);
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

    /// time_within_tick is a float from 0 to 1
    pub fn render_trains(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        time_within_tick: f32,
    ) {
        if self.train_entities.len() != self.train_activity.len() {
            self.despawn_trains(commands);

            for _ in self.train_activity.iter() {
                let bundle = (
                    Sprite {
                        image: asset_server.load("sprites/Train.png"),
                        color: Color::srgba(0.0, 0.0, 0.0, 0.0),
                        ..default()
                    },
                    TrainSprite,
                    Name::new("train"),
                );
                let train_entity = commands.spawn(bundle).id();
                commands
                    .entity(self.base_entity)
                    .add_children(&[train_entity]);
                self.train_entities.push(train_entity);
            }
        }
        for (entity, activity) in self.train_entities.iter().zip(self.train_activity.iter()) {
            let r = activity.row;
            let c = activity.col;
            let x = c as f32 * TILE_SIZE_PX + TILE_SIZE_PX / 2.0;
            let y = r as f32 * TILE_SIZE_PX + TILE_SIZE_PX / 2.0;
            let t = time_within_tick;
            let base_transform = Transform::from_xyz(x, y, 0.5);

            let to_dir = activity.activity.to_dir;
            let from_dir = activity.activity.from_dir;
            if to_dir.is_none() && time_within_tick > 0.5 {
                continue;
            }
            if from_dir.is_none() && time_within_tick < 0.5 {
                continue;
            }
            let (to_dir, from_dir) = match (to_dir, from_dir) {
                (None, None) => {
                    continue;
                }
                (None, Some(from_dir)) => (from_dir.flip(), from_dir),
                (Some(to_dir), None) => (to_dir, to_dir.flip()),
                (Some(to_dir), Some(from_dir)) => (to_dir, from_dir),
            };

            if to_dir == from_dir {
                panic!("a train cannot go from one direction in a tile to itself");
            }
            // the transform to place the train, within a single tile
            let local_transform;
            if to_dir == from_dir.flip() {
                local_transform = Transform::from_rotation(Quat::from(to_dir))
                    * Transform::from_xyz(0.0, (t - 0.5) * TILE_SIZE_PX, 0.0);
            } else {
                local_transform = get_local_transform_in_turn(from_dir, to_dir, time_within_tick);
            }

            let train_color = if time_within_tick < 0.5 {
                activity.activity.start_color
            } else {
                activity.activity.end_color
            };
            commands.entity(*entity).insert((
                base_transform * local_transform,
                Sprite {
                    image: asset_server.load("sprites/Train.png"),
                    color: train_color.into(),
                    ..default()
                },
            ));
        }
    }

    pub fn tick(&mut self) -> YardProcessTickResult {
        let mut start_tick_events = Vec::new();
        let mut mid_tick_events = Vec::new();
        let mut end_tick_events = Vec::new();

        let mut outgoing_border_states: [[TileBorderState; NUM_COLS as usize]; NUM_ROWS as usize] =
            Default::default();
        self.train_activity = Vec::new();

        for row in 0..(NUM_ROWS as usize) {
            for col in 0..(NUM_COLS as usize) {
                let incoming_border_state = &self.borders[row][col];
                let tile = &mut self.tiles[row][col];

                let tile_process_tick_result =
                    tile.process_and_output(incoming_border_state.clone());
                let train_tile_activity = tile_process_tick_result.trains;

                for e in tile_process_tick_result.start_tick_events {
                    start_tick_events.push(TileEventWithLocation { event: e, row, col });
                }
                for e in tile_process_tick_result.mid_tick_events {
                    mid_tick_events.push(TileEventWithLocation { event: e, row, col });
                }
                for e in tile_process_tick_result.end_tick_events {
                    end_tick_events.push(TileEventWithLocation { event: e, row, col });
                }

                let mut outgoing_border_state = TileBorderState::new();

                for dir_u8 in 0..4 {
                    let out_dir = Dir::from(dir_u8);
                    let mut colors_to_mix: Vec<TrainColor> = Vec::new();
                    for train_coming_thru in train_tile_activity.iter() {
                        if train_coming_thru.to_dir == Some(out_dir) {
                            colors_to_mix.push(train_coming_thru.end_color);
                        }
                    }
                    if !colors_to_mix.is_empty() {
                        let new_train_color = TrainColor::mix_many(&colors_to_mix);
                        outgoing_border_state.add_train(new_train_color, out_dir);
                        if colors_to_mix.len() > 1 {
                            end_tick_events.push(TileEventWithLocation {
                                event: TileEvent::MixColors(
                                    new_train_color,
                                    out_dir.to_local_coords_of_edge(),
                                ),
                                row,
                                col,
                            });
                        }
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
                    end_tick_events.push(TileEventWithLocation {
                        event: TileEvent::MixColors(new_color, Dir::Up.to_local_coords_of_edge()),
                        row,
                        col,
                    })
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
                    end_tick_events.push(TileEventWithLocation {
                        event: TileEvent::MixColors(
                            new_color,
                            Dir::Right.to_local_coords_of_edge(),
                        ),
                        row,
                        col,
                    })
                }
                outgoing_border_states[row][col].set_train(t2, Dir::Right);
                outgoing_border_states[row][col + 1].set_train(t1, Dir::Left);
            }
        }
        self.borders = outgoing_border_states;

        YardProcessTickResult {
            start_tick_events,
            mid_tick_events,
            end_tick_events,
        }
    }

    // check if all source tiles are empty, all destination tiles are empty, and if all borders are empty.
    pub fn has_won(&self) -> bool {
        if self.train_activity.len() > 0 {
            return false;
        }
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

    pub fn switch_active_passive(&mut self, row: usize, col: usize) {
        self.tiles[row][col].switch_active_passive();
    }
}

fn get_local_transform_in_turn(from_dir: Dir, to_dir: Dir, time_within_tick: f32) -> Transform {
    let turning_counter_clockwise = to_dir == from_dir.rotate_cw();

    let rotation_amount = if turning_counter_clockwise {
        FRAC_PI_2 * time_within_tick
    } else {
        -FRAC_PI_2 * time_within_tick
    };

    let mut initial_rotation = Transform::from_rotation(Quat::from(from_dir.flip()));
    if turning_counter_clockwise {
        // not sure why this if statement is needed, but it makes all the calculations appear to work.
        initial_rotation.rotate(Quat::from(Dir::Right));
    }

    let transform_to_point_of_rotation = if from_dir == Dir::Down && to_dir == Dir::Right
        || from_dir == Dir::Right && to_dir == Dir::Down
    {
        Transform::from_xyz(TILE_SIZE_PX / 2.0, -TILE_SIZE_PX / 2.0, 0.0)
    } else if from_dir == Dir::Down && to_dir == Dir::Left
        || from_dir == Dir::Left && to_dir == Dir::Down
    {
        Transform::from_xyz(-TILE_SIZE_PX / 2.0, -TILE_SIZE_PX / 2.0, 0.0)
    } else if from_dir == Dir::Up && to_dir == Dir::Left
        || from_dir == Dir::Left && to_dir == Dir::Up
    {
        Transform::from_xyz(-TILE_SIZE_PX / 2.0, TILE_SIZE_PX / 2.0, 0.0)
    } else {
        Transform::from_xyz(TILE_SIZE_PX / 2.0, TILE_SIZE_PX / 2.0, 0.0)
    };

    let transform_after_rotation = if turning_counter_clockwise {
        Transform::from_xyz(0.0, TILE_SIZE_PX / 2.0, 0.0)
            .with_rotation(Quat::from_rotation_z(FRAC_PI_2))
    } else {
        Transform::from_xyz(-TILE_SIZE_PX / 2.0, 0.0, 0.0)
    };

    transform_to_point_of_rotation
        * initial_rotation
        * Transform::from_rotation(Quat::from_rotation_z(rotation_amount))
        * transform_after_rotation
}
