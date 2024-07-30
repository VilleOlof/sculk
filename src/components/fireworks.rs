//! Firework rocket item component.

use super::firework_explosion::FireworkExplosion;
use crate::{error::SculkParseError, traits::FromCompoundNbt, util::get_t_list};

#[cfg(feature = "serde")]
fn default_flight_duration() -> i8 {
    1
}

/// A firework rocket item.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Fireworks {
    /// List of the explosion effects caused by this [firework rocket](https://minecraft.wiki/w/Firework_rocket). Has a maximum of 256 explosions.  
    pub explosions: Vec<FireworkExplosion>,

    /// The flight duration of this firework rocket, i.e. the number of gunpowders used to craft it. Must be an integer between -128 and 127. Defaults to 1.
    #[cfg_attr(feature = "serde", serde(default = "default_flight_duration"))]
    pub flight_duration: i8,
}

impl FromCompoundNbt for Fireworks {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let explosions = get_t_list(
            &nbt.list("explosions")
                .ok_or(SculkParseError::MissingField("explosions".into()))?,
            "explosions",
            FireworkExplosion::from_compound_nbt,
        )?;

        let flight_duration = nbt.byte("Flight").unwrap_or(1);

        Ok(Fireworks {
            explosions,
            flight_duration,
        })
    }
}
