pub mod level_load_info;
pub mod levels_json;

use bevy::prelude::*;
use level_load_info::LevelLoadInfo;
use serde::{Deserialize, Serialize};

#[derive(Resource, Serialize, Deserialize, Debug)]
pub struct StockLevelInfos(pub Vec<LevelLoadInfo>);

pub struct LevelLoaderPlugin;
impl Plugin for LevelLoaderPlugin {
    fn build(&self, app: &mut App) {
        let my_levels: StockLevelInfos = serde_json::from_str(levels_json::LEVEL_DATA).unwrap();
        app.insert_resource(my_levels);
    }
}
