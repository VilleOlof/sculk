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
        "sherds": [
            "minecraft:heartbreak_pottery_sherd",
            "minecraft:blade_pottery_sherd",
            "minecraft:brick",
            "minecraft:prize_pottery_sherd",
        ],
        "item": {
            "Slot": 0u8,
            "id": "minecraft:flower_pot",
            "Count": 1
        }
    });

    let decorated_pot: DecoratedPot = fastnbt::from_value(&nbt).unwrap();

    assert_eq!(decorated_pot.sherds.len(), 4);
    assert_eq!(decorated_pot.item.id, "minecraft:flower_pot");
    assert_eq!(decorated_pot.item.count, 1);

    let serialized_nbt = fastnbt::to_value(&decorated_pot).unwrap();

    assert_eq!(nbt, serialized_nbt);
}
