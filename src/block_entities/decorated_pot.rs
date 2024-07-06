use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::item::Item;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct DecoratedPot<'a> {
    /// List of sherds on this decorated pot.  
    ///
    /// Item ID of this face. Each value defaults to bricks's ID, and can be either a brick or any sherd.  
    pub sherds: Vec<Cow<'a, str>>,

    /// The item stored within the pot. A decorated pot does not use Slot to describe its contents, even though it functionally has 1 item slot.
    #[serde(borrow)]
    pub item: Item<'a>,

    /// Optional. Name of the loot table to use. If this is used in a chest-like container, the loot table generates its content when it is opened. Generating the items in the container removes both loot table tags ( LootTable and  LootTableSeed).
    #[serde(rename = "LootTable")]
    pub loot_table: Option<Cow<'a, str>>,

    /// Optional. Seed for generating the loot table. The default value works similarly to the seeds for worlds, where value of 0 or an omitted value causes the game to use a random seed.
    #[serde(rename = "LootTableSeed")]
    pub loot_table_seed: i64,
}