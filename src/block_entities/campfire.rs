use crate::{
    item::Item,
    traits::FromCompoundNbt,
    util::{get_int_array, get_t_compound_vec},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Campfire {
    /// How long each item has been cooking, first index is slot 0, etc.
    ///
    /// `CookingTimes`
    pub cooking_times: Vec<i32>,

    /// How long each item has to cook, first index is slot 0, etc.
    ///
    /// `CookingTotalTimes`
    pub cooking_total_times: Vec<i32>,

    /// List of up to 4 items currently cooking.
    ///
    /// `Items`
    pub items: Vec<Item>,
}

impl FromCompoundNbt for Campfire {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let cooking_times = get_int_array(&nbt, "CookingTimes")?;
        let cooking_total_times = get_int_array(&nbt, "CookingTotalTimes")?;

        let items = get_t_compound_vec(&nbt, "Items", Item::from_compound_nbt)?;

        Ok(Campfire {
            cooking_times,
            cooking_total_times,
            items,
        })
    }
}
