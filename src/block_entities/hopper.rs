use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::item::Item;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Hopper<'a> {
    /// Optional. The name of this container in JSON text component, which appears in its GUI where the default name ordinarily appears.
    #[serde(borrow)]
    #[serde(rename = "CustomName")]
    pub custom_name: Option<Cow<'a, str>>,

    /// List of items in this container.  
    #[serde(borrow)]
    #[serde(rename = "Items")]
    pub items: Vec<Item<'a>>,

    /// Optional. When not blank, prevents the container from being opened unless the opener is holding an item whose name matches this string.
    #[serde(borrow)]
    #[serde(rename = "Lock")]
    pub lock: Option<Cow<'a, str>>,

    /// Optional. Name of the loot table to use. If this is used in a chest-like container, the loot table generates its content when it is opened. Generating the items in the container removes both loot table tags ( LootTable and  LootTableSeed).
    #[serde(borrow)]
    #[serde(rename = "LootTable")]
    pub loot_table: Option<Cow<'a, str>>,

    /// Optional. Seed for generating the loot table. The default value works similarly to the seeds for worlds, where value of 0 or an omitted value causes the game to use a random seed.
    #[serde(rename = "LootTableSeed")]
    pub loot_table_seed: i64,

    /// Time until the next transfer in game ticks, naturally between 1 and 8 or 0 if there is no transfer.
    #[serde(rename = "TransferCooldown")]
    pub transfer_cooldown: i32,
}