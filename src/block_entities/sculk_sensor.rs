use crate::{error::SculkParseError, traits::FromCompoundNbt};

use super::calibrated_sculk_sensor::Listener;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SculkSensor {
    /// The frequency of the last vibration.
    pub last_vibration_frequency: i32,

    /// The vibration event listener for this sculk shrieker, sculk sensor, or calibrated sculk sensor.
    pub listener: Listener,
}

impl FromCompoundNbt for SculkSensor {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let last_vibration_frequency =
            nbt.int("last_vibration_frequency")
                .ok_or(SculkParseError::MissingField(
                    "last_vibration_frequency".into(),
                ))?;

        let listener = nbt
            .compound("listener")
            .map(|nbt| Listener::from_compound_nbt(&nbt))
            .ok_or(SculkParseError::MissingField("listener".into()))??;

        Ok(SculkSensor {
            last_vibration_frequency,
            listener,
        })
    }
}
