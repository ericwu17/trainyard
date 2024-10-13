use bevy::prelude::*;

use crate::{direction::Dir, trains::TrainColor};

use super::NonDrawableTile;

const INNER_SPRITE_SIZE: f32 = 52.0;

#[derive(Component)]
pub struct SourceTile {
    pub out: Dir,
    pub trains: Vec<TrainColor>,
}

#[derive(Component)]
pub struct SourceTileInitialCapacity(pub u8);

#[derive(Component)]
pub struct SourceTileInnerSign {
    pub index: u8,
    pub color: TrainColor,
}

#[derive(Component)]
pub struct SourceTileBorderSprite;

#[derive(Component)]
pub struct SourceTileBackgroundSprite;

pub fn spawn_source_tile(
    mut commands: Commands,
    entity: Entity,
    out_dir: Dir,
    trains: Vec<TrainColor>,
    asset_server: Res<AssetServer>,
) {
    let num_trains = trains.len();

    commands
        .get_entity(entity)
        .unwrap()
        .insert((
            NonDrawableTile,
            SourceTileInitialCapacity(num_trains as u8),
            SourceTile {
                out: out_dir,
                trains: trains.clone(),
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                SourceTileBackgroundSprite,
                SpriteBundle {
                    transform: Transform::from_rotation(Quat::from(out_dir)),
                    texture: asset_server.load("sprites/Trainsource_exit.png"),
                    ..default()
                },
            ));

            parent.spawn((
                SourceTileBorderSprite,
                SpriteBundle {
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    texture: asset_server.load("sprites/Source_sink_border.png"),
                    ..default()
                },
            ));

            for (index, color) in trains.iter().enumerate() {
                parent.spawn(SourceTileInnerSign {
                    index: index as u8,
                    color: *color,
                });
            }
        });
}

pub fn render_source_tiles(
    mut commands: Commands,
    query: Query<(&SourceTileInitialCapacity, &Children)>,
    inner_sprites_query: Query<(Entity, &SourceTileInnerSign)>,
    asset_server: Res<AssetServer>,
) {
    for (init_cap, children) in query.iter() {
        let init_cap = init_cap.0;

        for &child in children.iter() {
            // Draw inner sprites (little plus signs)
            if let Ok((inner_entity, SourceTileInnerSign { index, color })) =
                inner_sprites_query.get(child)
            {
                let num_cols = if init_cap <= 1 {
                    1
                } else if init_cap <= 4 {
                    2
                } else if init_cap <= 9 {
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
                    0.0,
                )
                .with_scale(Vec2::splat(1.0 / (num_cols as f32)).extend(0.0));

                commands.entity(inner_entity).insert(SpriteBundle {
                    transform: xf,
                    texture: asset_server.load("sprites/Plus_sign.png"),
                    sprite: Sprite {
                        color: Color::from(*color),
                        ..default()
                    },
                    ..default()
                });
            }
        }
    }
}
