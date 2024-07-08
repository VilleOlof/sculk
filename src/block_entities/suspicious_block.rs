use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::item::Item;

/// Both loot table tags are removed once the items have been generated.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct SuspiciousBlock<'a> {
    /// Optional. Name of the loot table to use. If this is used in a chest-like container, the loot table generates its content when it is opened. Generating the items in the container removes both loot table tags ( LootTable and  LootTableSeed).
    #[serde(borrow)]
    #[serde(rename = "LootTable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loot_table: Option<Cow<'a, str>>,

    /// Optional. Seed for generating the loot table. The default value works similarly to the seeds for worlds, where value of 0 or an omitted value causes the game to use a random seed.
    #[serde(rename = "LootTableSeed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loot_table_seed: Option<i64>,

    /// The item in the block. May not exist.
    #[serde(borrow)]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Item<'a>>,
}

#[cfg(test)]
#[test]
fn test() {
    use fastnbt::nbt;

    let nbt = nbt!({
        "LootTable": "LootTable",
        "LootTableSeed": 0i64,
        "item": {
            "Slot": 0u8,
            "id": "minecraft:stone",
            "Count": 1
        }
    });

    let suspicious_block: SuspiciousBlock = fastnbt::from_value(&nbt).unwrap();

    assert_eq!(suspicious_block.loot_table.as_ref().unwrap(), "LootTable");
    assert_eq!(suspicious_block.loot_table_seed.unwrap(), 0);

    let item = suspicious_block.item.as_ref().unwrap();

    assert_eq!(item.id, "minecraft:stone");
    assert_eq!(item.count, 1);

    let serialized_nbt = fastnbt::to_value(&suspicious_block).unwrap();

    assert_eq!(nbt, serialized_nbt);
}

#[cfg(test)]
#[test]
fn empty_test() {
    use fastnbt::nbt;

    let nbt = nbt!({});

    let suspicious_block: SuspiciousBlock = fastnbt::from_value(&nbt).unwrap();

    let serialized_nbt = fastnbt::to_value(&suspicious_block).unwrap();

    assert_eq!(nbt, serialized_nbt);
}
