use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::item::Item;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Dropper<'a> {
    /// Optional. The name of this container in JSON text component, which appears in its GUI where the default name ordinarily appears.
    #[serde(borrow)]
    #[serde(rename = "CustomName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_name: Option<Cow<'a, str>>,

    /// List of items in the container.  
    ///
    /// Dispenser slots are numbered 0-8 with 0 in the top left corner.  
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
fn test() {
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
        "Lock": "minecraft:lock",
        "LootTable": "minecraft:loot_table",
        "LootTableSeed": 42i64
    });

    let dropper: Dropper = fastnbt::from_value(&nbt).unwrap();

    assert_eq!(dropper.custom_name, None);
    assert_eq!(dropper.items.len(), 3);
    assert_eq!(dropper.lock, Some(Cow::Borrowed("minecraft:lock")));
    assert_eq!(
        dropper.loot_table,
        Some(Cow::Borrowed("minecraft:loot_table"))
    );
    assert_eq!(dropper.loot_table_seed, Some(42));

    let serialized_nbt = fastnbt::to_value(&dropper).unwrap();

    assert_eq!(nbt, serialized_nbt);
}
