use bevy::prelude::*;

use super::TrainyardButton;

pub fn on_enter_erase(mut interaction_query: Query<(&mut Text, &TrainyardButton)>) {
    for (mut text, button_type) in interaction_query.iter_mut() {
        if *button_type == TrainyardButton::LevelStartEraseButton {
            text.0 = String::from("Stop Erasing (Q)");
        }
    }
}

pub fn on_exit_erase(mut interaction_query: Query<(&mut Text, &TrainyardButton)>) {
    for (mut text, button_type) in interaction_query.iter_mut() {
        if *button_type == TrainyardButton::LevelStartEraseButton {
            text.0 = String::from("Erase (Q)");
        }
    }
}
