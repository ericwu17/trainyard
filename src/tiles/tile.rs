use bevy::prelude::*;

use crate::direction::Dir;

use super::connections::TileBorderState;

pub trait Tile {
    fn add_connection(&mut self, _d1: Dir, _d2: Dir) {}

    fn erase_connections(&mut self) {}

    fn switch_active_passive(&mut self) {}

    // the function argument represents an __incoming__ border state,
    // while the output represents an __outgoing__ border state.
    fn process_and_output(&mut self, incoming: TileBorderState) -> TileBorderState;

    fn get_entity(&self) -> Entity;

    fn render(&mut self, _commands: &mut Commands, _asset_server: &Res<AssetServer>) {}
}
