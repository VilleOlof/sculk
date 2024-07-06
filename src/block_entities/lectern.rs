use serde::{Deserialize, Serialize};

use crate::item::ItemWithNoSlot;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Lectern<'a> {
    /// The book item, without the slot tag, currently on the lectern, may not exist.
    #[serde(borrow)]
    #[serde(rename = "Book")]
    pub book: Option<ItemWithNoSlot<'a>>,

    /// The page the book is currently on, starting from 0, does not exist if there's no book. Value is clamped between 0 and the last page - 1.
    #[serde(rename = "Page")]
    pub page: i32,
}
