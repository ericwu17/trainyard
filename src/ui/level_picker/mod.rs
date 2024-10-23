use bevy::prelude::*;

use crate::level_loader::StockLevelInfos;

use super::{button::create_trainyard_button, UIState};

#[derive(Component)]
pub struct LevelPickerUIRoot;

pub struct LevelPickerUIPlugin;
impl Plugin for LevelPickerUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UIState::LevelPicker), spawn_level_picker)
            .add_systems(OnExit(UIState::LevelPicker), teardown_level_picker);
    }
}
fn spawn_level_picker(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ui_root_query: Query<Entity, With<super::UIRootContainer>>,
    level_names: Res<StockLevelInfos>,
) {
    let ui_root = ui_root_query.single();
    let font: Handle<Font> = asset_server.load("fonts/kenyan_coffee_rg.otf");

    let level_names = level_names.0.iter().map(|level| &level.name);

    // =============================================================================================
    // root container for the level picker
    // =============================================================================================
    let level_picker_root = (
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::FlexStart,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        },
        LevelPickerUIRoot,
    );

    // =============================================================================================
    // text which says "TRAINYARD"
    // =============================================================================================
    let title_text_box = NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Px(120.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    };
    let title_text = TextBundle::from_section(
        "Abbotsford",
        TextStyle {
            font: font.clone(),
            font_size: 85.0,
            color: Color::srgb(1.0, 1.0, 1.0),
            ..default()
        },
    ) // Set the justification of the Text
    .with_text_justify(JustifyText::Center)
    .with_style(Style {
        position_type: PositionType::Absolute,
        width: Val::Percent(100.0),
        ..default()
    });

    // =============================================================================================
    // box that holds the rest of the GUI (all the buttons for selecting each individual level)
    // =============================================================================================

    let body_box = NodeBundle {
        style: Style {
            width: Val::Percent(85.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::FlexStart,
            flex_wrap: FlexWrap::Wrap,
            ..default()
        },
        ..default()
    };

    // =============================================================================================
    // construct one button for each level
    // =============================================================================================

    let mut buttons: Vec<Entity> = Vec::new();
    for name in level_names {
        let button = create_trainyard_button(
            &mut commands,
            &name,
            200.0,
            90.0,
            20.0,
            super::BTN_BORDER_GREEN,
            font.clone(),
        );
        buttons.push(button);
    }

    // putting it all together

    let level_picker_root = commands.spawn(level_picker_root).id();
    let title_text_box = commands.spawn(title_text_box).id();
    let title_text = commands.spawn(title_text).id();
    let body_box = commands.spawn(body_box).id();

    commands.entity(ui_root).push_children(&[level_picker_root]);
    commands
        .entity(level_picker_root)
        .push_children(&[title_text_box, body_box]);
    commands.entity(title_text_box).push_children(&[title_text]);

    commands.entity(body_box).push_children(&buttons);
}

fn teardown_level_picker(
    mut commands: Commands,
    level_picker_root_query: Query<Entity, With<LevelPickerUIRoot>>,
) {
    for entity in level_picker_root_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
