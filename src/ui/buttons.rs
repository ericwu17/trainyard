pub mod erase;
pub mod level_run;

use bevy::prelude::*;

use super::{level_picker::StartLevelEvent, UIState};
use crate::level::{
    cursor::CursorState, loader::StockLevelInfos, toggle_level_state, CurrentLevelName, LevelState,
};

#[derive(Component, Clone, PartialEq, Eq)]
pub enum TrainyardButton {
    // enum variant to represent a "generic button, ignored by the button handler"
    Unknown,
    MainMenuStartGame,
    MainMenuCredits,
    CreditsBack,
    LevelPickerStartLevel(String),
    LevelBackButton,
    LevelStartTrainsButton,
    LevelStartEraseButton,
    LevelWinDialogNextButton,
    LevelWinDialogBackButton,
}

pub struct ButtonPlugin;
impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (button_sounds_system, trainyard_ui_button_handler))
            .add_systems(OnEnter(CursorState::Erasing), erase::on_enter_erase)
            .add_systems(OnExit(CursorState::Erasing), erase::on_exit_erase)
            .add_systems(OnExit(LevelState::Editing), level_run::on_enter_run)
            .add_systems(OnEnter(LevelState::Editing), level_run::on_exit_run);
    }
}

// util function for creating a button and getting a handle to the entity
pub fn create_trainyard_button(
    commands: &mut Commands,
    text: &str,
    width_px: f32,
    height_px: f32,
    text_size: f32,
    border_color: Color,
    font: Handle<Font>,
    button_type: TrainyardButton,
) -> Entity {
    let button_bundle = (
        ButtonBundle {
            style: Style {
                width: Val::Px(width_px),
                height: Val::Px(height_px),
                border: UiRect::all(Val::Px(3.0)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            border_color: border_color.into(),
            border_radius: BorderRadius::all(Val::Px(12.0)),
            background_color: super::BTN_BG.into(),
            ..default()
        },
        button_type.clone(),
    );

    let text_component = TextBundle::from_section(
        text,
        TextStyle {
            font,
            font_size: text_size,
            color: Color::srgb(1.0, 1.0, 1.0),
            ..default()
        },
    )
    .with_text_justify(JustifyText::Center)
    .with_style(Style {
        position_type: PositionType::Absolute,
        width: Val::Percent(100.0),
        ..default()
    });
    let text_bundle = (text_component, button_type);

    let button_entity = commands.spawn(button_bundle).id();
    let text_entity = commands.spawn(text_bundle).id();
    commands.entity(button_entity).push_children(&[text_entity]);
    return button_entity;
}

pub fn button_sounds_system(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<TrainyardButton>)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            commands.spawn(AudioBundle {
                source: asset_server.load("audio/button_press.ogg"),
                ..default()
            });
        }
    }
}

pub fn trainyard_ui_button_handler(
    interaction_query: Query<(&Interaction, &TrainyardButton), Changed<Interaction>>,
    level_state: Res<State<LevelState>>,
    cursor_state: Res<State<CursorState>>,
    mut next_ui_state: ResMut<NextState<UIState>>,
    mut next_level_state: ResMut<NextState<LevelState>>,
    mut next_cursor_state: ResMut<NextState<CursorState>>,
    mut start_lvl_ev_writer: EventWriter<StartLevelEvent>,
    curr_lvl_name: Res<CurrentLevelName>,
    levels: Res<StockLevelInfos>,
) {
    for (interaction, button) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            match button {
                TrainyardButton::Unknown => {}
                TrainyardButton::MainMenuStartGame => {
                    next_ui_state.set(UIState::LevelPicker);
                }
                TrainyardButton::MainMenuCredits => {
                    next_ui_state.set(UIState::Credits);
                }
                TrainyardButton::CreditsBack => {
                    next_ui_state.set(UIState::MainMenu);
                }
                TrainyardButton::LevelPickerStartLevel(level_name) => {
                    start_lvl_ev_writer.send(StartLevelEvent {
                        level_name: level_name.clone(),
                    });
                    next_ui_state.set(UIState::Level);
                }
                TrainyardButton::LevelBackButton => {
                    if *level_state.get() != LevelState::Won {
                        next_ui_state.set(UIState::LevelPicker);
                        next_level_state.set(LevelState::None);
                    }
                }
                TrainyardButton::LevelStartTrainsButton => {
                    toggle_level_state(&level_state, &mut next_level_state);
                }
                TrainyardButton::LevelStartEraseButton => {
                    if *level_state.get() == LevelState::Editing {
                        next_cursor_state.set(cursor_state.get().toggle_erase())
                    }
                }
                TrainyardButton::LevelWinDialogNextButton => {
                    start_lvl_ev_writer.send(StartLevelEvent {
                        level_name: find_next_level(curr_lvl_name.0.as_deref().unwrap(), &levels),
                    });
                }
                TrainyardButton::LevelWinDialogBackButton => {
                    next_ui_state.set(UIState::LevelPicker);
                    next_level_state.set(LevelState::None);
                }
            }
        }
    }
}

fn find_next_level(curr_lvl_name: &str, levels: &Res<StockLevelInfos>) -> String {
    for (index, level) in levels.0.iter().enumerate() {
        if level.name == curr_lvl_name {
            return levels.0[index + 1].name.clone();
        }
    }
    panic!("could not find the next level.");
}
