use bevy::prelude::*;
use bevy::{input::common_conditions::input_pressed, window::PrimaryWindow};

use crate::{direction::Dir, TilePosition, NUM_COLS, NUM_ROWS, TILE_SIZE_PX};

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CursorMovedEvent>()
            .insert_state(CursorState::NotDrawing)
            .add_systems(Startup, spawn_cursor)
            .add_systems(
                Update,
                (
                    draw_cursor_position,
                    move_cursor,
                    toggle_cursor_drawing,
                    play_cursor_sounds,
                ),
            )
            .add_systems(
                Update,
                move_cursor_by_mouse.run_if(input_pressed(MouseButton::Left)),
            )
            .add_systems(OnEnter(CursorState::Drawing), change_cursor_to_drawing)
            .add_systems(
                OnEnter(CursorState::NotDrawing),
                change_cursor_to_not_drawing,
            );
    }
}

#[derive(Event)]
pub struct CursorMovedEvent {
    dir: Dir,
    old_r: u8,
    old_c: u8,
    new_r: u8,
    new_c: u8,
}

#[derive(Component)]
pub struct CursorComponent;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum CursorState {
    Drawing,
    #[default]
    NotDrawing,
}

impl CursorState {
    pub fn toggle(&self) -> Self {
        match self {
            CursorState::Drawing => CursorState::NotDrawing,
            CursorState::NotDrawing => CursorState::Drawing,
        }
    }
}

fn spawn_cursor(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TilePosition { r: 3, c: 3 },
        CursorComponent,
        SpriteBundle {
            texture: asset_server.load("sprites/Cursor1.png"),
            transform: Transform::from_xyz(TILE_SIZE_PX * 3.5, TILE_SIZE_PX * 3.5, 1.0),
            ..default()
        },
    ));
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

fn change_cursor_to_drawing(
    mut query: Query<&mut Handle<Image>, With<CursorComponent>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(mut img_handle) = query.get_single_mut() {
        *img_handle = asset_server.load("sprites/Cursor2.png");
    }
}

fn change_cursor_to_not_drawing(
    mut query: Query<&mut Handle<Image>, With<CursorComponent>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(mut img_handle) = query.get_single_mut() {
        *img_handle = asset_server.load("sprites/Cursor1.png");
    }
}

fn toggle_cursor_drawing(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    state: Res<State<CursorState>>,
    mut next_state: ResMut<NextState<CursorState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyE) {
        next_state.set(state.get().toggle())
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
                    new_r: *r,
                    new_c: *c - 1,
                });
                *c -= 1;
            }
        }
        if keyboard_input.just_pressed(KeyCode::KeyD) {
            if *c < NUM_COLS - 1 {
                moved_events.send(CursorMovedEvent {
                    dir: Dir::Left,
                    old_r: *r,
                    old_c: *c,
                    new_r: *r,
                    new_c: *c + 1,
                });
                *c += 1;
            }
        }
        if keyboard_input.just_pressed(KeyCode::KeyS) {
            if *r > 0 {
                moved_events.send(CursorMovedEvent {
                    dir: Dir::Left,
                    old_r: *r,
                    old_c: *c,
                    new_r: *r - 1,
                    new_c: *c,
                });
                *r -= 1;
            }
        }
        if keyboard_input.just_pressed(KeyCode::KeyW) {
            if *r < NUM_ROWS - 1 {
                moved_events.send(CursorMovedEvent {
                    dir: Dir::Left,
                    old_r: *r,
                    old_c: *c,
                    new_r: *r + 1,
                    new_c: *c,
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
    mut moved_events: EventWriter<CursorMovedEvent>,
) {
    let window = q_windows.single();

    let mut position = q_position.single_mut();

    if let Some(cursor_position) = window.cursor_position() {
        // cursor is currently inside the window
        if *state.get() == CursorState::NotDrawing {
            next_state.set(CursorState::Drawing);
        }

        let x = cursor_position.x;
        let y = window.height() - cursor_position.y;

        let mut c = (x / TILE_SIZE_PX) as i32;
        let mut r = (y / TILE_SIZE_PX) as i32;

        if c < 0 {
            c = 0;
        }
        if c >= NUM_COLS as i32 {
            c = NUM_COLS as i32 - 1;
        }
        if r < 0 {
            r = 0;
        }
        if r >= NUM_ROWS as i32 {
            r = NUM_ROWS as i32 - 1;
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
            moved_events.send(CursorMovedEvent {
                dir,
                old_r,
                old_c,
                new_r: r,
                new_c: c,
            });
        }

        position.r = r;
        position.c = c;
    }
}

fn play_cursor_sounds(
    mut moved_events: EventReader<CursorMovedEvent>,
    mut state_events: EventReader<StateTransitionEvent<CursorState>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if moved_events.read().count() > 0 {
        commands.spawn(AudioBundle {
            source: asset_server.load("audio/draw_track.ogg"),
            ..default()
        });
    }
    if state_events.read().count() > 0 {
        commands.spawn(AudioBundle {
            source: asset_server.load("audio/button_press.ogg"),
            ..default()
        });
    }
}
