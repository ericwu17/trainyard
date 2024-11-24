pub mod level_won_dialog;
pub mod speed_slider;
pub mod status_text;

use bevy::prelude::*;
use speed_slider::{spawn_speed_slider, TrainSpeed};
use status_text::update_status_text;

use crate::level::LevelState;

use super::{
    buttons::{create_trainyard_button, TrainyardButton},
    UIState,
};

#[derive(Component)]
pub struct LevelUIRoot;

#[derive(Component)]
pub struct YardPlaceholderNode;

#[derive(Component)]
pub struct LevelStatusText;

pub struct LevelUIPlugin;
impl Plugin for LevelUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            level_won_dialog::LevelWonDialogPlugin,
            speed_slider::SpeedSliderPlugin,
        ))
        .add_systems(OnEnter(UIState::Level), spawn_level_ui)
        .add_systems(OnExit(UIState::Level), teardown_level_ui)
        .add_systems(
            Update,
            update_status_text.run_if(on_event::<StateTransitionEvent<LevelState>>()),
        );
    }
}

pub const BUTTON_WIDTH: f32 = 180.0;
pub const BUTTON_HEIGHT: f32 = 60.0;
pub const BUTTON_TEXT_SIZE: f32 = 23.0;
pub const BUTTON_BORDER_COLOR: Color = super::BTN_BORDER_BLACK;

fn spawn_level_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ui_root_query: Query<Entity, With<super::UIRootContainer>>,
    train_speed: Res<TrainSpeed>,
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

    // =============================================================================================
    // canvas placeholder: a 672x672 rectangle where the trainyard yard will go
    // =============================================================================================
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

    let back_button = create_trainyard_button(
        &mut commands,
        "Back to levels",
        BUTTON_WIDTH,
        BUTTON_HEIGHT,
        BUTTON_TEXT_SIZE,
        BUTTON_BORDER_COLOR,
        font.clone(),
        TrainyardButton::LevelBackButton,
    );

    let start_trains_button = create_trainyard_button(
        &mut commands,
        "Start trains (SPACE)",
        BUTTON_WIDTH,
        BUTTON_HEIGHT,
        BUTTON_TEXT_SIZE,
        BUTTON_BORDER_COLOR,
        font.clone(),
        TrainyardButton::LevelStartTrainsButton,
    );

    let start_erase_button = create_trainyard_button(
        &mut commands,
        "Erase (Q)",
        BUTTON_WIDTH,
        BUTTON_HEIGHT,
        BUTTON_TEXT_SIZE,
        BUTTON_BORDER_COLOR,
        font.clone(),
        TrainyardButton::LevelStartEraseButton,
    );

    // =============================================================================================
    // Status indicator (only visible when the level is running)
    // =============================================================================================
    let status_text_box = NodeBundle {
        style: Style {
            width: Val::Px(BUTTON_WIDTH),
            height: Val::Px(BUTTON_HEIGHT),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    };
    let status_text = TextBundle::from_section(
        "",
        TextStyle {
            font: font.clone(),
            font_size: 23.0,
            color: Color::srgb(0.0, 1.0, 0.0),
            ..default()
        },
    ) // Set the justification of the Text
    .with_text_justify(JustifyText::Center)
    .with_style(Style {
        position_type: PositionType::Absolute,
        width: Val::Percent(100.0),
        ..default()
    });

    // putting it all together

    let level_root = commands.spawn(level_root).id();
    let button_container = commands.spawn(button_container).id();
    let canvas_placeholder = commands
        .spawn((canvas_placeholder, YardPlaceholderNode))
        .id();
    let status_text_box = commands.spawn(status_text_box).id();
    let slider = spawn_speed_slider(&mut commands, font, &train_speed);
    let status_text = commands.spawn((status_text, LevelStatusText)).id();

    commands.entity(ui_root).push_children(&[level_root]);
    commands
        .entity(level_root)
        .push_children(&[canvas_placeholder, button_container]);
    commands.entity(button_container).push_children(&[
        back_button,
        start_trains_button,
        start_erase_button,
        slider,
        status_text_box,
    ]);
    commands
        .entity(status_text_box)
        .push_children(&[status_text]);
}

fn teardown_level_ui(mut commands: Commands, level_root_query: Query<Entity, With<LevelUIRoot>>) {
    for entity in level_root_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
