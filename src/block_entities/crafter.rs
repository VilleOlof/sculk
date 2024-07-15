use crate::{
    error::SculkParseError,
    item::Item,
    traits::FromCompoundNbt,
    util::{get_bool, get_int_array, get_loot_table_data, get_optional_lock, get_t_compound_vec},
};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Crafter {
    /// Set to 6 when the crafter crafts something
    pub crafting_ticks_remaining: i32,

    /// Set to 1 when it is powered. It is otherwise 0.
    pub triggered: bool,

    /// Indexes of slots that are disabled.
    pub disabled_slots: Vec<i32>,

    /// List of items in this container.  
    ///
    /// Crafter slots are numbered 0-8. 0 starts in the top left corner.
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

impl FromCompoundNbt for Crafter {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let crafting_ticks_remaining =
            nbt.int("crafting_ticks_remaining")
                .ok_or(SculkParseError::MissingField(
                    "crafting_ticks_remaining".into(),
                ))?;
        let triggered = get_bool(&nbt, "triggered");
        let disabled_slots = get_int_array(&nbt, "disabled_slots")?;

        let items = get_t_compound_vec(&nbt, "Items", Item::from_compound_nbt)?;
        let lock = get_optional_lock(&nbt);
        let loot_table = get_loot_table_data(&nbt);

        Ok(Crafter {
            crafting_ticks_remaining,
            triggered,
            disabled_slots,
            items,
            lock,
            loot_table: loot_table.loot_table,
            loot_table_seed: loot_table.loot_table_seed,
        })
    }
}
