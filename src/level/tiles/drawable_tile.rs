use bevy::prelude::*;

use crate::level::{
    direction::Dir,
    tiles::connections::{TileBorderState, TileConnections},
    tiles::tile::Tile,
    trains::TrainColor,
    TrainCrashedEvent,
};

#[derive(Component)]
pub struct DrawableTileSpriteComponent;

#[derive(Clone)]
pub struct DrawableTile {
    connections: TileConnections,
    entity: Entity,
    sprite_entity: Option<Entity>,
}

impl DrawableTile {
    pub fn new(entity: Entity) -> Self {
        Self {
            connections: TileConnections::empty(),
            entity,
            sprite_entity: None,
        }
    }
}

impl Tile for DrawableTile {
    fn add_connection(&mut self, d1: Dir, d2: Dir) {
        self.connections = self.connections.add_connection(d1, d2);
    }

    fn erase_connections(&mut self) {
        self.connections = TileConnections::empty();
    }

    fn switch_active_passive(&mut self) {
        self.connections = self.connections.switch_active_passive();
    }

    fn process_and_output(
        &mut self,
        incoming: TileBorderState,
        crashed_event: &mut EventWriter<TrainCrashedEvent>,
    ) -> TileBorderState {
        let active_conn = self.connections.get_active_conn();
        let passive_conn = self.connections.get_passive_conn();

        struct TrainComingThrough {
            color: TrainColor,
            from: Dir,
            to: Dir,
        }

        let mut init_trains_coming_thru: Vec<TrainComingThrough> = Vec::with_capacity(4);

        for dir_u8 in 0..4 {
            let incoming_dir = Dir::from(dir_u8);
            if let Some(color) = incoming.get_train(incoming_dir) {
                let outgoing_dir: Option<Dir> =
                    if let Some(d) = active_conn.get_other_dir(incoming_dir) {
                        Some(d)
                    } else if let Some(d) = passive_conn.get_other_dir(incoming_dir) {
                        Some(d)
                    } else {
                        crashed_event.send_default();
                        None
                    };
                if let Some(outgoing_dir) = outgoing_dir {
                    init_trains_coming_thru.push(TrainComingThrough {
                        color,
                        from: incoming_dir,
                        to: outgoing_dir,
                    });
                }
            }
        }

        let will_toggle_tracks = init_trains_coming_thru.len() % 2 == 1;

        let mut trains_after_internal_mixing: Vec<TrainComingThrough> = Vec::with_capacity(4);
        for train_coming_thru in init_trains_coming_thru.iter() {
            let mut colors_to_mix: Vec<TrainColor> = Vec::with_capacity(4);
            for other_train_coming_thru in init_trains_coming_thru.iter() {
                if paths_collide(
                    train_coming_thru.from,
                    train_coming_thru.to,
                    other_train_coming_thru.from,
                    other_train_coming_thru.to,
                ) {
                    colors_to_mix.push(other_train_coming_thru.color);
                }
            }
            let new_color = TrainColor::mix_many(colors_to_mix);
            trains_after_internal_mixing.push(TrainComingThrough {
                color: new_color,
                from: train_coming_thru.from,
                to: train_coming_thru.to,
            });
        }

        let mut outgoing_border_state = TileBorderState::new();
        for dir_u8 in 0..4 {
            let out_dir = Dir::from(dir_u8);
            let mut colors_to_mix: Vec<TrainColor> = Vec::with_capacity(2);
            for train_coming_thru in trains_after_internal_mixing.iter() {
                if train_coming_thru.to == out_dir {
                    colors_to_mix.push(train_coming_thru.color);
                }
            }
            if !colors_to_mix.is_empty() {
                outgoing_border_state.add_train(TrainColor::mix_many(colors_to_mix), out_dir);
            }
        }

        let (connection_type, _) = self.connections.type_and_rotation();

        if will_toggle_tracks && connection_type.should_toggle_active_and_passive_when_trains_pass()
        {
            self.switch_active_passive();
        }

        outgoing_border_state
    }

    fn render(&mut self, commands: &mut Commands, asset_server: &Res<AssetServer>) {
        let (conn_type, rotation_quat) = self.connections.type_and_rotation();

        let bundle = (
            DrawableTileSpriteComponent,
            SpriteBundle {
                transform: Transform::from_rotation(rotation_quat),
                texture: asset_server.load(conn_type.get_asset_path()),
                ..default()
            },
            Name::new("drawable tile"),
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
    fn get_connection_data(&self) -> u8 {
        self.connections.get_data()
    }
}

fn paths_collide(d1: Dir, d2: Dir, d3: Dir, d4: Dir) -> bool {
    if d1.flip() == d2 && d3.flip() == d4 {
        // in the "H" pattern, the two connections intersect, despite having different destinations
        return true;
    }
    if (d1 == d3 && d2 == d4) || (d1 == d4 && d2 == d3) {
        // in this case, the two trains either coincide, or are "on the same track"
        return true;
    }
    return false;
}
