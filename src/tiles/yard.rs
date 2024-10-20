use bevy::prelude::*;

use crate::tiles::tile::Tile;

use super::connections::TileBorderState;

#[derive(Component)]
pub struct Yard {
    pub tiles: Vec<Vec<Box<dyn Tile + Send + Sync>>>,
    pub borders: Vec<Vec<TileBorderState>>,
}

impl Yard {
    pub fn render(&mut self, commands: &mut Commands, asset_server: &Res<AssetServer>) {
        for row in &mut self.tiles {
            for tile in row {
                tile.render(commands, asset_server);
            }
        }
    }
}
