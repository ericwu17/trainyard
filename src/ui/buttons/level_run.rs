use bevy::prelude::*;

use super::TrainyardButton;

pub fn on_enter_run(mut interaction_query: Query<(&mut Text, &TrainyardButton)>) {
    for (mut text, button_type) in interaction_query.iter_mut() {
        if *button_type == TrainyardButton::LevelStartTrainsButton {
            text.sections[0].value = String::from("Stop trains (SPACE)");
        }
    }
}

pub fn on_exit_run(mut interaction_query: Query<(&mut Text, &TrainyardButton)>) {
    for (mut text, button_type) in interaction_query.iter_mut() {
        if *button_type == TrainyardButton::LevelStartTrainsButton {
            text.sections[0].value = String::from("Start trains (SPACE)");
        }
    }
}
