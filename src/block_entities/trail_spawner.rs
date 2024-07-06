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
    pub normal_config: Option<TrailSpawnerConfig<'a>>,

    /// Optional, defaults to  normal_config. When individual entries are omitted, they also default to their setting in  normal_config. — The configuration to use when ominous.
    #[serde(borrow)]
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
    pub ejecting_loot_table: Option<Cow<'a, str>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct TrailSpawnerConfig<'a> {
    /// Between 1 and 128. Defaults to 4 — Maximum distance in blocks that mobs can spawn.
    pub spawn_range: Option<i32>,

    /// Defaults to 6 — Total amount of mobs spawned before cooldown for a single player.
    pub total_mobs: Option<f32>,

    /// Defaults to 2 — The amount of spawned mobs from this spawner that are allowed to exist simultaneously.
    pub simultaneous_mobs: Option<i32>,

    /// Defaults to 2 — Amount of total mobs added for each additional player.
    pub total_mobs_added_per_player: Option<f32>,

    /// Defaults to 1 — Amount of simultaneous mobs added for each additional player.
    pub simultaneous_mobs_added_per_player: Option<f32>,

    /// Defaults to 40 — Time in ticks between spawn attempts.
    pub ticks_between_spawn: Option<i32>,

    ///  Defaults to an empty list — List of possible entities to spawn.
    #[serde(borrow)]
    pub spawn_potentials: Option<Vec<PotentialSpawn<'a>>>,

    ///  Defaults to a list of `minecraft:spawners/trial_chamber/consumables` and `minecraft:spawners/trial_chamber/key`, both with weight 1 — List of possible loot tables to give as reward.
    #[serde(borrow)]
    pub loot_tables_to_eject: Option<Vec<LootTable<'a>>>,

    /// Defaults to minecraft:spawners/trial_chamber/items_to_drop_when_ominous — A resource location to a loot table. Determines the items used by ominous item spawners spawned during the active phase when ominous. Ignored in normal mode.
    #[serde(borrow)]
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
