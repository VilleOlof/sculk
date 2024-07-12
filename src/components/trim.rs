use simdnbt::Mutf8Str;
use std::borrow::Cow;

use crate::{traits::FromCompoundNbt, util::get_owned_mutf8str};

#[derive(Debug, Clone, PartialEq)]
pub struct Trim<'a> {
    /// The ID of the trim pattern.
    pub pattern: Cow<'a, Mutf8Str>,

    /// The ID of the trim material, which applies a color to the trim.
    pub material: Cow<'a, Mutf8Str>,

    /// Show or hide the trim on this item's tooltip. Defaults to true.
    pub show_in_tooltip: bool,
}

impl<'a> FromCompoundNbt for Trim<'a> {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let pattern = get_owned_mutf8str(&nbt, "pattern")?;
        let material = get_owned_mutf8str(&nbt, "material")?;

        let show_in_tooltip = nbt.byte("show_in_tooltip").map(|b| b != 0).unwrap_or(true);

        Ok(Trim {
            pattern,
            material,
            show_in_tooltip,
        })
    }
}
