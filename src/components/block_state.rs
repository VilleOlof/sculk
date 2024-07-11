use std::{borrow::Cow, collections::HashMap};

use simdnbt::Mutf8Str;

use crate::{error::SculkParseError, traits::FromCompoundNbt, util::KeyValuePair};

#[derive(Debug, Clone, PartialEq)]
pub struct BlockState<'a>(KeyValuePair<'a>);

impl<'a> FromCompoundNbt for BlockState<'a> {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let mut map = HashMap::new();

        for (key, value) in nbt.iter() {
            let key = key.to_string();
            let value = value
                .string()
                .ok_or(SculkParseError::InvalidField(key.clone().into()))?;

            map.insert(
                Cow::<'a, String>::Owned(key),
                Cow::<'a, Mutf8Str>::Owned(value.to_owned()),
            );
        }

        Ok(BlockState(map))
    }
}
