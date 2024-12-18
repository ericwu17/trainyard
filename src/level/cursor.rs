use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy::{input::common_conditions::input_pressed, window::PrimaryWindow};

use crate::level::yard::Yard;
use crate::level::{LevelEditingSet, LevelState};
use crate::{NUM_COLS, NUM_ROWS, TILE_SIZE_PX};

use super::direction::Dir;

pub const NEUTRAL_CURSOR_COLOR: Color = Color::WHITE;
pub const DRAWING_CURSOR_COLOR: Color = Color::srgb(0.0, 0.0, 1.0);
pub const ERASING_CURSOR_COLOR: Color = Color::srgb(0.93, 0.59, 0.51);

#[derive(Component)]
pub struct TilePosition {
    pub r: u8,
    pub c: u8,
}

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CursorMovedEvent>()
            .insert_state(CursorState::NotDrawing)
            .add_systems(OnEnter(LevelState::Editing), spawn_cursor)
            .add_systems(OnExit(LevelState::Editing), despawn_cursor)
            .add_systems(
                Update,
                (
                    (
                        draw_cursor_position,
                        move_cursor,
                        toggle_cursor_drawing,
                        play_cursor_sounds,
                    ),
                    move_cursor_by_mouse.run_if(input_pressed(MouseButton::Left)),
                    clear_cursor_old_dir.run_if(input_just_pressed(MouseButton::Left)),
                    add_connections_from_cursor_movement
                        .run_if(in_state(CursorState::Drawing).and(in_state(LevelState::Editing))),
                    destroy_connections_under_cursor
                        .run_if(in_state(CursorState::Erasing).and(in_state(LevelState::Editing))),
                )
                    .in_set(LevelEditingSet),
            )
            .add_systems(OnEnter(CursorState::Drawing), clear_cursor_old_dir)
            .add_systems(
                Update,
                update_cursor_color.run_if(on_event::<StateTransitionEvent<CursorState>>),
            );
    }
}

#[derive(Event)]
pub struct CursorMovedEvent {
    dir: Dir,
    old_r: u8,
    old_c: u8,
}

#[derive(Component)]
pub struct CursorComponent;

#[derive(Component)]
pub struct OldCursorMovementDir {
    dir: Option<Dir>,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum CursorState {
    Drawing,
    #[default]
    NotDrawing,
    Erasing,
}

impl CursorState {
    pub fn toggle_draw(&self) -> Self {
        match self {
            CursorState::Drawing => CursorState::NotDrawing,
            CursorState::NotDrawing => CursorState::Drawing,
            CursorState::Erasing => CursorState::NotDrawing,
        }
    }

    pub fn toggle_erase(&self) -> Self {
        match self {
            CursorState::Drawing => CursorState::Erasing,
            CursorState::NotDrawing => CursorState::Erasing,
            CursorState::Erasing => CursorState::NotDrawing,
        }
    }

    pub fn get_color(&self) -> Color {
        match self {
            CursorState::Drawing => DRAWING_CURSOR_COLOR,
            CursorState::NotDrawing => NEUTRAL_CURSOR_COLOR,
            CursorState::Erasing => ERASING_CURSOR_COLOR,
        }
    }
}

fn spawn_cursor(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    cursor_state: Res<State<CursorState>>,
    yard_entity_query: Query<Entity, With<Yard>>,
) {
    let yard_entity = yard_entity_query.single();

    let cursor_color = cursor_state.get().get_color();

    let cursor = commands
        .spawn((
            TilePosition { r: 3, c: 3 },
            CursorComponent,
            OldCursorMovementDir { dir: None },
            Transform::from_xyz(TILE_SIZE_PX * 3.5, TILE_SIZE_PX * 3.5, 1.0),
            Sprite {
                image: asset_server.load("sprites/Cursor.png"),
                color: Color::from(cursor_color),
                ..default()
            },
        ))
        .id();
    commands.entity(yard_entity).add_children(&[cursor]);
}

fn despawn_cursor(
    mut commands: Commands,
    cursor_query: Query<Entity, With<CursorComponent>>,
    mut next_cursor_state: ResMut<NextState<CursorState>>,
) {
    for entity in cursor_query.iter() {
        commands.entity(entity).remove_parent().despawn();
    }
    next_cursor_state.set(CursorState::NotDrawing);
}

fn draw_cursor_position(
    mut query: Query<
        (&mut Transform, &TilePosition),
        (With<CursorComponent>, Changed<TilePosition>),
    >,
) {
    if let Ok((mut transform, position)) = query.get_single_mut() {
        transform.translation.x = position.c as f32 * TILE_SIZE_PX + TILE_SIZE_PX / 2.0;
        transform.translation.y = position.r as f32 * TILE_SIZE_PX + TILE_SIZE_PX / 2.0;
    }
}

fn clear_cursor_old_dir(mut query: Query<&mut OldCursorMovementDir>) {
    if let Ok(mut old_movement_dir) = query.get_single_mut() {
        old_movement_dir.dir = None;
    }
}

fn update_cursor_color(
    mut query: Query<&mut Sprite, With<CursorComponent>>,
    new_state: Res<State<CursorState>>,
) {
    if let Ok(mut sprite) = query.get_single_mut() {
        sprite.color = new_state.get().get_color();
    }
}

fn toggle_cursor_drawing(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    state: Res<State<CursorState>>,
    mut next_state: ResMut<NextState<CursorState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyE) {
        next_state.set(state.get().toggle_draw())
    }
    if keyboard_input.just_pressed(KeyCode::KeyQ) {
        next_state.set(state.get().toggle_erase())
    }
}

fn move_cursor(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut cursor_query: Query<&mut TilePosition, With<CursorComponent>>,
    mut moved_events: EventWriter<CursorMovedEvent>,
) {
    if let Ok(tile_pos) = cursor_query.get_single_mut() {
        let tile_pos = tile_pos.into_inner();
        let c = &mut tile_pos.c;
        let r = &mut tile_pos.r;

        if keyboard_input.just_pressed(KeyCode::KeyA) {
            if *c > 0 {
                moved_events.send(CursorMovedEvent {
                    dir: Dir::Left,
                    old_r: *r,
                    old_c: *c,
                });
                *c -= 1;
            }
        }
        if keyboard_input.just_pressed(KeyCode::KeyD) {
            if *c < NUM_COLS - 1 {
                moved_events.send(CursorMovedEvent {
                    dir: Dir::Right,
                    old_r: *r,
                    old_c: *c,
                });
                *c += 1;
            }
        }
        if keyboard_input.just_pressed(KeyCode::KeyS) {
            if *r > 0 {
                moved_events.send(CursorMovedEvent {
                    dir: Dir::Down,
                    old_r: *r,
                    old_c: *c,
                });
                *r -= 1;
            }
        }
        if keyboard_input.just_pressed(KeyCode::KeyW) {
            if *r < NUM_ROWS - 1 {
                moved_events.send(CursorMovedEvent {
                    dir: Dir::Up,
                    old_r: *r,
                    old_c: *c,
                });
                *r += 1;
            }
        }
    }
}

fn move_cursor_by_mouse(
    state: Res<State<CursorState>>,
    mut next_state: ResMut<NextState<CursorState>>,
    mut q_position: Query<&mut TilePosition, With<CursorComponent>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut q_old_movement_dir: Query<&mut OldCursorMovementDir>,
    mut moved_events: EventWriter<CursorMovedEvent>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    yard_global_transform_query: Query<&GlobalTransform, With<Yard>>,
) {
    let window = q_windows.single();
    let (camera, camera_transform) = camera_query.single();
    let yard_transform = yard_global_transform_query.single();
    let yard_transform = yard_transform.translation().truncate();

    let mut position = q_position.single_mut();
    let mut old_movement_dir = q_old_movement_dir.single_mut();

    if let Some(cursor_position) = window.cursor_position() {
        // cursor is currently inside the window

        let world_coordinates_of_mouse = camera
            .viewport_to_world_2d(camera_transform, cursor_position)
            .unwrap();

        let yard_local_coords = world_coordinates_of_mouse - yard_transform;

        let x = yard_local_coords.x;
        let y = yard_local_coords.y;

        let c = (x / TILE_SIZE_PX) as i32;
        let r = (y / TILE_SIZE_PX) as i32;

        if c < 0 || c >= NUM_COLS as i32 || r < 0 || r >= NUM_ROWS as i32 {
            return;
        }

        let c = c as u8;
        let r = r as u8;

        let old_r = position.r;
        let old_c = position.c;

        let mut maybe_dir = None;

        if old_r == r {
            if old_c + 1 == c {
                maybe_dir = Some(Dir::Right);
            }
            if old_c == c + 1 {
                maybe_dir = Some(Dir::Left);
            }
        }
        if old_c == c {
            if old_r + 1 == r {
                maybe_dir = Some(Dir::Up);
            }
            if old_r == r + 1 {
                maybe_dir = Some(Dir::Down);
            }
        }
        if let Some(dir) = maybe_dir {
            moved_events.send(CursorMovedEvent { dir, old_r, old_c });
        } else if old_c != c || old_r != r {
            old_movement_dir.dir = None;
        }

        position.r = r;
        position.c = c;

        if *state.get() == CursorState::NotDrawing {
            next_state.set(CursorState::Drawing);
        }
    }
}

fn play_cursor_sounds(
    mut moved_events: EventReader<CursorMovedEvent>,
    mut state_events: EventReader<StateTransitionEvent<CursorState>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if moved_events.read().count() > 0 {
        commands.spawn(AudioPlayer::<AudioSource>(
            asset_server.load("audio/draw_track.ogg"),
        ));
    }
    if state_events.read().count() > 0 {
        commands.spawn(AudioPlayer::<AudioSource>(
            asset_server.load("audio/button_press.ogg"),
        ));
    }
}

fn add_connections_from_cursor_movement(
    mut moved_events: EventReader<CursorMovedEvent>,
    mut old_movement_dir_query: Query<&mut OldCursorMovementDir>,
    mut yard_query: Query<&mut Yard>,
) {
    if let Ok(old_movement) = old_movement_dir_query.get_single_mut() {
        let yard = yard_query.single_mut().into_inner();
        let old_movement = old_movement.into_inner();

        for e in moved_events.read() {
            let new_dir = e.dir;

            if let Some(old_dir) = old_movement.dir {
                let old_dir = old_dir.flip();

                let r = e.old_r;
                let c = e.old_c;

                yard.tiles
                    .get_mut(r as usize)
                    .unwrap()
                    .get_mut(c as usize)
                    .unwrap()
                    .add_connection(new_dir, old_dir);
            }

            old_movement.dir = Some(e.dir);
        }
    }
}

fn destroy_connections_under_cursor(
    cursor_query: Query<&TilePosition, With<CursorComponent>>,
    mut yard_query: Query<&mut Yard>,
) {
    let cursor = cursor_query.single();
    let yard = yard_query.single_mut().into_inner();

    let tile = yard
        .tiles
        .get_mut(cursor.r as usize)
        .unwrap()
        .get_mut(cursor.c as usize)
        .unwrap();

    tile.erase_connections();
}
