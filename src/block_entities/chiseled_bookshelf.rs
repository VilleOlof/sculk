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
