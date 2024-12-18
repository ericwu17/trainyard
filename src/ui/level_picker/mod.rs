use bevy::prelude::*;

use crate::level::{loader::StockLevelInfos, persistence::GameLevelProgress};

use super::{
    buttons::{create_trainyard_button, TrainyardButton},
    UIState,
};

#[derive(Component)]
pub struct LevelPickerUIRoot;

#[derive(Event)]
pub struct StartLevelEvent {
    pub level_name: String,
}

pub struct LevelPickerUIPlugin;
impl Plugin for LevelPickerUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StartLevelEvent>()
            .add_systems(OnEnter(UIState::LevelPicker), spawn_level_picker)
            .add_systems(OnExit(UIState::LevelPicker), teardown_level_picker);
    }
}
fn spawn_level_picker(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ui_root_query: Query<Entity, With<super::UIRootContainer>>,
    level_names: Res<StockLevelInfos>,
    progress: Res<GameLevelProgress>,
) {
    let ui_root = ui_root_query.single();
    let font: Handle<Font> = asset_server.load("fonts/kenyan_coffee_rg.otf");

    let level_names = level_names.0.iter().map(|level| &level.name);

    // =============================================================================================
    // root container for the level picker
    // =============================================================================================
    let level_picker_root = (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::FlexStart,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        },
        LevelPickerUIRoot,
    );

    // =============================================================================================
    // text which says "TRAINYARD"
    // =============================================================================================
    let title_text_box = Node {
        width: Val::Percent(100.0),
        height: Val::Px(120.0),
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,

        ..default()
    };
    let title_text = (
        Text::new("Abbotsford"),
        TextFont {
            font: font.clone(),
            font_size: 85.0,
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            ..default()
        },
    );

    // =============================================================================================
    // box that holds the rest of the GUI (all the buttons for selecting each individual level)
    // =============================================================================================

    let body_box = Node {
        width: Val::Percent(85.0),
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::FlexStart,
        flex_wrap: FlexWrap::Wrap,
        ..default()
    };

    // =============================================================================================
    // construct one button for each level
    // =============================================================================================

    let mut buttons: Vec<Entity> = Vec::new();
    for name in level_names {
        let mut has_won_this_level = false;

        if progress.0.contains_key(name) {
            has_won_this_level = progress.0.get(name).unwrap().has_won;
        }

        let border_color = if has_won_this_level {
            super::BTN_BORDER_GREEN
        } else {
            super::BTN_BORDER_BLACK
        };

        let button = create_trainyard_button(
            &mut commands,
            &name,
            200.0,
            90.0,
            20.0,
            border_color,
            font.clone(),
            TrainyardButton::LevelPickerStartLevel(name.clone()),
        );
        buttons.push(button);
    }

    // putting it all together

    let level_picker_root = commands.spawn(level_picker_root).id();
    let title_text_box = commands.spawn(title_text_box).id();
    let title_text = commands.spawn(title_text).id();
    let body_box = commands.spawn(body_box).id();

    commands.entity(ui_root).add_children(&[level_picker_root]);
    commands
        .entity(level_picker_root)
        .add_children(&[title_text_box, body_box]);
    commands.entity(title_text_box).add_children(&[title_text]);

    commands.entity(body_box).add_children(&buttons);
}

fn teardown_level_picker(
    mut commands: Commands,
    level_picker_root_query: Query<Entity, With<LevelPickerUIRoot>>,
) {
    for entity in level_picker_root_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
