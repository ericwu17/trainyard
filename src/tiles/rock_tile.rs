use bevy::prelude::*;

use crate::TILE_SIZE_PX;

use super::TilePosition;

#[derive(Component)]
pub struct RockTile;

pub fn render_rocks(
    mut commands: Commands,
    query: Query<(Entity, &TilePosition), With<RockTile>>,
    asset_server: Res<AssetServer>,
) {
    for (entity, position) in query.iter() {
        let x = position.c as f32 * TILE_SIZE_PX + TILE_SIZE_PX / 2.0;
        let y = position.r as f32 * TILE_SIZE_PX + TILE_SIZE_PX / 2.0;
        commands.entity(entity).insert(SpriteBundle {
            transform: Transform::from_xyz(x, y, 0.0),
            texture: asset_server.load("sprites/Rock.png"),
            ..default()
        });
    }
}
