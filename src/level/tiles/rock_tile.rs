use bevy::prelude::*;

use super::{
    connections::TileBorderState,
    tile::{Tile, TileEvent, TileProcessTickResult},
};
use crate::level::direction::Dir;

#[derive(Component)]
pub struct RockTileSpriteComponent;

#[derive(Clone)]
pub struct RockTile {
    entity: Entity,
    sprite_entity: Option<Entity>,
}

impl RockTile {
    pub fn new(entity: Entity) -> Self {
        Self {
            entity,
            sprite_entity: None,
        }
    }
}

impl Tile for RockTile {
    fn process_and_output(&mut self, incoming: TileBorderState) -> TileProcessTickResult {
        let mut start_tick_events = Vec::new();

        for dir_u8 in 0..4 {
            if let Some(color) = incoming.get_train(Dir::from(dir_u8)) {
                start_tick_events.push(TileEvent::CrashedOnEdge(color, dir_u8.into()));
            }
        }

        TileProcessTickResult {
            start_tick_events,
            ..default()
        }
    }

    fn render(&mut self, commands: &mut Commands, asset_server: &Res<AssetServer>) {
        let bundle = (
            RockTileSpriteComponent,
            Sprite::from_image(asset_server.load("sprites/Rock.png")),
            Name::new("Rock"),
        );

        match self.sprite_entity {
            Some(inner_entity) => {
                commands.get_entity(inner_entity).unwrap().insert(bundle);
            }
            None => {
                commands
                    .get_entity(self.entity)
                    .unwrap()
                    .with_children(|parent| {
                        let entity = parent.spawn(bundle).id();
                        self.sprite_entity = Some(entity);
                    });
            }
        };
    }

    fn despawn_entities_recursive(&self, commands: &mut Commands) {
        commands.entity(self.entity).despawn_recursive();
    }

    fn get_entity(&self) -> Entity {
        self.entity
    }

    fn box_clone(&self) -> Box<dyn Tile + Send + Sync> {
        Box::new(self.clone())
    }
}
