use crate::{traits::FromCompoundNbt, util::get_owned_mutf8str};
use simdnbt::Mutf8Str;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq)]
pub struct ContainerLoot<'a> {
    pub loot_table: Cow<'a, Mutf8Str>,

    pub seed: Option<i64>,
}

impl<'a> FromCompoundNbt for ContainerLoot<'a> {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let loot_table = get_owned_mutf8str(&nbt, "loot_table")?;

        let seed = nbt.long("seed");

        Ok(ContainerLoot { loot_table, seed })
    }
}