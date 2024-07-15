//! Bee component for entities in a hive.

use crate::{entity::Entity, error::SculkParseError, traits::FromCompoundNbt};

/// A bee entity in a hive.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Bee {
    /// The NBT data of the entity in the hive.
    pub entity_data: Entity,

    /// The minimum amount of time in ticks for this entity to stay in the hive.
    pub min_ticks_in_hive: i32,

    /// The amount of ticks the entity has stayed in the hive.
    pub ticks_in_hive: i32,
}

impl FromCompoundNbt for Bee {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let entity_data = Entity::from_compound_nbt(nbt)?;

        let min_ticks_in_hive = nbt
            .int("min_ticks_in_hive")
            .ok_or(SculkParseError::MissingField("min_ticks_in_hive".into()))?;
        let ticks_in_hive = nbt
            .int("ticks_in_hive")
            .ok_or(SculkParseError::MissingField("ticks_in_hive".into()))?;

        Ok(Bee {
            entity_data,
            min_ticks_in_hive,
            ticks_in_hive,
        })
    }
}
