use bevy::prelude::*;

use super::source_tile::INNER_SPRITE_SIZE;
use super::{connections::TileBorderState, tile::Tile};
use crate::{direction::Dir, trains::TrainColor};

#[derive(Clone)]
pub struct SinkTile {
    pub in_dirs: [bool; 4],
    pub trains: Vec<TrainColor>,

    pub base_entity: Entity,
    pub background_entity: Entity,
    pub entry_spout_entities: Vec<Entity>,
    pub border_entity: Entity,
    pub inner_entities: Vec<Entity>, // these are the entities for the sprites for the small plus symbols inside the source tile
    pub removed_entities: Vec<Entity>, // these are entities that have been removed by `process_and_output`, but still need to be despawned in the `render` function.
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
                    .spawn(SpriteBundle {
                        texture: asset_server.load("sprites/Tracktile_blank.png"),
                        ..default()
                    })
                    .id();

                for dir_u8 in 0..4 {
                    if in_dirs[dir_u8 as usize] {
                        let entry_spout_entity = parent
                            .spawn(SpriteBundle {
                                transform: Transform::from_rotation(Quat::from(Dir::from(dir_u8))),
                                texture: asset_server.load("sprites/Trainsink_entry.png"),
                                ..default()
                            })
                            .id();
                        entry_spout_entities.push(entry_spout_entity);
                    }
                }

                border_entity = parent
                    .spawn(SpriteBundle {
                        transform: Transform::from_xyz(0.0, 0.0, 1.0),
                        texture: asset_server.load("sprites/Source_sink_border.png"),
                        ..default()
                    })
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
            removed_entities: vec![],
        }
    }
}

impl Tile for SinkTile {
    fn process_and_output(&mut self, incoming: TileBorderState) -> TileBorderState {
        for dir in Dir::all_dirs() {
            if !self.in_dirs[u8::from(dir) as usize] && incoming.get_train(dir).is_some() {
                todo!("train crashed!");
            }

            if let Some(train) = incoming.get_train(dir) {
                let mut index = 0;
                let mut successfully_received_train = false;
                while index < self.trains.len() {
                    if self.trains[index] == train {
                        self.trains.remove(index);
                        self.removed_entities
                            .push(self.inner_entities.remove(index));
                        successfully_received_train = true;
                        break;
                    }
                    index += 1;
                }
                if !successfully_received_train {
                    todo!("train crashed!");
                }
            }
        }
        TileBorderState::new()
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

                let bundle = SpriteBundle {
                    transform: xf,
                    texture: asset_server.load("sprites/Circle.png"),
                    sprite: Sprite {
                        color: Color::from(*color),
                        ..default()
                    },
                    ..default()
                };

                commands
                    .get_entity(self.base_entity)
                    .unwrap()
                    .with_children(|parent| {
                        let inner_entity = parent.spawn(bundle).id();
                        self.inner_entities.push(inner_entity);
                    });
            }
        }

        while !self.removed_entities.is_empty() {
            let entity = self.removed_entities.pop().unwrap();
            commands.entity(self.base_entity).remove_children(&[entity]);
            commands.entity(entity).despawn_recursive();
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
            }
        }
        self.inner_entities = Vec::new();
    }
}
