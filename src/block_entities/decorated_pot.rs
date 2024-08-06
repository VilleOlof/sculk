use crate::{
    error::SculkParseError, item::ItemWithNoSlot, traits::FromCompoundNbt,
    util::get_loot_table_data,
};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DecoratedPot {
    /// List of sherds on this decorated pot.  
    ///
    /// Item ID of this face. Each value defaults to bricks's ID, and can be either a brick or any sherd.  
    pub sherds: Vec<String>,

    /// The item stored within the pot. A decorated pot does not use Slot to describe its contents, even though it functionally has 1 item slot.
    pub item: Option<ItemWithNoSlot>,

    /// Optional. Name of the loot table to use. If this is used in a chest-like container, the loot table generates its content when it is opened. Generating the items in the container removes both loot table tags ( LootTable and  LootTableSeed).
    ///
    /// `LootTable`
    pub loot_table: Option<String>,

    /// Optional. Seed for generating the loot table. The default value works similarly to the seeds for worlds, where value of 0 or an omitted value causes the game to use a random seed.
    ///
    /// `LootTableSeed`
    pub loot_table_seed: Option<i64>,
}

impl FromCompoundNbt for DecoratedPot {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let sherds = if let Some(sherds_list) = nbt.list("sherds") {
            let mut sherds: Vec<String> = vec![];
            for sherd in sherds_list.strings().into_iter() {
                // maybe???
                let str = (*sherd.first().unwrap()).to_string();
                sherds.push(str);
            }

            sherds
        } else {
            vec![]
        };

        let item = if let Some(item) = nbt.compound("item") {
            Some(ItemWithNoSlot::from_compound_nbt(&item)?)
        } else {
            None
        };

        let loot_table = get_loot_table_data(&nbt);

        Ok(DecoratedPot {
            sherds,
            item,
            loot_table: loot_table.loot_table,
            loot_table_seed: loot_table.loot_table_seed,
        })
    }
}
