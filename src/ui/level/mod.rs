use bevy::prelude::*;

use super::{
    buttons::{create_trainyard_button, TrainyardButton},
    UIState,
};

#[derive(Component)]
pub struct LevelUIRoot;

#[derive(Component)]
pub struct YardPlaceholderNode;

pub struct LevelUIPlugin;
impl Plugin for LevelUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UIState::Level), spawn_level_ui)
            .add_systems(OnExit(UIState::Level), teardown_level_ui);
    }
}

fn spawn_level_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ui_root_query: Query<Entity, With<super::UIRootContainer>>,
) {
    let ui_root = ui_root_query.single();
    let font: Handle<Font> = asset_server.load("fonts/kenyan_coffee_rg.otf");

    // =============================================================================================
    // root container for the level UI
    // =============================================================================================
    let level_root = (
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        },
        LevelUIRoot,
    );

    let canvas_placeholder = NodeBundle {
        style: Style {
            width: Val::Px(672.0),
            height: Val::Px(672.0),
            ..default()
        },
        background_color: Color::srgba(1.0, 0.0, 0.0, 0.0).into(),
        ..default()
    };

    // =============================================================================================
    // container for action buttons on the right
    // =============================================================================================
    let button_container = NodeBundle {
        style: Style {
            width: Val::Auto,
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Px(20.0)),
            ..default()
        },
        ..default()
    };

    // =============================================================================================
    // buttons
    // =============================================================================================

    let button_width = 180.0;
    let button_height = 60.0;
    let button_text_size = 23.0;
    let button_border_color = super::BTN_BORDER_BLACK;

    let back_button = create_trainyard_button(
        &mut commands,
        "Back to levels",
        button_width,
        button_height,
        button_text_size,
        button_border_color,
        font.clone(),
        TrainyardButton::LevelBackButton,
    );

    let start_trains_button = create_trainyard_button(
        &mut commands,
        "Start trains (SPACE)",
        button_width,
        button_height,
        button_text_size,
        button_border_color,
        font.clone(),
        TrainyardButton::LevelStartTrainsButton,
    );

    let start_erase_button = create_trainyard_button(
        &mut commands,
        "Erase (Q)",
        button_width,
        button_height,
        button_text_size,
        button_border_color,
        font.clone(),
        TrainyardButton::LevelStartEraseButton,
    );

    // putting it all together

    let level_root = commands.spawn(level_root).id();
    let button_container = commands.spawn(button_container).id();
    let canvas_placeholder = commands
        .spawn((canvas_placeholder, YardPlaceholderNode))
        .id();

    commands.entity(ui_root).push_children(&[level_root]);
    commands
        .entity(level_root)
        .push_children(&[canvas_placeholder, button_container]);
    commands.entity(button_container).push_children(&[
        back_button,
        start_trains_button,
        start_erase_button,
    ]);
}

fn teardown_level_ui(mut commands: Commands, level_root_query: Query<Entity, With<LevelUIRoot>>) {
    for entity in level_root_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
