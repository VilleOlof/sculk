use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::ComponentMap;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Item<'a> {
    /// The inventory slot the item is in.
    #[serde(rename = "Slot")]
    pub slot: u8,

    /// The resource location of the item. Must not be `air`.
    #[serde(borrow)]
    pub id: Cow<'a, str>,

    /// Number of items stacked in this inventory slot. Any item can be stacked, even if unstackable through normal means. Defaults to 1.
    #[serde(rename = "Count")]
    pub count: i32,

    /// Optional map of data components. Additional information about the item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<ComponentMap<'a>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ItemWithNoSlot<'a> {
    /// The resource location of the item. Must not be `air`.
    #[serde(borrow)]
    pub id: Cow<'a, str>,

    /// Number of items stacked in this inventory slot. Any item can be stacked, even if unstackable through normal means. Defaults to 1.
    #[serde(rename = "Count")]
    pub count: i32,

    /// Optional map of data components. Additional information about the item.
    pub components: Option<ComponentMap<'a>>,
}
