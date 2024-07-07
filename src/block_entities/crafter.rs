use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::item::Item;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Crafter<'a> {
    /// Set to 6 when the crafter crafts something
    pub crafting_ticks_remaining: i32,

    /// Set to 1 when it is powered. It is otherwise 0.
    #[serde(deserialize_with = "crate::util::i8_to_bool")]
    pub triggered: bool,

    /// Indexes of slots that are disabled.
    pub disabled_slots: Vec<i32>,

    /// List of items in this container.  
    ///
    /// Crafter slots are numbered 0-8. 0 starts in the top left corner.
    #[serde(borrow)]
    #[serde(default)]
    #[serde(rename = "Items")]
    pub items: Vec<Item<'a>>,

    /// Optional. When not blank, prevents the container from being opened unless the opener is holding an item whose name matches this string.
    #[serde(rename = "Lock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock: Option<Cow<'a, str>>,

    /// Optional. Name of the loot table to use. If this is used in a chest-like container, the loot table generates its content when it is opened. Generating the items in the container removes both loot table tags ( LootTable and  LootTableSeed).
    #[serde(rename = "LootTable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loot_table: Option<Cow<'a, str>>,

    /// Optional. Seed for generating the loot table. The default value works similarly to the seeds for worlds, where value of 0 or an omitted value causes the game to use a random seed.
    #[serde(rename = "LootTableSeed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loot_table_seed: Option<i64>,
}

#[cfg(test)]
#[test]
fn basic_test() {
    use fastnbt::nbt;

    let nbt = nbt!({
        "Items": [
            {
                "Slot": 0u8,
                "id": "minecraft:stone",
                "Count": 1
            },
            {
                "Slot": 1u8,
                "id": "minecraft:dirt",
                "Count": 32
            },
            {
                "Slot": 2u8,
                "id": "minecraft:iron_ingot",
                "Count": 64,
                "components": {}
            }
        ],
        "Lock": "key",
        "LootTable": "minecraft:chests/simple_dungeon",
        "LootTableSeed": 0i64,
        "crafting_ticks_remaining": 0,
        "triggered": 1i8,
        "disabled_slots": [3, 4, 5]
    });

    let crafter: Crafter = fastnbt::from_value(&nbt).unwrap();

    assert_eq!(crafter.crafting_ticks_remaining, 0);
    assert_eq!(crafter.triggered, true);
    assert_eq!(crafter.disabled_slots, vec![3, 4, 5]);

    let nbt = fastnbt::to_value(&crafter).unwrap();

    assert_eq!(nbt, nbt);
}

#[cfg(test)]
#[test]
fn empty_test() {
    use fastnbt::nbt;

    let nbt = nbt!({
        "Items": [],
        "crafting_ticks_remaining": 0,
        "triggered": 0i8,
        "disabled_slots": []
    });

    let crafter: Crafter = fastnbt::from_value(&nbt).unwrap();

    assert_eq!(crafter.crafting_ticks_remaining, 0);
    assert_eq!(crafter.triggered, false);
    assert_eq!(crafter.disabled_slots, Vec::<i32>::new());
    assert_eq!(crafter.lock, None);
    assert_eq!(crafter.loot_table, None);
    assert_eq!(crafter.loot_table_seed, None);

    let nbt = fastnbt::to_value(&crafter).unwrap();

    assert_eq!(nbt, nbt);
}
