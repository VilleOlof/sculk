use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::item::Item;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Hopper<'a> {
    /// Optional. The name of this container in JSON text component, which appears in its GUI where the default name ordinarily appears.
    #[serde(borrow)]
    #[serde(rename = "CustomName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_name: Option<Cow<'a, str>>,

    /// List of items in this container.  
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

    /// Time until the next transfer in game ticks, naturally between 1 and 8 or 0 if there is no transfer.
    #[serde(rename = "TransferCooldown")]
    pub transfer_cooldown: i32,
}

#[cfg(test)]
#[test]
fn test() {
    use fastnbt::nbt;

    let nbt = nbt!({
        "CustomName": "Hopper",
        "Items": [
            {
                "Slot": 0u8,
                "id": "minecraft:stone",
                "Count": 1,
            }
        ],
        "Lock": "Key",
        "LootTable": "minecraft:chests/simple_dungeon",
        "LootTableSeed": 0i64,
        "TransferCooldown": 0,
    });

    let hopper: Hopper = fastnbt::from_value(&nbt).unwrap();

    assert_eq!(hopper.custom_name, Some(Cow::Borrowed("Hopper")));
    assert_eq!(hopper.items.len(), 1);
    assert_eq!(hopper.lock, Some(Cow::Borrowed("Key")));
    assert_eq!(
        hopper.loot_table,
        Some(Cow::Borrowed("minecraft:chests/simple_dungeon"))
    );
    assert_eq!(hopper.loot_table_seed, Some(0));
    assert_eq!(hopper.transfer_cooldown, 0);

    let nbt = fastnbt::to_value(&hopper).unwrap();

    assert_eq!(nbt, nbt);
}
