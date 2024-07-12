use crate::{error::SculkParseError, kv::KVPair, traits::FromCompoundNbt};
use simdnbt::Mutf8Str;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq)]
pub enum CustomData<'a> {
    Snbt(Cow<'a, Mutf8Str>),
    KeyValues(KVPair<'a, Cow<'a, Mutf8Str>>),
}

impl<'a> FromCompoundNbt for CustomData<'a> {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        if let Some(string) = nbt.string("minecraft:custom_data") {
            let snbt = Cow::<'a, Mutf8Str>::Owned(string.to_owned());
            return Ok(CustomData::Snbt(snbt));
        } else if let Some(compound) = nbt.compound("minecraft:custom_data") {
            let map = KVPair::from_compound_nbt(&compound)?;
            return Ok(CustomData::KeyValues(map));
        } else {
            return Err(SculkParseError::MissingField(
                "minecraft:custom_data".into(),
            ));
        }
    }
}
