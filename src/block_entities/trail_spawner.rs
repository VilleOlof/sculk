use crate::{
    error::SculkParseError,
    traits::FromCompoundNbt,
    util::{get_owned_optional_string, get_owned_string},
    uuid::Uuid,
};

use super::mob_spawner::{PotentialSpawn, SpawnData};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TrailSpawner {
    /// Between 1 and 128. Defaults to 14. — Maximum distance in blocks for players to join the battle.
    pub required_player_range: i32,

    /// Defaults to 36000. — Time in ticks of the cooldown period. Includes the time spend dispensing the reward.
    pub target_cooldown_length: i32,

    /// Optional, see configuration for defaults. — The configuration to use when not ominous.
    pub normal_config: Option<TrailSpawnerConfig>,

    /// Optional, defaults to  normal_config. When individual entries are omitted, they also default to their setting in  normal_config. — The configuration to use when ominous.
    pub ominous_config: Option<TrailSpawnerConfig>,

    /// A set of player UUIDs. — All the players that have joined the battle. The length of this array determines the amount of mobs and amount of reward.
    pub registered_players: Vec<Uuid>,

    /// A set of mob UUIDs. — The mobs that were spawned by this spawner and are still alive.
    pub current_mobs: Vec<Uuid>,

    /// Gametime in ticks when the cooldown ends. 0 if not currently in cooldown.
    pub cooldown_ends_at: i64,

    /// Gametime in ticks when the next spawn attempt happens. 0 if not currently active.
    pub next_mob_spawns_at: i64,

    /// The total amount of mobs that have already been spawned in this cycle. 0 if not currently active.
    pub total_mobs_spawned: i32,

    /// The next mob to attempt to spawn. Selected from  spawn_potentials after the last attempt. Determines the mob displayed in the spawner.
    pub spawn_data: SpawnData,

    /// A resource location to the loot table that is given as reward. Unset if not currently giving rewards. Selected from  loot_tables_to_eject after all mobs are defeated.
    pub ejecting_loot_table: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TrailSpawnerConfig {
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
    pub spawn_potentials: Option<Vec<PotentialSpawn>>,

    ///  Defaults to a list of `minecraft:spawners/trial_chamber/consumables` and `minecraft:spawners/trial_chamber/key`, both with weight 1 — List of possible loot tables to give as reward.
    pub loot_tables_to_eject: Option<Vec<LootTable>>,

    /// Defaults to minecraft:spawners/trial_chamber/items_to_drop_when_ominous — A resource location to a loot table. Determines the items used by ominous item spawners spawned during the active phase when ominous. Ignored in normal mode.
    pub items_to_drop_when_ominous: Option<String>,
}

/// A potential loot table.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LootTable {
    /// The chance that this loot table gets picked in comparison to other loot table weights. Must be positive and at least 1.
    pub weight: i32,

    /// A resource location to a loot table.
    pub data: String,
}

impl FromCompoundNbt for TrailSpawner {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let required_player_range = nbt.int("required_player_range").unwrap_or(14);

        let target_cooldown_length = nbt.int("target_cooldown_length").unwrap_or(36000);

        let normal_config = if let Some(nbt) = nbt.compound("normal_config") {
            Some(TrailSpawnerConfig::from_compound_nbt(&nbt)?)
        } else {
            None
        };
        let ominous_config = if let Some(nbt) = nbt.compound("ominous_config") {
            Some(TrailSpawnerConfig::from_compound_nbt(&nbt)?)
        } else {
            None
        };

        let registered_players = Uuid::from_nbt_to_vec(&nbt, "registered_players");
        let current_mobs = Uuid::from_nbt_to_vec(&nbt, "current_mobs");

        let cooldown_ends_at = nbt.long("cooldown_ends_at").unwrap_or(0);
        let next_mob_spawns_at = nbt.long("next_mob_spawns_at").unwrap_or(0);
        let total_mobs_spawned = nbt.int("total_mobs_spawned").unwrap_or(0);

        let spawn_data = nbt
            .compound("spawn_data")
            .map(|nbt| SpawnData::from_compound_nbt(&nbt))
            .ok_or(SculkParseError::MissingField("spawn_data".into()))??;

        let ejecting_loot_table = get_owned_optional_string(&nbt, "ejecting_loot_table");

        Ok(TrailSpawner {
            required_player_range,
            target_cooldown_length,
            normal_config,
            ominous_config,
            registered_players,
            current_mobs,
            cooldown_ends_at,
            next_mob_spawns_at,
            total_mobs_spawned,
            spawn_data,
            ejecting_loot_table,
        })
    }
}

impl FromCompoundNbt for TrailSpawnerConfig {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let spawn_range = nbt.int("spawn_range");
        let total_mobs = nbt.float("total_mobs");
        let simultaneous_mobs = nbt.int("simultaneous_mobs");
        let total_mobs_added_per_player = nbt.float("total_mobs_added_per_player");
        let simultaneous_mobs_added_per_player = nbt.float("simultaneous_mobs_added_per_player");
        let ticks_between_spawn = nbt.int("ticks_between_spawn");

        let spawn_potentials = if let Some(spawn_potentials) = nbt.list("spawn_potentials") {
            let mut potentials = vec![];

            for potential in spawn_potentials
                .compounds()
                .ok_or(SculkParseError::InvalidField("spawn_potentials".into()))?
            {
                potentials.push(PotentialSpawn::from_compound_nbt(&potential)?);
            }

            Some(potentials)
        } else {
            None
        };

        let loot_tables_to_eject = if let Some(nbt) = nbt.list("loot_tables_to_eject") {
            let mut loot_tables = vec![];

            for loot_table in nbt
                .compounds()
                .ok_or(SculkParseError::InvalidField("loot_tables_to_eject".into()))?
            {
                loot_tables.push(LootTable::from_compound_nbt(&loot_table)?);
            }

            Some(loot_tables)
        } else {
            None
        };

        let items_to_drop_when_ominous =
            get_owned_optional_string(&nbt, "items_to_drop_when_ominous");

        Ok(TrailSpawnerConfig {
            spawn_range,
            total_mobs,
            simultaneous_mobs,
            total_mobs_added_per_player,
            simultaneous_mobs_added_per_player,
            ticks_between_spawn,
            spawn_potentials,
            loot_tables_to_eject,
            items_to_drop_when_ominous,
        })
    }
}

impl FromCompoundNbt for LootTable {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let weight = nbt
            .int("weight")
            .ok_or(SculkParseError::MissingField("weight".into()))?;

        let data = get_owned_string(&nbt, "data")?;

        Ok(LootTable { weight, data })
    }
}
