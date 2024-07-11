use simdnbt::Mutf8Str;

use crate::{
    error::SculkParseError,
    item::Item,
    traits::FromCompoundNbt,
    util::{get_optional_lock, get_optional_name, get_t_compound_vec},
};
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq)]
pub struct BrewingStand<'a> {
    /// The number of ticks the potions have to brew.
    ///
    /// `BrewTime`
    pub brew_time: i16,

    /// Optional. The name of this container in JSON text component, which appears in its GUI where the default name ordinarily appears.
    ///
    /// `CustomName`
    pub custom_name: Option<Cow<'a, Mutf8Str>>,

    /// Remaining fuel for the brewing stand. 20 when full, and counts down by 1 each time a potion is brewed.
    ///
    /// `Fuel`
    pub fuel: i8,

    /// List of items in this container.  
    ///
    /// Slot 0: Left potion slot.  
    /// Slot 1: Middle potion slot.  
    /// Slot 2: Right potion slot.  
    /// Slot 3: Where the potion ingredient goes.  
    /// Slot 4: Fuel (Blaze Powder).  
    ///
    /// `Items`
    pub items: Vec<Item<'a>>,

    /// Optional. When not blank, prevents the container from being opened unless the opener is holding an item whose name matches this string.
    ///
    /// `Lock`
    pub lock: Option<Cow<'a, Mutf8Str>>,
}

impl<'a> FromCompoundNbt for BrewingStand<'a> {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let brew_time = nbt
            .short("BrewTime")
            .ok_or(SculkParseError::MissingField("BrewTime".into()))?;
        let custom_name = get_optional_name(&nbt);
        let fuel = nbt
            .byte("Fuel")
            .ok_or(SculkParseError::MissingField("Fuel".into()))?;
        let items = get_t_compound_vec(&nbt, "Items", Item::from_compound_nbt)?;
        let lock = get_optional_lock(&nbt);

        Ok(BrewingStand {
            brew_time,
            custom_name,
            fuel,
            items,
            lock,
        })
    }
}
