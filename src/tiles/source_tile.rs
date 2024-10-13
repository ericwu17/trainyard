use bevy::prelude::*;

use crate::{direction::Dir, trains::TrainColor};

#[derive(Component)]
pub struct SourceTile {
    pub out: Dir,
    pub trains: Vec<TrainColor>,
}

#[derive(Component)]
pub struct SourceTileInitialCapacity(pub u8);
