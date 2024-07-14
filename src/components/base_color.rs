//! Base color of the banner applied on a shield.

use crate::{traits::FromCompoundNbt, util::get_owned_mutf8str};
use simdnbt::Mutf8Str;
use std::borrow::Cow;

/// The base dye color of the banner applied on a shield.
#[derive(Debug, Clone, PartialEq)]
pub struct BaseColor<'a>(Cow<'a, Mutf8Str>);

impl<'a> FromCompoundNbt for BaseColor<'a> {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let base_color = get_owned_mutf8str(&nbt, "minecraft:base_color")?;

        Ok(BaseColor(base_color))
    }
}
