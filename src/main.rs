pub mod connections;
pub mod cursor;
pub mod direction;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use connections::TileConnections;
use cursor::CursorPlugin;

const NUM_ROWS: u8 = 7;
const NUM_COLS: u8 = 7;
const TILE_SIZE_PX: f32 = 96.0;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CursorPlugin))
        .init_resource::<TileGrid>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, (spawn_game_tiles, init_render_game_tiles).chain())
        .add_systems(Update, (init_render_game_tiles).chain())
        .run();
}

#[derive(Component)]
struct Tile;

#[derive(Component)]
struct TilePosition {
    r: u8,
    c: u8,
}

#[derive(Resource, Default)]
struct TileGrid {
    tiles: Vec<Vec<Entity>>,
}

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
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
}

fn init_render_game_tiles(
    mut commands: Commands,
    tile_query: Query<(Entity, &TilePosition, &TileConnections), With<Tile>>,
    asset_server: Res<AssetServer>,
) {
    for (entity, position, connection) in tile_query.iter() {
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
