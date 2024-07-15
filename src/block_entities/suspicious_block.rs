use crate::{item::Item, traits::FromCompoundNbt, util::get_loot_table_data};

/// Both loot table tags are removed once the items have been generated.
#[derive(Debug, Clone, PartialEq)]
pub struct SuspiciousBlock {
    /// Optional. Name of the loot table to use. If this is used in a chest-like container, the loot table generates its content when it is opened. Generating the items in the container removes both loot table tags ( LootTable and  LootTableSeed).
    ///
    /// `LootTable`
    pub loot_table: Option<String>,

    /// Optional. Seed for generating the loot table. The default value works similarly to the seeds for worlds, where value of 0 or an omitted value causes the game to use a random seed.
    ///
    /// `LootTableSeed`
    pub loot_table_seed: Option<i64>,

    /// The item in the block. May not exist.
    pub item: Option<Item>,
}

impl FromCompoundNbt for SuspiciousBlock {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let loot_table = get_loot_table_data(&nbt);

        let item = if let Some(item) = nbt.compound("item") {
            Some(Item::from_compound_nbt(&item)?)
        } else {
            None
        };

        Ok(SuspiciousBlock {
            loot_table: loot_table.loot_table,
            loot_table_seed: loot_table.loot_table_seed,
            item,
        })
    }
}
