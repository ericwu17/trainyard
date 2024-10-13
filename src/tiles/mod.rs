pub mod connections;
pub mod rock_tile;
pub mod source_tile;

use crate::{direction::Dir, trains::TrainColor, NUM_COLS, NUM_ROWS, TILE_SIZE_PX};
use bevy::prelude::*;
use connections::TileConnections;
use rock_tile::{render_rocks, RockTile};
use source_tile::{render_source_tiles, spawn_source_tile};

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
        let systems = (
            render_drawable_game_tiles,
            render_rocks,
            render_source_tiles,
        );
        app.init_resource::<TileGrid>()
            .add_systems(Startup, spawn_game_tiles)
            .add_systems(Update, systems);
    }
}

fn spawn_game_tiles(
    mut commands: Commands,
    mut grid: ResMut<TileGrid>,
    asset_server: Res<AssetServer>,
) {
    for row in 0..NUM_ROWS {
        let mut row_vec = Vec::new();
        for col in 0..NUM_COLS {
            let x = col as f32 * TILE_SIZE_PX + TILE_SIZE_PX / 2.0;
            let y = row as f32 * TILE_SIZE_PX + TILE_SIZE_PX / 2.0;
            let entity = commands
                .spawn((
                    Tile,
                    TilePosition { r: row, c: col },
                    TileConnections::default(),
                    SpriteBundle {
                        transform: Transform::from_xyz(x, y, 0.0),
                        ..default()
                    },
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
    spawn_source_tile(
        commands,
        grid.tiles[3][4],
        Dir::Right,
        vec![
            TrainColor::Green,
            TrainColor::Red,
            TrainColor::Green,
            TrainColor::Red,
        ],
        asset_server,
    );
}

fn render_drawable_game_tiles(
    mut commands: Commands,
    drawn_tiles_query: Query<
        (Entity, &TilePosition, &TileConnections),
        (
            With<Tile>,
            Without<NonDrawableTile>,
            Changed<TileConnections>,
        ),
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
}
