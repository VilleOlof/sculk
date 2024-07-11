use crate::{
    error::SculkParseError,
    item::Item,
    traits::FromCompoundNbt,
    util::{get_int_array, get_t_compound_vec},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Campfire<'a> {
    /// How long each item has been cooking, first index is slot 0, etc.
    ///
    /// `CookingTimes`
    pub cooking_times: [i32; 4],

    /// How long each item has to cook, first index is slot 0, etc.
    ///
    /// `CookingTotalTimes`
    pub cooking_total_times: [i32; 4],

    /// List of up to 4 items currently cooking.
    ///
    /// `Items`
    pub items: Vec<Item<'a>>,
}

impl<'a> FromCompoundNbt for Campfire<'a> {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let cooking_times = get_int_array(&nbt, "cooking_times").and_then(|arr| {
            if arr.len() == 4 {
                Ok([arr[0], arr[1], arr[2], arr[3]])
            } else {
                Err(SculkParseError::InvalidField("cooking_times".into()))
            }
        })?;

        let cooking_total_times = get_int_array(&nbt, "cooking_total_times").and_then(|arr| {
            if arr.len() == 4 {
                Ok([arr[0], arr[1], arr[2], arr[3]])
            } else {
                Err(SculkParseError::InvalidField("cooking_total_times".into()))
            }
        })?;

        let items = get_t_compound_vec(&nbt, "Items", Item::from_compound_nbt)?;

        Ok(Campfire {
            cooking_times,
            cooking_total_times,
            items,
        })
    }
}
