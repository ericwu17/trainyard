use bevy::prelude::*;

use super::{
    buttons::{create_trainyard_button, TrainyardButton},
    UIState,
};

#[derive(Component)]
pub struct CreditsUIRoot;

pub struct CreditsUIPlugin;
impl Plugin for CreditsUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UIState::Credits), spawn_credits)
            .add_systems(OnExit(UIState::Credits), teardown_credits);
    }
}

fn spawn_credits(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ui_root_query: Query<Entity, With<super::UIRootContainer>>,
) {
    let ui_root = ui_root_query.single();
    let font: Handle<Font> = asset_server.load("fonts/kenyan_coffee_rg.otf");

    // =============================================================================================
    // root container for the credits
    // =============================================================================================
    let credits_root = (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::FlexStart,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        },
        CreditsUIRoot,
    );

    // =============================================================================================
    // text which says "Credits"
    // =============================================================================================
    let title_text_box = Node {
        width: Val::Auto,
        height: Val::Auto,
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        padding: UiRect::all(Val::Px(5.0)),
        ..default()
    };
    let title_text = (
        Text::new("Credits"),
        TextFont {
            font: font.clone(),
            font_size: 85.0,
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            width: Val::Percent(100.0),
            ..default()
        },
    );

    // =============================================================================================
    // text with the body of the credits
    // =============================================================================================

    let body_text_box = Node {
        width: Val::Percent(85.0),
        height: Val::Auto,
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        margin: UiRect::all(Val::Px(20.0)),
        ..default()
    };
    let body_text = (
        Text::new("Trainyard was originally created by Matt Rix. The game concept and game art are his (until I stole it lmao).\n\n"),
        TextFont {
            font: font.clone(),
            font_size: 25.0,
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(JustifyText::Center),
        Node{
            width: Val::Percent(100.0),
            ..default()
        },
    );

    // =============================================================================================
    // Button to return to main menu
    // =============================================================================================
    let back_button = create_trainyard_button(
        &mut commands,
        "BACK",
        200.0,
        90.0,
        50.0,
        super::BTN_BORDER_GREEN,
        font.clone(),
        TrainyardButton::CreditsBack,
    );

    let credits_root = commands.spawn(credits_root).id();
    let title_text_box = commands.spawn(title_text_box).id();
    let title_text = commands.spawn(title_text).id();
    let body_text_box = commands.spawn(body_text_box).id();
    let body_text = commands.spawn(body_text).id();

    commands.entity(ui_root).add_children(&[credits_root]);
    commands
        .entity(credits_root)
        .add_children(&[title_text_box, body_text_box, back_button]);
    commands.entity(title_text_box).add_children(&[title_text]);
    commands.entity(body_text_box).add_children(&[body_text]);
}

fn teardown_credits(
    mut commands: Commands,
    credits_root_query: Query<Entity, With<CreditsUIRoot>>,
) {
    for entity in credits_root_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
