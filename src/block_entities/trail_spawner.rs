use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::mob_spawner::{PotentialSpawn, SpawnData};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct TrailSpawner<'a> {
    /// Between 1 and 128. Defaults to 14. — Maximum distance in blocks for players to join the battle.
    pub required_player_range: i32,

    /// Defaults to 36000. — Time in ticks of the cooldown period. Includes the time spend dispensing the reward.
    pub target_cooldown_length: i32,

    /// Optional, see configuration for defaults. — The configuration to use when not ominous.
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normal_config: Option<TrailSpawnerConfig<'a>>,

    /// Optional, defaults to  normal_config. When individual entries are omitted, they also default to their setting in  normal_config. — The configuration to use when ominous.
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ominous_config: Option<TrailSpawnerConfig<'a>>,

    /// A set of player UUIDs. — All the players that have joined the battle. The length of this array determines the amount of mobs and amount of reward.
    pub registered_players: Vec<i128>,

    /// A set of mob UUIDs. — The mobs that were spawned by this spawner and are still alive.
    pub current_mobs: Vec<i128>,

    /// Gametime in ticks when the cooldown ends. 0 if not currently in cooldown.
    pub cooldown_ends_at: i64,

    /// Gametime in ticks when the next spawn attempt happens. 0 if not currently active.
    pub next_mob_spawns_at: i64,

    /// The total amount of mobs that have already been spawned in this cycle. 0 if not currently active.
    pub total_mobs_spawned: i32,

    /// The next mob to attempt to spawn. Selected from  spawn_potentials after the last attempt. Determines the mob displayed in the spawner.
    #[serde(borrow)]
    pub spawn_data: SpawnData<'a>,

    /// A resource location to the loot table that is given as reward. Unset if not currently giving rewards. Selected from  loot_tables_to_eject after all mobs are defeated.
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ejecting_loot_table: Option<Cow<'a, str>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct TrailSpawnerConfig<'a> {
    /// Between 1 and 128. Defaults to 4 — Maximum distance in blocks that mobs can spawn.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spawn_range: Option<i32>,

    /// Defaults to 6 — Total amount of mobs spawned before cooldown for a single player.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_mobs: Option<f32>,

    /// Defaults to 2 — The amount of spawned mobs from this spawner that are allowed to exist simultaneously.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simultaneous_mobs: Option<i32>,

    /// Defaults to 2 — Amount of total mobs added for each additional player.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_mobs_added_per_player: Option<f32>,

    /// Defaults to 1 — Amount of simultaneous mobs added for each additional player.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simultaneous_mobs_added_per_player: Option<f32>,

    /// Defaults to 40 — Time in ticks between spawn attempts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticks_between_spawn: Option<i32>,

    ///  Defaults to an empty list — List of possible entities to spawn.
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spawn_potentials: Option<Vec<PotentialSpawn<'a>>>,

    ///  Defaults to a list of `minecraft:spawners/trial_chamber/consumables` and `minecraft:spawners/trial_chamber/key`, both with weight 1 — List of possible loot tables to give as reward.
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loot_tables_to_eject: Option<Vec<LootTable<'a>>>,

    /// Defaults to minecraft:spawners/trial_chamber/items_to_drop_when_ominous — A resource location to a loot table. Determines the items used by ominous item spawners spawned during the active phase when ominous. Ignored in normal mode.
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items_to_drop_when_ominous: Option<Cow<'a, str>>,
}

/// A potential loot table.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct LootTable<'a> {
    /// The chance that this loot table gets picked in comparison to other loot table weights. Must be positive and at least 1.
    pub weight: i32,

    /// A resource location to a loot table.
    #[serde(borrow)]
    pub data: Cow<'a, str>,
}

#[cfg(test)]
#[test]
fn test() {
    use fastnbt::nbt;

    let nbt = nbt!({
        "required_player_range": 14,
        "target_cooldown_length": 36000,
        "normal_config": {
            "spawn_range": 4,
            "total_mobs": 6.0f32,
            "simultaneous_mobs": 2,
            "total_mobs_added_per_player": 2.0f32,
            "simultaneous_mobs_added_per_player": 1.0f32,
            "ticks_between_spawn": 40,
            "spawn_potentials": [
                {
                    "weight": 1,
                    "data": {
                        "entity": {
                            "Air": 32i16,
                            "CustomName": "Cool Breeze",
                            "FallDistance": 0.0f32,
                            "Glowing": true,
                            "HasVisualFire": true,
                            "id": "minecraft:breeze",
                            "Invulnerable": false,
                            "Motion": [0.0f64, 0.0f64, 0.0f64],
                            "NoGravity": false,
                            "OnGround": true,
                            "PortalCooldown": 0,
                            "Pos": [5.5f64, 6.3f64, 2.1f64],
                            "Rotation": [0.0f32, 0.0f32],
                            "UUID": [I; 1, 2, 3, 4,]
                        },
                        "custom_spawn_rules": {
                            "block_light_limit": 10,
                            "sky_light_limit": 15
                        },
                        "equipment": {
                            "loot_table": "minecraft:entities/breeze",
                            "slot_drop_chances": {
                                "feet": 0.045,
                                "chest": 0.1,
                                "mainhand": 0.085
                            }
                        }
                    },
                }
            ],
            "loot_tables_to_eject": [
                {
                    "weight": 1,
                    "data": "minecraft:spawners/trial_chamber/consumables"
                },
                {
                    "weight": 1,
                    "data": "minecraft:spawners/trial_chamber/key"
                }
            ],
            "items_to_drop_when_ominous": "minecraft:spawners/trial_chamber/items_to_drop_when_ominous"
        },
        "registered_players": [
            [I; 1, 2, 3, 4],
            [I; 5, 6, 7, 8]
        ],
        "current_mobs": [
            [I; 9, 10, 11, 12],
            [I; 13, 14, 15, 16]
        ],
        "cooldown_ends_at": 0i64,
        "next_mob_spawns_at": 0i64,
        "total_mobs_spawned": 0,
        "spawn_data": {
            "entity": {
                "Air": 32i16,
                "CustomName": "Cool Breeze",
                "FallDistance": 0.0f32,
                "Glowing": true,
                "HasVisualFire": true,
                "id": "minecraft:breeze",
                "Invulnerable": false,
                "Motion": [0.0f64, 0.0f64, 0.0f64],
                "NoGravity": false,
                "OnGround": true,
                "PortalCooldown": 0,
                "Pos": [5.5f64, 6.3f64, 2.1f64],
                "Rotation": [0.0f32, 0.0f32],
                "UUID": [I; 1, 2, 3, 4,]
            },
            "custom_spawn_rules": {
                "block_light_limit": 10,
                "sky_light_limit": 15
            },
            "equipment": {
                "loot_table": "minecraft:entities/breeze",
                "slot_drop_chances": {
                    "feet": 0.045,
                    "chest": 0.1,
                    "mainhand": 0.085
                }
            }
        },
        "ejecting_loot_table": "minecraft:spawners/trial_chamber/loot_table"
    });

    let trail_spawner: TrailSpawner = fastnbt::from_value(&nbt).unwrap();

    assert_eq!(trail_spawner.required_player_range, 14);
    assert_eq!(trail_spawner.target_cooldown_length, 36000);

    let normal_config = trail_spawner.normal_config.as_ref().unwrap();

    assert_eq!(normal_config.spawn_range.unwrap(), 4);
    assert_eq!(normal_config.total_mobs.unwrap(), 6.0);
    assert_eq!(normal_config.simultaneous_mobs.unwrap(), 2);
    assert_eq!(normal_config.total_mobs_added_per_player.unwrap(), 2.0);
    assert_eq!(
        normal_config.simultaneous_mobs_added_per_player.unwrap(),
        1.0
    );
    assert_eq!(normal_config.ticks_between_spawn.unwrap(), 40);
    assert_eq!(normal_config.spawn_potentials.as_ref().unwrap().len(), 1);
    assert_eq!(
        normal_config.loot_tables_to_eject.as_ref().unwrap().len(),
        2
    );
    assert_eq!(
        normal_config.items_to_drop_when_ominous.as_ref().unwrap(),
        "minecraft:spawners/trial_chamber/items_to_drop_when_ominous"
    );

    assert_eq!(trail_spawner.registered_players.len(), 2);
    assert_eq!(trail_spawner.current_mobs.len(), 2);
    assert_eq!(trail_spawner.cooldown_ends_at, 0);
    assert_eq!(trail_spawner.next_mob_spawns_at, 0);
    assert_eq!(trail_spawner.total_mobs_spawned, 0);
    assert_eq!(
        trail_spawner.spawn_data,
        normal_config.spawn_potentials.as_ref().unwrap()[0]
            .data
            .clone()
    );
    assert_eq!(
        trail_spawner.ejecting_loot_table.as_ref().unwrap(),
        "minecraft:spawners/trial_chamber/loot_table"
    );

    let nbt = fastnbt::to_value(&trail_spawner).unwrap();

    assert_eq!(nbt, nbt);
}
