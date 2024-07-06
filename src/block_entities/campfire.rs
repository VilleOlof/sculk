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
    pub items: Vec<Item<'a>>,
}
