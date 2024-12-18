use bevy::prelude::*;

use crate::level::{direction::Dir, trains::TrainColor};

use super::connections::TileBorderState;

/// A struct to represent how trains are moving within a tile,
#[derive(Clone, Debug)]
pub struct TileTrainActivity {
    pub from_dir: Option<Dir>,
    pub to_dir: Option<Dir>,
    pub start_color: TrainColor,
    pub end_color: TrainColor,
}

#[derive(Clone, Debug, Default)]
pub struct TileProcessTickResult {
    pub trains: Vec<TileTrainActivity>,
    pub start_tick_events: Vec<TileEvent>,
    pub mid_tick_events: Vec<TileEvent>,
    pub end_tick_events: Vec<TileEvent>,
}

#[derive(Clone, Debug)]
pub enum TileEvent {
    MixColors(TrainColor, (f32, f32)),
    CrashedOnEdge(TrainColor, Dir),
    ShrinkAwayInnerEntity(Entity),
    SinkReceivedTrain(TrainColor),
    SwitchActivePassive,
}
pub trait Tile {
    fn add_connection(&mut self, _d1: Dir, _d2: Dir) {}

    fn erase_connections(&mut self) {}

    fn switch_active_passive(&mut self) {}

    // the function argument represents an __incoming__ border state,
    // while the output represents an __outgoing__ border state.
    fn process_and_output(&mut self, incoming: TileBorderState) -> TileProcessTickResult;

    fn render(&mut self, _commands: &mut Commands, _asset_server: &Res<AssetServer>);

    fn get_entity(&self) -> Entity;

    fn despawn_entities_recursive(&self, commands: &mut Commands);

    fn reset_inner_entities(&mut self, _commands: &mut Commands) {}

    fn box_clone(&self) -> Box<dyn Tile + Send + Sync>;

    fn has_no_remaining_trains(&self) -> bool {
        true
    }
    fn get_connection_data(&self) -> u8 {
        0
    }
}

impl Clone for Box<dyn Tile + Send + Sync> {
    fn clone(&self) -> Box<dyn Tile + Send + Sync> {
        self.box_clone()
    }
}
