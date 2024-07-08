use serde::{Deserialize, Serialize};

use crate::item::Item;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Campfire<'a> {
    /// How long each item has been cooking, first index is slot 0, etc.
    #[serde(rename = "CookingTimes")]
    pub cooking_times: [i32; 4],

    /// How long each item has to cook, first index is slot 0, etc.
    #[serde(rename = "CookingTotalTimes")]
    pub cookning_total_times: [i32; 4],

    /// : List of up to 4 items currently cooking.
    #[serde(borrow)]
    #[serde(default)]
    #[serde(rename = "Items")]
    pub items: Vec<Item<'a>>,
}

#[cfg(test)]
#[test]
fn test() {
    use fastnbt::nbt;

    let nbt = nbt!({
        "CookingTimes": [100i32, 200i32, 300i32, 400i32],
        "CookingTotalTimes": [200i32, 400i32, 600i32, 800i32],
        "Items": [
            {
                "Slot": 0u8,
                "id": "minecraft:potato",
                "Count": 1
            },
            {
                "Slot": 1u8,
                "id": "minecraft:potato",
                "Count": 1
            },
            {
                "Slot": 2u8,
                "id": "minecraft:potato",
                "Count": 1
            },
            {
                "Slot": 3u8,
                "id": "minecraft:potato",
                "Count": 1
            }
        ]
    });

    let campfire: Campfire = fastnbt::from_value(&nbt).unwrap();

    assert_eq!(campfire.cooking_times, [100, 200, 300, 400]);
    assert_eq!(campfire.cookning_total_times, [200, 400, 600, 800]);
    assert_eq!(campfire.items.len(), 4);
    assert_eq!(campfire.items[0].id, "minecraft:potato");
    assert_eq!(campfire.items[0].count, 1);
    assert_eq!(campfire.items[1].id, "minecraft:potato");
    assert_eq!(campfire.items[1].count, 1);
    assert_eq!(campfire.items[2].id, "minecraft:potato");
    assert_eq!(campfire.items[2].count, 1);
    assert_eq!(campfire.items[3].id, "minecraft:potato");
    assert_eq!(campfire.items[3].count, 1);

    let serialized_nbt = fastnbt::to_value(&campfire).unwrap();

    assert_eq!(nbt, serialized_nbt);
}
