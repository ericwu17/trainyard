use bevy::{prelude::*, utils::HashMap};

#[derive(Resource, Default)]
pub struct GameLevelProgress(pub HashMap<String, LevelProgress>);

#[derive(Default)]
pub struct LevelProgress {
    pub has_won: bool,
    pub drawn_tracks: Vec<u8>,
}

pub struct PersistencePlugin;
impl Plugin for PersistencePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameLevelProgress>();
    }
}
