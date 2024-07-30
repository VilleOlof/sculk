//! The effects of a suspicious stew.

use crate::{traits::FromCompoundNbt, util::get_owned_string};

#[cfg(feature = "serde")]
fn default_duration() -> i32 {
    160
}

/// The effects of a suspicious stew.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SuspiciousStewEffects {
    /// The ID of the effect.
    pub id: String,

    /// The duration of the effect in ticks. Defaults to 160.
    #[cfg_attr(feature = "serde", serde(default = "default_duration"))]
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
