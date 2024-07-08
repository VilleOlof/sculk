use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::item::Item;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ShulkerBox<'a> {
    /// Optional. The name of this container in JSON text component, which appears in its GUI where the default name ordinarily appears.
    #[serde(borrow)]
    #[serde(rename = "CustomName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_name: Option<Cow<'a, str>>,

    /// List of items in this container.  
    ///
    /// Shulker box slots are numbered 0–26, 0 starts in the top left corner.
    #[serde(borrow)]
    #[serde(default)]
    #[serde(rename = "Items")]
    pub items: Vec<Item<'a>>,

    /// Optional. When not blank, prevents the container from being opened unless the opener is holding an item whose name matches this string.
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
fn test() {
    use fastnbt::nbt;

    let nbt = nbt!({
        "CustomName": "Custom Name",
        "Items": [
            {
                "Slot": 0u8,
                "id": "minecraft:stone",
                "Count": 1
            }
        ],
        "Lock": "Lock",
        "LootTable": "LootTable",
        "LootTableSeed": 0i64
    });

    let shulker_box: ShulkerBox = fastnbt::from_value(&nbt).unwrap();

    assert_eq!(shulker_box.custom_name.as_ref().unwrap(), "Custom Name");
    assert_eq!(shulker_box.items.len(), 1);
    assert_eq!(shulker_box.lock.as_ref().unwrap(), "Lock");
    assert_eq!(shulker_box.loot_table.as_ref().unwrap(), "LootTable");
    assert_eq!(shulker_box.loot_table_seed.unwrap(), 0);

    let item = shulker_box.items.get(0).unwrap();

    assert_eq!(item.id, "minecraft:stone");
    assert_eq!(item.count, 1);

    let nbt = fastnbt::to_value(&shulker_box).unwrap();

    assert_eq!(nbt, nbt);
}

#[cfg(test)]
#[test]
fn empty_test() {
    use fastnbt::nbt;

    let nbt = nbt!({
        "Items": []
    });

    let shulker_box: ShulkerBox = fastnbt::from_value(&nbt).unwrap();

    assert_eq!(shulker_box.custom_name, None);
    assert_eq!(shulker_box.items.len(), 0);
    assert_eq!(shulker_box.lock, None);
    assert_eq!(shulker_box.loot_table, None);
    assert_eq!(shulker_box.loot_table_seed, None);

    let serialized_nbt = fastnbt::to_value(&shulker_box).unwrap();

    assert_eq!(nbt, serialized_nbt);
}
