use bevy::prelude::*;

use super::{connections::TileBorderState, tile::Tile};
use crate::level::{direction::Dir, TrainCrashedEvent};

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
    fn process_and_output(
        &mut self,
        incoming: TileBorderState,
        crashed_event: &mut EventWriter<TrainCrashedEvent>,
    ) -> TileBorderState {
        for dir_u8 in 0..4 {
            if incoming.get_train(Dir::from(dir_u8)).is_some() {
                crashed_event.send_default();
            }
        }
        TileBorderState::new()
    }

    fn render(&mut self, commands: &mut Commands, asset_server: &Res<AssetServer>) {
        let bundle = (
            RockTileSpriteComponent,
            SpriteBundle {
                texture: asset_server.load("sprites/Rock.png"),
                ..default()
            },
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
