use bevy::prelude::*;

use crate::{direction::Dir, trains::TrainColor};

use super::{connections::TileBorderState, tile::Tile};

pub const INNER_SPRITE_SIZE: f32 = 52.0;

#[derive(Component)]
pub struct SourceTile {
    pub out_dir: Dir,
    pub trains: Vec<TrainColor>,

    pub base_entity: Entity,
    pub background_entity: Entity,
    pub exit_spout_entity: Entity,
    pub border_entity: Entity,
    pub inner_entities: Vec<Entity>, // these are the entities for the sprites for the small plus symbols inside the source tile
}

impl SourceTile {
    pub fn new(
        out_dir: Dir,
        trains: Vec<TrainColor>,
        base_entity: Entity,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
    ) -> Self {
        let mut background_entity = base_entity;
        let mut exit_spout_entity = base_entity;
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
                exit_spout_entity = parent
                    .spawn(SpriteBundle {
                        transform: Transform::from_rotation(Quat::from(out_dir)),
                        texture: asset_server.load("sprites/Trainsource_exit.png"),
                        ..default()
                    })
                    .id();
                border_entity = parent
                    .spawn(SpriteBundle {
                        transform: Transform::from_xyz(0.0, 0.0, 1.0),
                        texture: asset_server.load("sprites/Source_sink_border.png"),
                        ..default()
                    })
                    .id();
            });

        Self {
            out_dir,
            trains,
            base_entity,
            background_entity,
            exit_spout_entity,
            border_entity,
            inner_entities: vec![],
        }
    }
}

impl Tile for SourceTile {
    fn process_and_output(&mut self, incoming: TileBorderState) -> TileBorderState {
        for dir_u8 in 0..4 {
            if incoming.get_train(Dir::from(dir_u8)).is_some() {
                todo!("train crashed!");
            }
        }

        let mut output_state = TileBorderState::new();
        if !self.trains.is_empty() {
            output_state.add_train(self.trains.remove(0), self.out_dir);
        }
        return output_state;
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
                    1.0,
                )
                .with_scale(Vec2::splat(1.0 / (num_cols as f32)).extend(0.0));

                let bundle = SpriteBundle {
                    transform: xf,
                    texture: asset_server.load("sprites/Plus_sign.png"),
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
        } else if self.inner_entities.len() > self.trains.len() {
            while self.inner_entities.len() > self.trains.len() {
                let entity = self.inner_entities.remove(0);
                commands.entity(entity).despawn_recursive();
            }
        }
    }

    fn despawn_entities_recursive(&self, commands: &mut Commands) {
        commands.entity(self.base_entity).despawn_recursive();
    }

    fn get_entity(&self) -> Entity {
        self.base_entity
    }
}
