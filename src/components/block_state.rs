//! Block state component.

use crate::{kv::KVPair, traits::FromCompoundNbt};
use simdnbt::Mutf8Str;
use std::borrow::Cow;

/// Represents multiple key-value pairs of block states.
#[derive(Debug, Clone, PartialEq)]
pub struct BlockState<'a>(KVPair<'a, Cow<'a, Mutf8Str>>);

impl<'a> FromCompoundNbt for BlockState<'a> {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        Ok(BlockState(KVPair::from_compound_nbt(&nbt)?))
    }
}
