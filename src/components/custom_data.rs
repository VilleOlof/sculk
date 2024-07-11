use crate::{error::SculkParseError, traits::FromCompoundNbt, util::KeyValuePair};
use simdnbt::Mutf8Str;
use std::{borrow::Cow, collections::HashMap};

#[derive(Debug, Clone, PartialEq)]
pub enum CustomData<'a> {
    Snbt(Cow<'a, Mutf8Str>),
    KeyValues(KeyValuePair<'a>),
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
            //
            let mut map = HashMap::new();

            for (key, value) in compound.iter() {
                let key = key.to_string();
                let value = value
                    .string()
                    .ok_or(SculkParseError::InvalidField(key.clone().into()))?;

                map.insert(
                    Cow::<'a, String>::Owned(key),
                    Cow::<'a, Mutf8Str>::Owned(value.to_owned()),
                );
            }

            return Ok(CustomData::KeyValues(map));
        } else {
            return Err(SculkParseError::MissingField(
                "minecraft:custom_data".into(),
            ));
        }
    }
}
