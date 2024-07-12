use std::{borrow::Cow, collections::HashMap, ops::Deref};

use simdnbt::{borrow::NbtCompound, Mutf8Str};

use crate::{error::SculkParseError, traits::FromCompoundNbt};

#[derive(Debug, Clone, PartialEq)]
pub struct KVPair<'a, T>(HashMap<Cow<'a, String>, T>);

impl<'a, T> Deref for KVPair<'a, T> {
    type Target = HashMap<Cow<'a, String>, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T> KVPair<'a, T> {
    pub fn new(map: HashMap<Cow<'a, String>, T>) -> Self {
        KVPair(map)
    }
}

impl<'a> FromCompoundNbt for KVPair<'a, Cow<'a, Mutf8Str>> {
    fn from_compound_nbt(nbt: &NbtCompound) -> Result<Self, SculkParseError> {
        let mut map = HashMap::new();

        for (key, value) in nbt.iter() {
            let key = key.to_string();
            let value = match value.string() {
                Some(string) => string.to_owned(),
                None => continue,
            };

            map.insert(Cow::Owned(key), Cow::Owned(value));
        }

        Ok(KVPair(map))
    }
}

impl<'a> FromCompoundNbt for KVPair<'a, i32> {
    fn from_compound_nbt(nbt: &NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let mut map = HashMap::new();

        for (key, value) in nbt.iter() {
            let key = key.to_string();
            let value = match value.int() {
                Some(int) => int,
                None => continue,
            };

            map.insert(Cow::Owned(key), value);
        }

        Ok(KVPair(map))
    }
}
