//! Contains the `ContainerLoot` component.

use crate::{traits::FromCompoundNbt, util::get_owned_string};

/// Represents the loot table of a container.
#[derive(Debug, Clone, PartialEq)]
pub struct ContainerLoot {
    /// Reference to the loot table.
    pub loot_table: String,

    /// Its seed.
    pub seed: Option<i64>,
}

impl FromCompoundNbt for ContainerLoot {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let loot_table = get_owned_string(&nbt, "loot_table")?;

        let seed = nbt.long("seed");

        Ok(ContainerLoot { loot_table, seed })
    }
}
