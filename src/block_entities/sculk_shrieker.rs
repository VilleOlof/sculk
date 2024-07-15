use crate::{error::SculkParseError, traits::FromCompoundNbt};

use super::calibrated_sculk_sensor::Listener;

#[derive(Debug, Clone, PartialEq)]
pub struct SculkShrieker {
    /// The vibration event listener of the sculk shrieker, sculk sensor, and calibrated sculk sensor.
    // TODO: This one is hella sketch, wiki just says a bunch of `Unknown` and it seems old, gonna assume its like the other sculk blocks.
    pub listener: Listener,
}

impl FromCompoundNbt for SculkShrieker {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let listener = nbt
            .compound("listener")
            .map(|nbt| Listener::from_compound_nbt(&nbt))
            .ok_or(SculkParseError::MissingField("listener".into()))??;

        Ok(SculkShrieker { listener })
    }
}
