use bevy::prelude::*;

use super::{
    buttons::{create_trainyard_button, TrainyardButton},
    UIState,
};

#[derive(Component)]
pub struct MainMenuUIRoot;
pub struct MainMenuUIPlugin;

impl Plugin for MainMenuUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UIState::MainMenu), spawn_main_menu)
            .add_systems(OnExit(UIState::MainMenu), teardown_main_menu);
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
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        },
        MainMenuUIRoot,
    );

    // =============================================================================================
    // text which says "TRAINYARD"
    // =============================================================================================
    let title_text_box = Node {
        width: Val::Percent(100.0),
        height: Val::Px(120.0),
        ..default()
    };

    let title_text = (
        Text::new("TRAINYARD"),
        TextFont {
            font: font.clone(),
            font_size: 100.0,
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
        TrainyardButton::MainMenuStartGame,
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
        TrainyardButton::MainMenuCredits,
    );
    // =============================================================================================
    // put it together
    // =============================================================================================

    let main_menu_root = commands.spawn(main_menu_root).id();
    let title_text_box = commands.spawn(title_text_box).id();
    let title_text = commands.spawn(title_text).id();

    commands.entity(ui_root).add_children(&[main_menu_root]);
    commands
        .entity(main_menu_root)
        .add_children(&[title_text_box, play_button, credits_button]);
    commands.entity(title_text_box).add_children(&[title_text]);
}

fn teardown_main_menu(
    mut commands: Commands,
    main_menu_root_query: Query<Entity, With<MainMenuUIRoot>>,
) {
    for entity in main_menu_root_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
