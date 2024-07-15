//! Block state component.

use crate::{kv::KVPair, traits::FromCompoundNbt};

/// Represents multiple key-value pairs of block states.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BlockState(KVPair<String>);

impl FromCompoundNbt for BlockState {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        Ok(BlockState(KVPair::from_compound_nbt(&nbt)?))
    }
}
