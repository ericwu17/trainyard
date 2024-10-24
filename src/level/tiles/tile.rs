use bevy::prelude::*;

use crate::level::{direction::Dir, TrainCrashedEvent};

use super::connections::TileBorderState;

pub trait Tile {
    fn add_connection(&mut self, _d1: Dir, _d2: Dir) {}

    fn erase_connections(&mut self) {}

    fn switch_active_passive(&mut self) {}

    // the function argument represents an __incoming__ border state,
    // while the output represents an __outgoing__ border state.
    fn process_and_output(
        &mut self,
        incoming: TileBorderState,
        crashed_event: &mut EventWriter<TrainCrashedEvent>,
    ) -> TileBorderState;

    fn render(&mut self, _commands: &mut Commands, _asset_server: &Res<AssetServer>);

    fn get_entity(&self) -> Entity;

    fn despawn_entities_recursive(&self, commands: &mut Commands);

    fn reset_inner_entities(&mut self, _commands: &mut Commands) {}

    fn box_clone(&self) -> Box<dyn Tile + Send + Sync>;

    fn has_no_remaining_trains(&self) -> bool {
        true
    }
}

impl Clone for Box<dyn Tile + Send + Sync> {
    fn clone(&self) -> Box<dyn Tile + Send + Sync> {
        self.box_clone()
    }
}
