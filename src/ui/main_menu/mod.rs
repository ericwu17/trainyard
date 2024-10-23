use bevy::prelude::*;

use super::{button::create_trainyard_button, UIState};

#[derive(Component)]
pub struct MainMenuUIRoot;
pub struct MainMenuUIPlugin;

impl Plugin for MainMenuUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UIState::MainMenu), spawn_main_menu);
    }
}

fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ui_root_query: Query<Entity, With<super::UIRootContainer>>,
) {
    let ui_root = ui_root_query.single();
    let font = asset_server.load("fonts/kenyan_coffee_rg.otf");

    // =============================================================================================
    // root container for the main menu
    // =============================================================================================
    let main_menu_root = (
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        },
        MainMenuUIRoot,
    );

    // =============================================================================================
    // text which says "TRAINYARD"
    // =============================================================================================
    let title_text_box = NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Px(120.0),
            ..default()
        },
        ..default()
    };

    let title_text = TextBundle::from_section(
        "TRAINYARD",
        TextStyle {
            font: font.clone(),
            font_size: 100.0,
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
    // "play" button
    // =============================================================================================
    let play_button = create_trainyard_button(
        &mut commands,
        "PLAY",
        200.0,
        90.0,
        50.0,
        super::BTN_BORDER_GREEN,
        font.clone(),
    );

    // =============================================================================================
    // "credits" button
    // =============================================================================================
    let credits_button = create_trainyard_button(
        &mut commands,
        "CREDITS",
        200.0,
        90.0,
        50.0,
        super::BTN_BORDER_BLUE,
        font.clone(),
    );

    // =============================================================================================
    // "credits" button
    // =============================================================================================

    let main_menu_root = commands.spawn(main_menu_root).id();
    let title_text_box = commands.spawn(title_text_box).id();
    let title_text = commands.spawn(title_text).id();

    commands.entity(ui_root).push_children(&[main_menu_root]);
    commands
        .entity(main_menu_root)
        .push_children(&[title_text_box, play_button, credits_button]);
    commands.entity(title_text_box).push_children(&[title_text]);
}
