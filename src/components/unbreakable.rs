//! Unbreakable component.

use crate::traits::FromCompoundNbt;

/// funny silly small struct
#[derive(Clone, Debug, PartialEq)]
pub struct Unbreakable {
    /// Show or hide the "Unbreakable" line on this item's tooltip. Defaults to true.  
    pub show_in_tooltip: bool,
}

impl FromCompoundNbt for Unbreakable {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        Ok(Unbreakable {
            show_in_tooltip: nbt.byte("show_in_tooltip").map(|b| b != 0).unwrap_or(true),
        })
    }
}
