pub mod connections;
pub mod rock_tile;
pub mod source_tile;

use crate::{direction::Dir, trains::TrainColor, NUM_COLS, NUM_ROWS, TILE_SIZE_PX};
use bevy::prelude::*;
use connections::TileConnections;
use rock_tile::RockTile;
use source_tile::{SourceTile, SourceTileInitialCapacity};

#[derive(Component)]
pub struct TilePosition {
    pub r: u8,
    pub c: u8,
}

#[derive(Resource, Default)]
pub struct TileGrid {
    pub tiles: Vec<Vec<Entity>>,
}

#[derive(Component)]
pub struct Tile;

#[derive(Component)]
pub struct NonDrawableTile;

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TileGrid>()
            .add_systems(Startup, (spawn_game_tiles, init_render_game_tiles).chain())
            .add_systems(Update, (init_render_game_tiles).chain());
    }
}

fn spawn_game_tiles(mut commands: Commands, mut grid: ResMut<TileGrid>) {
    for row in 0..NUM_ROWS {
        let mut row_vec = Vec::new();
        for col in 0..NUM_COLS {
            let entity = commands
                .spawn((
                    Tile,
                    TilePosition { r: row, c: col },
                    TileConnections::default(),
                ))
                .id();
            row_vec.push(entity);
        }
        grid.tiles.push(row_vec);
    }
    commands
        .get_entity(grid.tiles[3][3])
        .unwrap()
        .insert((NonDrawableTile, RockTile));
    commands.get_entity(grid.tiles[3][4]).unwrap().insert((
        NonDrawableTile,
        SourceTile {
            out: Dir::Down,
            trains: vec![TrainColor::Red],
        },
        SourceTileInitialCapacity(1),
    ));
}

fn init_render_game_tiles(
    mut commands: Commands,
    drawn_tiles_query: Query<
        (Entity, &TilePosition, &TileConnections),
        (
            With<Tile>,
            Without<NonDrawableTile>,
            Changed<TileConnections>,
        ),
    >,
    other_tiles_query: Query<
        (
            Entity,
            &TilePosition,
            Option<&RockTile>,
            Option<&SourceTile>,
        ),
        With<NonDrawableTile>,
    >,
    asset_server: Res<AssetServer>,
) {
    for (entity, position, connection) in drawn_tiles_query.iter() {
        let x = position.c as f32 * TILE_SIZE_PX + TILE_SIZE_PX / 2.0;
        let y = position.r as f32 * TILE_SIZE_PX + TILE_SIZE_PX / 2.0;

        let (conn_type, rotation_quat) = connection.type_and_rotation();

        commands.entity(entity).insert(SpriteBundle {
            transform: Transform::from_xyz(x, y, 0.0).with_rotation(rotation_quat),
            texture: asset_server.load(conn_type.get_asset_path()),
            ..default()
        });
    }
    for (entity, position, rock_tile, source_tile) in other_tiles_query.iter() {
        let x = position.c as f32 * TILE_SIZE_PX + TILE_SIZE_PX / 2.0;
        let y = position.r as f32 * TILE_SIZE_PX + TILE_SIZE_PX / 2.0;
        if rock_tile.is_some() {
            commands.entity(entity).insert(SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                texture: asset_server.load("sprites/Rock.png"),
                ..default()
            });
        }
        if let Some(source_tile) = source_tile {
            commands.entity(entity).insert(SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0)
                    .with_rotation(Quat::from(source_tile.out)),
                texture: asset_server.load("sprites/Trainsource_exit.png"),
                ..default()
            });
        }
    }
}
