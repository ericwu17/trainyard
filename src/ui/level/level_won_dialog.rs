use bevy::prelude::*;

use crate::{
    level::LevelState,
    ui::buttons::{create_trainyard_button, TrainyardButton},
};

#[derive(Component)]
pub struct LevelWonDialogRoot;

// This spacer is used for the animation where the dialog box comes in from the bottom of the screen
#[derive(Component)]
pub struct DialogBoxSpacer(f32);

pub struct LevelWonDialogPlugin;
impl Plugin for LevelWonDialogPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(LevelState::Won), spawn_level_won_dialog)
            .add_systems(OnExit(LevelState::Won), despawn_level_won_dialog)
            .add_systems(
                Update,
                dialog_box_animation_system.run_if(in_state(LevelState::Won)),
            );
    }
}

fn spawn_level_won_dialog(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font: Handle<Font> = asset_server.load("fonts/kenyan_coffee_rg.otf");

    // =============================================================================================
    // root container for the level UI
    // =============================================================================================
    let dialog_box_root = (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            overflow: Overflow {
                x: OverflowAxis::Visible,
                y: OverflowAxis::Hidden,
            },
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.85)),
        LevelWonDialogRoot,
    );
    // =============================================================================================
    // spacer used for animation of the dialog
    // =============================================================================================
    let initial_spacer_displacement = 2500.0;
    let dialog_box_spacer = (
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(initial_spacer_displacement),
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
        DialogBoxSpacer(initial_spacer_displacement),
    );

    // =============================================================================================
    // box to contain the dialog
    // =============================================================================================
    let dialog_box = (
        Node {
            width: Val::Px(300.0),
            height: Val::Px(300.0),
            min_height: Val::Px(300.0),
            border: UiRect::all(Val::Px(3.0)),
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        },
        BorderColor(Color::WHITE),
        BorderRadius::all(Val::Px(24.0)),
        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
    );

    // =============================================================================================
    // text at the top of the dialog
    // =============================================================================================
    let title_text = (
        Text::new("Congratulations!"),
        TextFont {
            font: font.clone(),
            font_size: 35.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 1.0, 1.0)),
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            width: Val::Percent(100.0),
            margin: UiRect::all(Val::Px(30.0)),
            ..default()
        },
    );

    // =============================================================================================
    // buttons
    // =============================================================================================

    let button_width = 180.0;
    let button_height = 60.0;
    let button_text_size = 23.0;
    let button_border_color = Color::WHITE;

    let next_level_button = create_trainyard_button(
        &mut commands,
        "Next Level",
        button_width,
        button_height,
        button_text_size,
        button_border_color,
        font.clone(),
        TrainyardButton::LevelWinDialogNextButton,
    );
    let back_button = create_trainyard_button(
        &mut commands,
        "Back to levels",
        button_width,
        button_height,
        button_text_size,
        button_border_color,
        font.clone(),
        TrainyardButton::LevelWinDialogBackButton,
    );

    // putting it together:

    let dialog_box_root = commands.spawn(dialog_box_root).id();
    let dialog_box_spacer = commands.spawn(dialog_box_spacer).id();
    let dialog_box = commands.spawn(dialog_box).id();
    let title_text = commands.spawn(title_text).id();

    commands
        .entity(dialog_box_root)
        .add_children(&[dialog_box_spacer, dialog_box]);
    commands
        .entity(dialog_box)
        .add_children(&[title_text, next_level_button, back_button]);
}

fn despawn_level_won_dialog(
    mut commands: Commands,
    ui_root_query: Query<Entity, With<LevelWonDialogRoot>>,
) {
    for entity in ui_root_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn dialog_box_animation_system(
    mut commands: Commands,
    mut spacer_query: Query<(&mut Node, &mut DialogBoxSpacer, Entity)>,
    time: Res<Time>,
) {
    for (mut spacer_node, mut spacer, entity) in spacer_query.iter_mut() {
        let curr_size = spacer.0;
        let new_size = curr_size * f32::powf(0.001, time.delta_secs());

        if new_size < 1.0 {
            commands.entity(entity).remove::<DialogBoxSpacer>();
        } else {
            spacer.0 = new_size;
            spacer_node.height = Val::Px(new_size);
        }
    }
}
