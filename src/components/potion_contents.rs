//! Potion contents component.

use crate::{
    color::RGB,
    traits::FromCompoundNbt,
    util::{get_owned_optional_string, get_t_compound_vec},
};

use super::food::EffectDetails;

/// The contents of a potion item.
#[derive(Debug, Clone, PartialEq)]
pub enum PotionContents {
    /// A resource location string representing the potion type.
    String(String),
    /// More detailed information about a potion item.
    Compound(PotionData),
}

/// More detailed information about a potion item.
#[derive(Debug, Clone, PartialEq)]
pub struct PotionData {
    /// The ID of a potion type. Optional. See [Potion#Item data](https://minecraft.wiki/w/Potion#Item_data).
    pub potion: Option<String>,

    /// The overriding color of this potion texture, and/or the particles of the area effect cloud created.
    pub custom_color: Option<RGB>,

    /// A list of the additional effects that this item should apply.
    pub custom_effects: Vec<EffectDetails>,
}

impl FromCompoundNbt for PotionContents {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        if let Some(string) = nbt.string("minecraft:potion_contents") {
            return Ok(PotionContents::String(string.to_string()));
        } else if let Some(compound) = nbt.compound("minecraft:potion_contents") {
            return Ok(PotionContents::Compound(PotionData::from_compound_nbt(
                &compound,
            )?));
        } else {
            return Err(crate::error::SculkParseError::MissingField(
                "minecraft:potion_contents".into(),
            ));
        }
    }
}

impl FromCompoundNbt for PotionData {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let potion = get_owned_optional_string(&nbt, "potion");

        let custom_color = nbt.int("custom_color").map(|c| RGB::new(c));

        let custom_effects =
            get_t_compound_vec(&nbt, "custom_effects", EffectDetails::from_compound_nbt)
                .unwrap_or_default();

        Ok(PotionData {
            potion,
            custom_color,
            custom_effects,
        })
    }
}
