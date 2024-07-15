use crate::{
    item::Item,
    traits::FromCompoundNbt,
    util::{get_loot_table_data, get_optional_lock, get_optional_name, get_t_compound_vec},
};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Dispenser {
    /// Optional. The name of this container in JSON text component, which appears in its GUI where the default name ordinarily appears.
    ///
    /// `CustomName`
    pub custom_name: Option<String>,

    /// List of items in the container.  
    ///
    /// Dispenser slots are numbered 0-8 with 0 in the top left corner.  
    ///
    /// `Items`
    pub items: Vec<Item>,

    /// Optional. When not blank, prevents the container from being opened unless the opener is holding an item whose name matches this string.
    ///
    /// `Lock`
    pub lock: Option<String>,

    /// Optional. Name of the loot table to use. If this is used in a chest-like container, the loot table generates its content when it is opened. Generating the items in the container removes both loot table tags ( LootTable and  LootTableSeed).
    ///
    /// `LootTable`
    pub loot_table: Option<String>,

    /// Optional. Seed for generating the loot table. The default value works similarly to the seeds for worlds, where value of 0 or an omitted value causes the game to use a random seed.
    ///
    /// `LootTableSeed`
    pub loot_table_seed: Option<i64>,
}

impl FromCompoundNbt for Dispenser {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let items = get_t_compound_vec(&nbt, "Items", Item::from_compound_nbt)?;

        let custom_name = get_optional_name(&nbt);
        let lock = get_optional_lock(&nbt);
        let loot_table = get_loot_table_data(&nbt);

        Ok(Dispenser {
            custom_name,
            items,
            lock,
            loot_table: loot_table.loot_table,
            loot_table_seed: loot_table.loot_table_seed,
        })
    }
}
