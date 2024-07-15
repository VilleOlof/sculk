//! The effects of a suspicious stew.

use crate::{traits::FromCompoundNbt, util::get_owned_string};

/// The effects of a suspicious stew.
#[derive(Debug, Clone, PartialEq)]
pub struct SuspiciousStewEffects {
    /// The ID of the effect.
    pub id: String,

    /// The duration of the effect in ticks. Defaults to 160.
    pub duration: i32,
}

impl FromCompoundNbt for SuspiciousStewEffects {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let id = get_owned_string(&nbt, "id")?;
        let duration = nbt.int("duration").unwrap_or(160);

        Ok(SuspiciousStewEffects { id, duration })
    }
}
