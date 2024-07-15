//! Base color of the banner applied on a shield.

use crate::{traits::FromCompoundNbt, util::get_owned_string};

/// The base dye color of the banner applied on a shield.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BaseColor(String);

impl FromCompoundNbt for BaseColor {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let base_color = get_owned_string(&nbt, "minecraft:base_color")?;

        Ok(BaseColor(base_color))
    }
}
