use std::borrow::Cow;

use simdnbt::Mutf8Str;

use crate::{kv::KVPair, traits::FromCompoundNbt};

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
