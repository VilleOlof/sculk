use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::item::Item;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct BrewingStand<'a> {
    /// The number of ticks the potions have to brew.
    #[serde(rename = "BrewTime")]
    pub brew_time: i16,

    /// Optional. The name of this container in JSON text component, which appears in its GUI where the default name ordinarily appears.
    #[serde(borrow)]
    #[serde(rename = "CustomName")]
    pub custom_name: Option<Cow<'a, str>>,

    /// Remaining fuel for the brewing stand. 20 when full, and counts down by 1 each time a potion is brewed.
    #[serde(rename = "Fuel")]
    pub fuel: i8,

    /// List of items in this container.  
    ///
    /// Slot 0: Left potion slot.  
    /// Slot 1: Middle potion slot.  
    /// Slot 2: Right potion slot.  
    /// Slot 3: Where the potion ingredient goes.  
    /// Slot 4: Fuel (Blaze Powder).  
    #[serde(borrow)]
    #[serde(rename = "Items")]
    pub items: Vec<Item<'a>>,

    /// Optional. When not blank, prevents the container from being opened unless the opener is holding an item whose name matches this string.
    #[serde(borrow)]
    #[serde(rename = "Lock")]
    pub lock: Option<Cow<'a, str>>,
}
