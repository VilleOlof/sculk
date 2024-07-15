use crate::{
    entity::MaybeEntity, error::SculkParseError, traits::FromCompoundNbt, util::get_owned_string,
};
use simdnbt::borrow::NbtCompound;

#[derive(Debug, Clone, PartialEq)]
pub struct MobSpawner {
    /// Ticks until next spawn. If 0, it spawns immediately when a player enters its range. If set to -1 (this state never occurs in a natural spawner; it seems to be a feature accessed only via NBT editing), the spawner resets this and SpawnData as though it had just completed a successful spawn cycle, immediately when a player enters its range. Setting this to -1 can be useful if the player wants the game to properly randomize the spawner's Delay and SpawnData, rather than starting with pre-defined values.
    ///
    /// `Delay`
    pub delay: i16,

    /// Overrides the maximum number of nearby (within a box of SpawnRange*2+1 × SpawnRange*2+1 × 8 centered around the spawner block) entities whose IDs match this spawner's entity ID. This is relative to a mob's hitbox, not its physical position. Also, all entities within all chunk sections (16×16×16 cubes) overlapped by this box are tested for their ID and hitbox overlap, rather than just entities within the box, meaning that a large amount of entities outside the box (or within it, of course) can cause substantial lag
    ///
    /// `MaxNearbyEntities`
    pub max_nearby_entities: i16,

    /// The maximum random delay for the next spawn delay. Requires the MinSpawnDelay and SpawnCount properties to also be set.
    ///
    /// `MaxSpawnDelay`
    pub max_spawn_delay: i16,

    /// The minimum random delay for the next spawn delay. May be equal to MaxSpawnDelay. Requires the SpawnCount property to also be set, otherwise it defaults to 0.
    ///
    /// `MinSpawnDelay`
    pub min_spawn_delay: i16,

    /// Overrides the block radius of the sphere of activation by players for this spawner. For every gametick, a spawner checks all players in the current world to test whether a player is within this sphere. Requires the MaxNearbyEntities property to also be set.
    ///
    /// `RequiredPlayerRange`
    pub required_player_range: i16,

    /// How many mobs to attempt to spawn each time. Requires the MinSpawnDelay property to also be set.
    ///
    /// `SpawnCount`
    pub spawn_count: i16,

    /// Contains tags to copy to the next spawned entity(s) after spawning. Any of the entity or mob tags may be used. If a spawner specifies any of these tags, almost all variable data such as mob equipment, villager profession, sheep wool color, etc., are not automatically generated, and must also be manually specified (that this does not apply to position data, which are randomized as normal unless Pos is specified. Similarly, unless Size and Health are specified for a Slime or Magma Cube, these are still randomized). This also determines the appearance of the miniature entity spinning in the spawner cage. Warning: If SpawnPotentials exists, this tag gets overwritten after the next spawning attempt: see above for more details.
    ///
    /// `SpawnData`
    pub spawn_data: SpawnData,

    /// Optional. List of possible entities to spawn. If this tag does not exist, but SpawnData exists, Minecraft generates it the next time the spawner tries to spawn an entity. The generated list contains a single entry derived from the SpawnData tag.
    ///
    /// `SpawnPotentials`
    pub spawn_potentials: Option<Vec<PotentialSpawn>>,

    /// The radius around which the spawner attempts to place mobs randomly. The spawn area is square, includes the block the spawner is in, and is centered around the spawner's x,z coordinates - not the spawner itself. It is 2 blocks high, centered around the spawner's y coordinate (its bottom), allowing mobs to spawn as high as its top surface and as low as 1 block below its bottom surface. Vertical spawn coordinates are integers, while horizontal coordinates are floating point and weighted toward values near the spawner itself. Default value is 4.
    ///
    /// `SpawnRange`
    pub spawn_range: i16,
}

/// A potential future spawn. After the spawner makes an attempt at spawning, it chooses one of these entries at random and uses it to prepare for the next spawn, overwriting SpawnData.
#[derive(Debug, Clone, PartialEq)]
pub struct PotentialSpawn {
    /// The chance that this spawn gets picked in comparison to other spawn weights. Must be positive and at least 1.
    pub weight: i32,

    pub data: SpawnData,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpawnData {
    /// An entity, including the entity id.
    pub entity: MaybeEntity,

    /// Optional custom fields to override spawning requirements.
    pub custom_spawn_rules: Option<SpawnRules>,

    /// Optional. Determines the equipment the entity will wear.
    pub equipment: Option<Equipment>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpawnRules {
    /// Can either be a single value, or a compound containing min_inclusive and max_inclusive. Overrides the block light check when spawning the entity.
    pub block_light_limit: i32,

    /// Can either be a single value, or a compound containing min_inclusive and max_inclusive. Overrides the sky light check when spawning the entity.
    pub sky_light_limit: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Equipment {
    /// Resource location of a loot table to use to generate the equipment
    pub loot_table: String,

    /// Optional. When a determines the drop chances for every slot. When a , controls the drop chances per slot.
    pub slot_drop_chances: Option<DropChanceType>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DropChanceType {
    All(f32),
    Indiviual(DropChances),
}

#[derive(Debug, Clone, PartialEq)]
pub struct DropChances {
    /// Optional. Drop chance of the boots.
    pub feet: Option<f32>,

    /// Optional. Drop chance of the leggings.
    pub legs: Option<f32>,

    /// Optional. Drop chance of the chestplate.
    pub chest: Option<f32>,

    /// Optional. Drop chance of the helmet.
    pub head: Option<f32>,

    /// Optional. Drop chance of the body armor.
    pub body: Option<f32>,

    /// Optional. Drop chance of the weapon in the main hand.
    pub mainhand: Option<f32>,

    /// Optional. Drop chance of the weapon in the off hand.
    pub offhand: Option<f32>,
}

impl FromCompoundNbt for MobSpawner {
    fn from_compound_nbt(nbt: &NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let delay = nbt
            .short("Delay")
            .ok_or(SculkParseError::MissingField("Delay".into()))?;
        let max_nearby_entities = nbt
            .short("MaxNearbyEntities")
            .ok_or(SculkParseError::MissingField("MaxNearbyEntities".into()))?;
        let max_spawn_delay = nbt
            .short("MaxSpawnDelay")
            .ok_or(SculkParseError::MissingField("MaxSpawnDelay".into()))?;
        let min_spawn_delay = nbt
            .short("MinSpawnDelay")
            .ok_or(SculkParseError::MissingField("MinSpawnDelay".into()))?;
        let required_player_range = nbt
            .short("RequiredPlayerRange")
            .ok_or(SculkParseError::MissingField("RequiredPlayerRange".into()))?;
        let spawn_count = nbt
            .short("SpawnCount")
            .ok_or(SculkParseError::MissingField("SpawnCount".into()))?;

        let spawn_data = nbt
            .compound("SpawnData")
            .map(|nbt| SpawnData::from_compound_nbt(&nbt))
            .ok_or(SculkParseError::MissingField("SpawnData".into()))??;

        let spawn_potentials = if let Some(spawn_potentials) = nbt.list("SpawnPotentials") {
            if spawn_potentials.empty() {
                None
            } else {
                let mut potentials = vec![];

                // unsure if this is correct
                for potential in spawn_potentials
                    .compounds()
                    .ok_or(SculkParseError::InvalidField("SpawnPotentials".into()))?
                {
                    potentials.push(PotentialSpawn::from_compound_nbt(&potential)?);
                }

                Some(potentials)
            }
        } else {
            None
        };

        let spawn_range = nbt.short("SpawnRange").unwrap_or(4);

        Ok(MobSpawner {
            delay,
            max_nearby_entities,
            max_spawn_delay,
            min_spawn_delay,
            required_player_range,
            spawn_count,
            spawn_data,
            spawn_potentials,
            spawn_range,
        })
    }
}

impl FromCompoundNbt for PotentialSpawn {
    fn from_compound_nbt(nbt: &NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        Ok(PotentialSpawn {
            weight: nbt
                .int("weight")
                .ok_or(SculkParseError::MissingField("weight".into()))?,
            data: nbt
                .compound("data")
                .map(|nbt| SpawnData::from_compound_nbt(&nbt))
                .ok_or(SculkParseError::MissingField("data".into()))??,
        })
    }
}

impl FromCompoundNbt for SpawnData {
    fn from_compound_nbt(nbt: &NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let entity = nbt
            .compound("entity")
            .map(|nbt| MaybeEntity::from_compound_nbt(&nbt))
            .ok_or(SculkParseError::MissingField("data-entity".into()))??;
        let custom_spawn_rules = if let Some(nbt) = nbt.compound("custom_spawn_rules") {
            Some(SpawnRules::from_compound_nbt(&nbt)?)
        } else {
            None
        };

        let equipment = if let Some(equipment) = nbt.compound("equipment") {
            Some(Equipment::from_compound_nbt(&equipment)?)
        } else {
            None
        };

        Ok(SpawnData {
            entity,
            custom_spawn_rules,
            equipment,
        })
    }
}

impl FromCompoundNbt for SpawnRules {
    fn from_compound_nbt(nbt: &NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        Ok(SpawnRules {
            block_light_limit: nbt
                .int("block_light_limit")
                .ok_or(SculkParseError::MissingField("block_light_limit".into()))?,

            sky_light_limit: nbt
                .int("sky_light_limit")
                .ok_or(SculkParseError::MissingField("sky_light_limit".into()))?,
        })
    }
}

impl FromCompoundNbt for Equipment {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let loot_table = get_owned_string(&nbt, "loot_table")?;
        let slot_drop_chances = get_slot_drop_chances(&nbt);

        Ok(Equipment {
            loot_table,
            slot_drop_chances,
        })
    }
}

/// Just wraps [`DropChanceType::from_compound_nbt`] into a [`Option<DropChanceType>`]
fn get_slot_drop_chances(nbt: &NbtCompound) -> Option<DropChanceType> {
    match DropChanceType::from_compound_nbt(&nbt) {
        Ok(slot_drop_chances) => Some(slot_drop_chances),
        Err(_) => None,
    }
}

impl FromCompoundNbt for DropChanceType {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        if let Some(chance) = nbt.float("slot_drop_chances") {
            Ok(DropChanceType::All(chance))
        } else if let Some(compound) = nbt.compound("slot_drop_chances") {
            let drop_chances = DropChances::from_compound_nbt(&compound)?;
            Ok(DropChanceType::Indiviual(drop_chances))
        } else {
            Err(SculkParseError::MissingField("slot_drop_chances".into()))
        }
    }
}

impl FromCompoundNbt for DropChances {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        Ok(DropChances {
            feet: nbt.float("feet"),
            legs: nbt.float("legs"),
            chest: nbt.float("chest"),
            head: nbt.float("head"),
            body: nbt.float("body"),
            mainhand: nbt.float("mainhand"),
            offhand: nbt.float("offhand"),
        })
    }
}
