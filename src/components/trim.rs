//! Armor Trim component.

use crate::{traits::FromCompoundNbt, util::get_owned_string};

/// An armor trim pattern.
#[derive(Debug, Clone, PartialEq)]
pub struct Trim {
    /// The ID of the trim pattern.
    pub pattern: String,

    /// The ID of the trim material, which applies a color to the trim.
    pub material: String,

    /// Show or hide the trim on this item's tooltip. Defaults to true.
    pub show_in_tooltip: bool,
}

impl FromCompoundNbt for Trim {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let pattern = get_owned_string(&nbt, "pattern")?;
        let material = get_owned_string(&nbt, "material")?;

        let show_in_tooltip = nbt.byte("show_in_tooltip").map(|b| b != 0).unwrap_or(true);

        Ok(Trim {
            pattern,
            material,
            show_in_tooltip,
        })
    }
}
