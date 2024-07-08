use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::item::Item;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Barrel<'a> {
    /// Optional. The name of this container in JSON text component, which appears in its GUI where the default name ordinarily appears.
    #[serde(borrow)]
    #[serde(rename = "CustomName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_name: Option<Cow<'a, str>>,

    /// List of items in the container.
    #[serde(borrow)]
    #[serde(rename = "Items")]
    #[serde(default)]
    pub items: Vec<Item<'a>>,

    /// Optional. When not blank, prevents the container from being opened unless the opener is holding an item whose name matches this string
    #[serde(borrow)]
    #[serde(rename = "Lock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock: Option<Cow<'a, str>>,

    /// Optional. Name of the loot table to use. If this is used in a chest-like container, the loot table generates its content when it is opened. Generating the items in the container removes both loot table tags ( LootTable and  LootTableSeed).
    #[serde(borrow)]
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
        "Lock": "my_key"
    });

    let barrel: Barrel = fastnbt::from_value(&nbt).unwrap();

    assert_eq!(barrel.custom_name, None);
    assert_eq!(barrel.items.len(), 3);
    assert_eq!(barrel.lock, Some(Cow::Borrowed("my_key")));
    assert_eq!(barrel.loot_table, None);
    assert_eq!(barrel.loot_table_seed, None);

    let serialized_nbt = fastnbt::to_value(&barrel).unwrap();

    assert_eq!(nbt, serialized_nbt);
}

#[cfg(test)]
#[test]
fn empty_test() {
    use fastnbt::nbt;

    let nbt = nbt!({
        "Items": []
    });

    let barrel: Barrel = fastnbt::from_value(&nbt).unwrap();

    assert_eq!(barrel.custom_name, None);
    assert_eq!(barrel.items.len(), 0);
    assert_eq!(barrel.lock, None);
    assert_eq!(barrel.loot_table, None);
    assert_eq!(barrel.loot_table_seed, None);

    let serialized_nbt = fastnbt::to_value(&barrel).unwrap();

    assert_eq!(nbt, serialized_nbt);
}

#[cfg(test)]
#[test]
fn extended_test() {
    use fastnbt::nbt;

    let nbt = nbt!({
        "CustomName": "My barrel",
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
        "Lock": "my_key",
        "LootTable": "minecraft:chests/simple_dungeon",
        "LootTableSeed": 1234567890i64
    });

    let barrel: Barrel = fastnbt::from_value(&nbt).unwrap();

    assert_eq!(barrel.custom_name, Some(Cow::Borrowed("My barrel")));
    assert_eq!(barrel.items.len(), 3);
    assert_eq!(barrel.lock, Some(Cow::Borrowed("my_key")));
    assert_eq!(
        barrel.loot_table,
        Some(Cow::Borrowed("minecraft:chests/simple_dungeon"))
    );
    assert_eq!(barrel.loot_table_seed, Some(1234567890));

    let serialized_nbt = fastnbt::to_value(&barrel).unwrap();

    assert_eq!(nbt, serialized_nbt);
}
