use serde::{Deserialize, Serialize};

use crate::item::Item;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ChiseledBookshelf<'a> {
    /// List of books in the bookshelf.  
    ///
    /// The valid slot numbers are 0-5.
    #[serde(borrow)]
    #[serde(rename = "Items")]
    pub items: Vec<Item<'a>>,

    /// Last interacted slot (0–5), or -1 if no slot has been interacted with yet.
    pub last_interacted_slot: i32,
}

#[cfg(test)]
#[test]
fn test() {
    use fastnbt::nbt;

    let nbt = nbt!({
        "Items": [
            {
                "Slot": 0u8,
                "id": "minecraft:book",
                "Count": 1
            },
            {
                "Slot": 1u8,
                "id": "minecraft:book",
                "Count": 1
            },
            {
                "Slot": 2u8,
                "id": "minecraft:book",
                "Count": 1
            },
            {
                "Slot": 3u8,
                "id": "minecraft:book",
                "Count": 1
            },
            {
                "Slot": 4u8,
                "id": "minecraft:book",
                "Count": 1
            },
        ],
        "last_interacted_slot": 3i32
    });

    let chiseled_bookshelf: ChiseledBookshelf = fastnbt::from_value(&nbt).unwrap();

    assert_eq!(chiseled_bookshelf.items.len(), 5);
    assert_eq!(chiseled_bookshelf.items[0].id, "minecraft:book");
    assert_eq!(chiseled_bookshelf.items[0].count, 1);
    assert_eq!(chiseled_bookshelf.last_interacted_slot, 3);

    let serialized_nbt = fastnbt::to_value(&chiseled_bookshelf).unwrap();

    assert_eq!(nbt, serialized_nbt);
}
