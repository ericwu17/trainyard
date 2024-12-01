use crate::level::LevelState;
use bevy::prelude::*;

use super::LevelStatusText;

pub fn update_status_text(
    level_status: Res<State<LevelState>>,
    mut status_text_q: Query<(&mut Text, &mut TextColor), With<LevelStatusText>>,
) {
    let text_color: Color;
    let text_content: &str;
    match level_status.get() {
        LevelState::None | LevelState::Editing => {
            text_content = "";
            text_color = Color::WHITE;
        }
        LevelState::RunningNotCrashed | LevelState::Won => {
            text_content = "STATUS: GOOD";
            text_color = Color::srgb(0.0, 0.7, 0.0);
        }
        LevelState::RunningCrashed => {
            text_content = "STATUS: CRASHED";
            text_color = Color::srgb(0.7, 0.0, 0.0);
        }
    }

    for (mut text, mut color) in status_text_q.iter_mut() {
        color.0 = text_color;
        text.0 = String::from(text_content);
    }
}
