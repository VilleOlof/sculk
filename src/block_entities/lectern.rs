use serde::{Deserialize, Serialize};

use crate::item::ItemWithNoSlot;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Lectern<'a> {
    /// The book item, without the slot tag, currently on the lectern, may not exist.
    #[serde(borrow)]
    #[serde(rename = "Book")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub book: Option<ItemWithNoSlot<'a>>,

    /// The page the book is currently on, starting from 0, does not exist if there's no book. Value is clamped between 0 and the last page - 1.
    #[serde(rename = "Page")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
}

#[cfg(test)]
#[test]
fn test() {
    use fastnbt::nbt;

    let nbt = nbt!({
        "Book": {
            "id": "minecraft:written_book",
            "Count": 1
        },
        "Page": 0
    });

    let lectern: Lectern = fastnbt::from_value(&nbt).unwrap();

    let book = lectern.book.as_ref().unwrap();

    assert_eq!(book.id, "minecraft:written_book");
    assert_eq!(book.count, 1);
    assert_eq!(lectern.page.unwrap(), 0);

    let serialized_nbt = fastnbt::to_value(&lectern).unwrap();

    assert_eq!(nbt, serialized_nbt);
}
