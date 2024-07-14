use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatsRoot {
    pub stats: Statistics,

    #[serde(rename = "DataVersion")]
    pub data_version: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Statistics {
    /// A multitude of generic statistics related to a player's actions. Players' statistics increase automatically when they perform the action relevant to the statistic names.  
    #[serde(rename = "minecraft:custom")]
    pub custom: Option<HashMap<String, i32>>,

    /// Statistic related to the number of blocks a player mined.  
    #[serde(rename = "minecraft:mined")]
    pub mined: Option<HashMap<String, i32>>,

    /// Statistics related to the number of items a player ran their durability negative.  
    #[serde(rename = "minecraft:broken")]
    pub broken: Option<HashMap<String, i32>>,

    /// Statistics related to the number of items crafted, smelted, etc.  
    #[serde(rename = "minecraft:crafted")]
    pub crafted: Option<HashMap<String, i32>>,

    /// Statistics related to the number of block or item used.  
    #[serde(rename = "minecraft:used")]
    pub used: Option<HashMap<String, i32>>,

    /// Statistics related to the number of dropped items a player picked up.  
    #[serde(rename = "minecraft:picked_up")]
    pub picked_up: Option<HashMap<String, i32>>,

    /// Statistics related to the number of items that droped.  
    #[serde(rename = "minecraft:dropped")]
    pub dropped: Option<HashMap<String, i32>>,

    /// Statistics related to the number of entities a player killed.  
    #[serde(rename = "minecraft:killed")]
    pub killed: Option<HashMap<String, i32>>,

    /// Statistics related to the times of a player being killed by entities.  
    #[serde(rename = "minecraft:killed_by")]
    pub killed_by: Option<HashMap<String, i32>>,
}

#[cfg(test)]
#[test]
fn test() {
    let file = std::fs::File::open("test_data/player_stats.json").unwrap();

    let stats: StatsRoot = serde_json::from_reader(file).unwrap();
}
