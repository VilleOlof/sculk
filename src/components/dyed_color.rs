//! Dyed color component.

use crate::{color::RGB, error::SculkParseError, traits::FromCompoundNbt};

/// A dyed color component.  
/// Used on stuff like leather armor.  
#[derive(Debug, Clone, PartialEq)]
pub enum DyedColor {
    /// A simple RGB color.
    Int(RGB),
    /// Also if the color should be shown in the tooltip.
    Compound {
        /// The RGB color.
        rgb: RGB,
        /// If the color should be shown in the tooltip.
        show_in_tooltip: bool,
    },
}

impl FromCompoundNbt for DyedColor {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        if let Some(rgb) = nbt.int("minecraft:dyed_color") {
            return Ok(DyedColor::Int(RGB::new(rgb)));
        } else if let Some(compound) = nbt.compound("minecraft:dyed_color") {
            let rgb = compound
                .int("rgb")
                .map(|rgb| RGB::new(rgb))
                .ok_or(SculkParseError::MissingField("rgb".into()))?;

            let show_in_tooltip = compound
                .byte("show_in_tooltip")
                .map(|b| b != 0)
                .unwrap_or(true);

            return Ok(DyedColor::Compound {
                rgb,
                show_in_tooltip,
            });
        } else {
            return Err(SculkParseError::MissingField("minecraft:dyed_color".into()));
        }
    }
}
