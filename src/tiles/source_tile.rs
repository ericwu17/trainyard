use bevy::prelude::*;

use crate::{direction::Dir, trains::TrainColor, TILE_SIZE_PX};

use super::TilePosition;

#[derive(Component)]
pub struct SourceTile {
    pub out: Dir,
    pub trains: Vec<TrainColor>,
}

#[derive(Component)]
pub struct SourceTileInitialCapacity(pub u8);

pub fn render_source_tiles(
    mut commands: Commands,
    query: Query<(Entity, &TilePosition, &SourceTile)>,
    asset_server: Res<AssetServer>,
) {
    for (entity, position, source_tile) in query.iter() {
        let x = position.c as f32 * TILE_SIZE_PX + TILE_SIZE_PX / 2.0;
        let y = position.r as f32 * TILE_SIZE_PX + TILE_SIZE_PX / 2.0;
        commands.entity(entity).insert(SpriteBundle {
            transform: Transform::from_xyz(x, y, 0.0).with_rotation(Quat::from(source_tile.out)),
            texture: asset_server.load("sprites/Trainsource_exit.png"),
            ..default()
        });
    }
}
