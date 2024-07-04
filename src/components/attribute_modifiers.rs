use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::util::default_as_true;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct AttributeModifiers<'a> {
    /// Show or hide attribute modifiers on this item's tooltip. Defaults to `true`.
    #[serde(default = "default_as_true")]
    pub show_in_tooltip: bool,

    /// Contains attribute modifiers on this item which modify attributes of the wearer or holder (if the item is not in the hand or armor slots, it has no effect).
    pub modifiers: Vec<Modifier<'a>>,
}

/// A single attribute modifier.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Modifier<'a> {
    /// The name of the attribute this modifier is to act upon.
    #[serde(rename = "type")]
    pub _type: Cow<'a, str>,

    /// Slot or slot type the item must be in for the modifier to take effect.
    pub slot: SlotType,

    /// The unique resource location to identify this modifier.
    pub id: Cow<'a, str>,

    /// Amount of change from the modifier.
    pub amount: f64,

    /// Modifier operation.
    pub operation: Operation,
}

/// Slot or slot type the item must be in for the modifier to take effect.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SlotType {
    Any,
    Hand,
    Armor,
    MainHand,
    OffHand,
    Head,
    Chest,
    Legs,
    Feet,
    Body,
}

/// Modifier operation.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum Operation {
    #[serde(rename = "add_value")]
    Add,
    #[serde(rename = "add_multiplied_base")]
    MultiplyBase,
    #[serde(rename = "add_multiplied_total")]
    MultiplyTotal,
}
