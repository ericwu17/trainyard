use bevy::prelude::*;

use super::source_tile::INNER_SPRITE_SIZE;
use super::tile::{TileEvent, TileProcessTickResult, TileTrainActivity};
use super::{connections::TileBorderState, tile::Tile};
use crate::level::{direction::Dir, trains::TrainColor};

#[derive(Clone)]
pub struct SinkTile {
    pub in_dirs: [bool; 4],
    pub trains: Vec<TrainColor>,

    pub base_entity: Entity,
    pub background_entity: Entity,
    pub entry_spout_entities: Vec<Entity>,
    pub border_entity: Entity,
    pub inner_entities: Vec<Entity>, // these are the entities for the sprites for the small plus symbols inside the source tile
}

impl SinkTile {
    pub fn new(
        in_dirs: [bool; 4],
        trains: Vec<TrainColor>,
        base_entity: Entity,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
    ) -> Self {
        let mut background_entity = base_entity;
        let mut entry_spout_entities: Vec<Entity> = Vec::new();
        let mut border_entity = base_entity;

        commands
            .get_entity(base_entity)
            .unwrap()
            .with_children(|parent| {
                background_entity = parent
                    .spawn((
                        SpriteBundle {
                            texture: asset_server.load("sprites/Tracktile_blank.png"),
                            ..default()
                        },
                        Name::new("blank background"),
                    ))
                    .id();

                for dir_u8 in 0..4 {
                    if in_dirs[dir_u8 as usize] {
                        let entry_spout_entity = parent
                            .spawn((
                                SpriteBundle {
                                    transform: Transform::from_xyz(0.0, 0.0, 0.1)
                                        .with_rotation(Quat::from(Dir::from(dir_u8))),
                                    texture: asset_server.load("sprites/Trainsink_entry.png"),
                                    ..default()
                                },
                                Name::new("trainsink entryway sprite"),
                            ))
                            .id();
                        entry_spout_entities.push(entry_spout_entity);
                    }
                }

                border_entity = parent
                    .spawn((
                        SpriteBundle {
                            transform: Transform::from_xyz(0.0, 0.0, 1.0),
                            texture: asset_server.load("sprites/Source_sink_border.png"),
                            ..default()
                        },
                        Name::new("trainsink border sprite"),
                    ))
                    .id();
            });
        Self {
            in_dirs,
            trains,
            base_entity,
            background_entity,
            entry_spout_entities,
            border_entity,
            inner_entities: vec![],
        }
    }
}

impl Tile for SinkTile {
    fn process_and_output(&mut self, incoming: TileBorderState) -> TileProcessTickResult {
        let mut train_activity = Vec::new();
        let mut mid_tick_events = Vec::new();
        let mut start_tick_events = Vec::new();

        for dir in Dir::all_dirs() {
            if !self.in_dirs[u8::from(dir) as usize] {
                if let Some(color) = incoming.get_train(dir) {
                    start_tick_events.push(TileEvent::CrashedOnEdge(color, dir));
                    continue;
                }
            }

            if let Some(train) = incoming.get_train(dir) {
                let mut index = 0;
                let mut inner_entity_to_remove = None;
                while index < self.trains.len() {
                    if self.trains[index] == train {
                        self.trains.remove(index);
                        inner_entity_to_remove = Some(self.inner_entities.remove(index));
                        break;
                    }
                    index += 1;
                }

                if let Some(entity) = inner_entity_to_remove {
                    train_activity.push(TileTrainActivity {
                        from_dir: Some(dir),
                        to_dir: None,
                        start_color: train,
                        end_color: train,
                    });
                    mid_tick_events.push(TileEvent::ShrinkAwayInnerEntity(entity));
                    mid_tick_events.push(TileEvent::SinkReceivedTrain(train));
                } else {
                    start_tick_events.push(TileEvent::CrashedOnEdge(train, dir));
                }
            }
        }

        TileProcessTickResult {
            trains: train_activity,
            mid_tick_events,
            start_tick_events,
            ..default()
        }
    }

    fn render(&mut self, commands: &mut Commands, asset_server: &Res<AssetServer>) {
        if self.inner_entities.is_empty() {
            let cap = self.trains.len();
            for (index, color) in self.trains.iter().enumerate() {
                let num_cols = if cap <= 1 {
                    1
                } else if cap <= 4 {
                    2
                } else if cap <= 9 {
                    3
                } else {
                    4
                };
                let col_size = INNER_SPRITE_SIZE / num_cols as f32;
                let row_size = col_size;
                let curr_col = index % num_cols;
                let curr_row = index / num_cols;

                let xf = Transform::from_xyz(
                    -(INNER_SPRITE_SIZE / 2.0) + col_size / 2.0 + col_size * curr_col as f32,
                    (INNER_SPRITE_SIZE / 2.0) - row_size / 2.0 - row_size * curr_row as f32,
                    1.5,
                )
                .with_scale(Vec2::splat(1.0 / (num_cols as f32)).extend(0.0));

                let bundle = (
                    SpriteBundle {
                        transform: xf,
                        texture: asset_server.load("sprites/Circle.png"),
                        sprite: Sprite {
                            color: Color::from(*color),
                            ..default()
                        },
                        ..default()
                    },
                    Name::new("circle sprite"),
                );

                commands
                    .get_entity(self.base_entity)
                    .unwrap()
                    .with_children(|parent| {
                        let inner_entity = parent.spawn(bundle).id();
                        self.inner_entities.push(inner_entity);
                    });
            }
        }
    }

    fn get_entity(&self) -> Entity {
        self.base_entity
    }

    fn despawn_entities_recursive(&self, commands: &mut Commands) {
        commands.entity(self.base_entity).despawn_recursive();
    }

    fn box_clone(&self) -> Box<dyn Tile + Send + Sync> {
        Box::new(self.clone())
    }

    fn reset_inner_entities(&mut self, commands: &mut Commands) {
        for entity in &self.inner_entities {
            if let Some(entity_cmds) = commands.get_entity(*entity) {
                entity_cmds.despawn_recursive();
                commands
                    .entity(self.base_entity)
                    .remove_children(&[*entity]);
            }
        }
        self.inner_entities = Vec::new();
    }

    fn has_no_remaining_trains(&self) -> bool {
        self.trains.is_empty()
    }
}
