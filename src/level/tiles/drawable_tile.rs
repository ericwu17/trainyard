use bevy::prelude::*;

use crate::level::{
    direction::Dir,
    tiles::connections::{TileBorderState, TileConnections},
    tiles::tile::Tile,
    trains::TrainColor,
};

use super::tile::{TileEvent, TileProcessTickResult, TileTrainActivity};

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

    fn process_and_output(&mut self, incoming: TileBorderState) -> TileProcessTickResult {
        let active_conn = self.connections.get_active_conn();
        let passive_conn = self.connections.get_passive_conn();

        let mut init_trains_coming_thru: Vec<TileTrainActivity> = Vec::with_capacity(4);

        let mut start_tick_events = Vec::new();
        let mut end_tick_events = Vec::new();
        let mut mid_tick_mixed_colors: Vec<(TrainColor, (Dir, Dir))> = Vec::new();

        for dir_u8 in 0..4 {
            let incoming_dir = Dir::from(dir_u8);
            if let Some(color) = incoming.get_train(incoming_dir) {
                let outgoing_dir: Option<Dir> =
                    if let Some(d) = active_conn.get_other_dir(incoming_dir) {
                        Some(d)
                    } else if let Some(d) = passive_conn.get_other_dir(incoming_dir) {
                        Some(d)
                    } else {
                        start_tick_events.push(TileEvent::CrashedOnEdge(color, incoming_dir));
                        None
                    };
                if let Some(outgoing_dir) = outgoing_dir {
                    init_trains_coming_thru.push(TileTrainActivity {
                        from_dir: Some(incoming_dir),
                        to_dir: Some(outgoing_dir),
                        start_color: color,
                        end_color: color, // temporary placeholder, this might be changed if trains mix
                    });
                }
            }
        }

        let will_toggle_tracks = init_trains_coming_thru.len() % 2 == 1;

        let mut trains_after_internal_mixing: Vec<TileTrainActivity> = Vec::with_capacity(4);
        for train_coming_thru in init_trains_coming_thru.iter() {
            let mut colors_to_mix: Vec<TrainColor> = Vec::with_capacity(4);
            for other_train_coming_thru in init_trains_coming_thru.iter() {
                if paths_collide(
                    train_coming_thru.from_dir.unwrap(),
                    train_coming_thru.to_dir.unwrap(),
                    other_train_coming_thru.from_dir.unwrap(),
                    other_train_coming_thru.to_dir.unwrap(),
                ) {
                    colors_to_mix.push(other_train_coming_thru.start_color);
                }
            }
            let new_color = TrainColor::mix_many(&colors_to_mix);
            if colors_to_mix.len() > 1 {
                if !mid_tick_mixed_colors
                    .iter()
                    .map(|x| x.0)
                    .any(|x| x == new_color)
                {
                    mid_tick_mixed_colors.push((
                        new_color,
                        (
                            train_coming_thru.from_dir.unwrap(),
                            train_coming_thru.to_dir.unwrap(),
                        ),
                    ));
                }
            }
            trains_after_internal_mixing.push(TileTrainActivity {
                end_color: new_color,
                start_color: train_coming_thru.start_color,
                from_dir: train_coming_thru.from_dir,
                to_dir: train_coming_thru.to_dir,
            });
        }

        let (connection_type, _) = self.connections.type_and_rotation();

        if will_toggle_tracks && connection_type.should_toggle_active_and_passive_when_trains_pass()
        {
            end_tick_events.push(TileEvent::SwitchActivePassive);
        }

        TileProcessTickResult {
            trains: trains_after_internal_mixing,
            start_tick_events,
            mid_tick_events: mid_tick_mixed_colors
                .iter()
                .map(|x| TileEvent::MixColors(x.0, Dir::pair_to_local_coords(x.1 .0, x.1 .1)))
                .collect(),
            end_tick_events,
        }
    }

    fn render(&mut self, commands: &mut Commands, asset_server: &Res<AssetServer>) {
        let (conn_type, rotation_quat) = self.connections.type_and_rotation();

        let bundle = (
            DrawableTileSpriteComponent,
            Transform::from_rotation(rotation_quat),
            Sprite::from_image(asset_server.load(conn_type.get_asset_path())),
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
