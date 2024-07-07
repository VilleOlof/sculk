use serde::{Deserialize, Serialize};
use std::{borrow::Cow, collections::HashMap};

use crate::item::Item;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Furnace<'a> {
    /// Number of ticks left before the current fuel runs out.
    #[serde(rename = "BurnTime")]
    pub burn_time: i16,

    /// Number of ticks the item has been smelting for. The item finishes smelting when this value reaches 200 (10 seconds). Is reset to 0 if BurnTime reaches 0.
    #[serde(rename = "CookTime")]
    pub cook_time: i16,

    /// Number of ticks It takes for the item to be smelted.
    #[serde(rename = "CookTimeTotal")]
    pub cook_time_total: i16,

    /// Optional. The name of this container in JSON text component, which appears in its GUI where the default name ordinarily appears.
    #[serde(rename = "CustomName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_name: Option<Cow<'a, str>>,

    /// List of items in this container.  
    ///
    /// Slot 0: The item(s) being smelted.  
    /// Slot 1: The item(s) to use as the next fuel source.  
    /// Slot 2: The item(s) in the result slot.  
    #[serde(borrow)]
    #[serde(rename = "Items")]
    pub items: Vec<Item<'a>>,

    /// Optional. When not blank, prevents the container from being opened unless the opener is holding an item whose name matches this string.
    #[serde(rename = "Lock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock: Option<Cow<'a, str>>,

    /// Recipes that have been used since the last time a recipe result item was manually removed from the GUI. Used to calculate experience given to the player when taking out the resulting item.
    ///
    /// Map entry: How many times this specific recipe has been used. The recipe ID is the identifier of the smelting recipe, as a resource location, as used in the /recipe command.
    #[serde(rename = "RecipesUsed")]
    pub recipes_used: HashMap<Cow<'a, str>, i32>,
}
