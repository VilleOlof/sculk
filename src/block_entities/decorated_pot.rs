use simdnbt::Mutf8Str;

use crate::{
    error::SculkParseError, item::Item, traits::FromCompoundNbt, util::get_loot_table_data,
};
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq)]
pub struct DecoratedPot<'a> {
    /// List of sherds on this decorated pot.  
    ///
    /// Item ID of this face. Each value defaults to bricks's ID, and can be either a brick or any sherd.  
    pub sherds: Vec<Cow<'a, Mutf8Str>>,

    /// The item stored within the pot. A decorated pot does not use Slot to describe its contents, even though it functionally has 1 item slot.
    pub item: Item<'a>,

    /// Optional. Name of the loot table to use. If this is used in a chest-like container, the loot table generates its content when it is opened. Generating the items in the container removes both loot table tags ( LootTable and  LootTableSeed).
    ///
    /// `LootTable`
    pub loot_table: Option<Cow<'a, Mutf8Str>>,

    /// Optional. Seed for generating the loot table. The default value works similarly to the seeds for worlds, where value of 0 or an omitted value causes the game to use a random seed.
    ///
    /// `LootTableSeed`
    pub loot_table_seed: Option<i64>,
}

impl<'a> FromCompoundNbt for DecoratedPot<'a> {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let sherds_list = nbt
            .list("sherds")
            .ok_or(SculkParseError::MissingField("sherds".into()))?;
        let mut sherds: Vec<Cow<'a, Mutf8Str>> = vec![];
        for sherd in sherds_list.strings().into_iter() {
            // maybe???
            let str = (*sherd.first().unwrap()).to_owned();
            sherds.push(Cow::Owned(str));
        }

        let item = nbt
            .compound("Item")
            .map(|i| Item::from_compound_nbt(&i))
            .ok_or(SculkParseError::MissingField("Item".into()))??;

        let loot_table = get_loot_table_data(&nbt);

        Ok(DecoratedPot {
            sherds,
            item,
            loot_table: loot_table.loot_table,
            loot_table_seed: loot_table.loot_table_seed,
        })
    }
}
