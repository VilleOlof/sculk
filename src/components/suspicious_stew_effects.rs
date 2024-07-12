use simdnbt::Mutf8Str;
use std::borrow::Cow;

use crate::{traits::FromCompoundNbt, util::get_owned_mutf8str};

#[derive(Debug, Clone, PartialEq)]
pub struct SuspiciousStewEffects<'a> {
    /// The ID of the effect.
    pub id: Cow<'a, Mutf8Str>,

    /// The duration of the effect in ticks. Defaults to 160.
    pub duration: i32,
}

impl<'a> FromCompoundNbt for SuspiciousStewEffects<'a> {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let id = get_owned_mutf8str(&nbt, "id")?;
        let duration = nbt.int("duration").unwrap_or(160);

        Ok(SuspiciousStewEffects { id, duration })
    }
}
