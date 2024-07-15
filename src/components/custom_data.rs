//! Custom data component.

use crate::{error::SculkParseError, kv::KVPair, traits::FromCompoundNbt};

/// Custom data component.
#[derive(Debug, Clone, PartialEq)]
pub enum CustomData {
    /// If its a string, it's the SNBT.
    Snbt(String),
    /// If its a compound, it's a key-value pair.
    KeyValues(KVPair<String>),
}

impl FromCompoundNbt for CustomData {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        if let Some(string) = nbt.string("minecraft:custom_data") {
            let snbt = string.to_string();
            return Ok(CustomData::Snbt(snbt));
        } else if let Some(compound) = nbt.compound("minecraft:custom_data") {
            let map = KVPair::from_compound_nbt(&compound)?;
            return Ok(CustomData::KeyValues(map));
        } else {
            return Err(SculkParseError::MissingField(
                "minecraft:custom_data".into(),
            ));
        }
    }
}
