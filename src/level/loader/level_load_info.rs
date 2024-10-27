use crate::level::direction::Dir;
use crate::level::tiles::{construct_new_tile, TileConstructionInfo};
use crate::level::trains::TrainColor;
use crate::level::yard::Yard;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct LevelLoadInfo {
    pub name: String,
    pub sources: Vec<(Vec<TrainColor>, Dir, (u8, u8))>,
    pub sinks: Vec<(Vec<TrainColor>, Vec<Dir>, (u8, u8))>,
    #[serde(default)]
    // if this field is not present when deserializing, it should be set to the default value of an empty Vec
    pub rocks: Vec<(u8, u8)>,
}

impl LevelLoadInfo {
    pub fn to_yard(&self, commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
        let mut yard: Yard = Yard::new(commands, asset_server);

        for source in self.sources.clone() {
            let (trains, out_dir, (row, col)) = source;
            yard.replace_tile(
                row as usize,
                col as usize,
                construct_new_tile(
                    TileConstructionInfo::SourceTile {
                        out: out_dir.clone(),
                        trains,
                    },
                    row,
                    col,
                    commands,
                    asset_server,
                ),
                commands,
            );
        }
        for sink in self.sinks.clone() {
            let (trains, in_dirs, (row, col)) = sink;
            let mut ins = [false; 4];
            for in_dir in in_dirs {
                ins[u8::from(in_dir) as usize] = true;
            }

            yard.replace_tile(
                row as usize,
                col as usize,
                construct_new_tile(
                    TileConstructionInfo::SinkTile { ins, trains },
                    row,
                    col,
                    commands,
                    asset_server,
                ),
                commands,
            );
        }
        for rock in self.rocks.clone() {
            let (row, col) = rock;
            yard.replace_tile(
                row as usize,
                col as usize,
                construct_new_tile(TileConstructionInfo::Rock, row, col, commands, asset_server),
                commands,
            );
        }
        commands.entity(yard.base_entity).insert(yard).id()
    }
}
