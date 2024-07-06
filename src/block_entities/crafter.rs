use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::item::Item;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Crafter<'a> {
    /// Set to 6 when the crafter crafts something
    pub crafting_ticks_remaining: i32,

    /// Set to 1 when it is powered. It is otherwise 0.
    pub triggered: bool,

    /// Indexes of slots that are disabled.
    pub disabled_slots: Vec<i32>,

    /// List of items in this container.  
    ///
    /// Crafter slots are numbered 0-8. 0 starts in the top left corner.
    #[serde(borrow)]
    #[serde(rename = "Items")]
    pub items: Vec<Item<'a>>,

    /// Optional. When not blank, prevents the container from being opened unless the opener is holding an item whose name matches this string.
    #[serde(rename = "Lock")]
    pub lock: Option<Cow<'a, str>>,

    /// Optional. Name of the loot table to use. If this is used in a chest-like container, the loot table generates its content when it is opened. Generating the items in the container removes both loot table tags ( LootTable and  LootTableSeed).
    #[serde(rename = "LootTable")]
    pub loot_table: Option<Cow<'a, str>>,

    /// Optional. Seed for generating the loot table. The default value works similarly to the seeds for worlds, where value of 0 or an omitted value causes the game to use a random seed.
    #[serde(rename = "LootTableSeed")]
    pub loot_table_seed: i64,
}
