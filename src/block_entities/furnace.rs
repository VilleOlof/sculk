use crate::{
    error::SculkParseError,
    item::Item,
    traits::FromCompoundNbt,
    util::{get_optional_lock, get_optional_name, get_t_compound_vec},
};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Furnace {
    /// Number of ticks left before the current fuel runs out.
    ///
    /// `BurnTime`
    pub burn_time: i16,

    /// Number of ticks the item has been smelting for. The item finishes smelting when this value reaches 200 (10 seconds). Is reset to 0 if BurnTime reaches 0.
    ///
    /// `CookTime`
    pub cook_time: i16,

    /// Number of ticks It takes for the item to be smelted.
    ///
    /// `CookTimeTotal`
    pub cook_time_total: i16,

    /// Optional. The name of this container in JSON text component, which appears in its GUI where the default name ordinarily appears.
    ///
    /// `CustomName`
    pub custom_name: Option<String>,

    /// List of items in this container.  
    ///
    /// Slot 0: The item(s) being smelted.  
    /// Slot 1: The item(s) to use as the next fuel source.  
    /// Slot 2: The item(s) in the result slot.  
    ///
    /// `Items`
    pub items: Vec<Item>,

    /// Optional. When not blank, prevents the container from being opened unless the opener is holding an item whose name matches this string.
    ///
    /// `Lock`
    pub lock: Option<String>,

    /// Recipes that have been used since the last time a recipe result item was manually removed from the GUI. Used to calculate experience given to the player when taking out the resulting item.
    ///
    /// Map entry: How many times this specific recipe has been used. The recipe ID is the identifier of the smelting recipe, as a resource location, as used in the /recipe command.  
    ///
    /// `RecipesUsed`
    pub recipes_used: HashMap<String, i32>,
}

impl FromCompoundNbt for Furnace {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let burn_time = nbt
            .short("BurnTime")
            .ok_or(SculkParseError::MissingField("BurnTime".into()))?;
        let cook_time = nbt
            .short("CookTime")
            .ok_or(SculkParseError::MissingField("CookTime".into()))?;
        let cook_time_total = nbt
            .short("CookTimeTotal")
            .ok_or(SculkParseError::MissingField("CookTimeTotal".into()))?;

        let custom_name = get_optional_name(&nbt);
        let items = get_t_compound_vec(&nbt, "Items", Item::from_compound_nbt)?;
        let lock = get_optional_lock(&nbt);

        let recipes_used = nbt
            .compound("RecipesUsed")
            .ok_or(SculkParseError::MissingField("RecipesUsed".into()))?
            .iter()
            .map(|(key, value)| {
                let key: String = key.to_string();
                let value = value
                    .int()
                    .ok_or(SculkParseError::InvalidField("RecipesUsed".into()))?;
                Ok((key, value))
            })
            .collect::<Result<HashMap<String, i32>, SculkParseError>>()?;

        Ok(Furnace {
            burn_time,
            cook_time,
            cook_time_total,
            custom_name,
            items,
            lock,
            recipes_used,
        })
    }
}
