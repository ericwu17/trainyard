pub mod level_load_info;

use bevy::prelude::*;
use level_load_info::LevelLoadInfo;
use serde::{Deserialize, Serialize};

#[derive(Resource, Serialize, Deserialize, Debug)]
pub struct StockLevelInfos(pub Vec<LevelLoadInfo>);

pub const LEVEL_DATA: &str = include_str!("../../../assets/levels/levels.json");

pub struct LevelLoaderPlugin;
impl Plugin for LevelLoaderPlugin {
    fn build(&self, app: &mut App) {
        let my_levels: StockLevelInfos = serde_json::from_str(LEVEL_DATA).unwrap();
        app.insert_resource(my_levels);
    }
}
