use crate::{error::SculkParseError, traits::FromCompoundNbt};
use simdnbt::borrow::NbtCompound;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct KVPair<T>(HashMap<String, T>);

impl<T> KVPair<T> {
    pub fn new(map: HashMap<String, T>) -> Self {
        KVPair(map)
    }
}

impl FromCompoundNbt for KVPair<String> {
    fn from_compound_nbt(nbt: &NbtCompound) -> Result<Self, SculkParseError> {
        let mut map = HashMap::new();

        for (key, value) in nbt.iter() {
            let key = key.to_string();
            let value = match value.string() {
                Some(string) => string.to_string(),
                None => continue,
            };

            map.insert(key, value);
        }

        Ok(KVPair::new(map))
    }
}

impl FromCompoundNbt for KVPair<i32> {
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

            map.insert(key, value);
        }

        Ok(KVPair::new(map))
    }
}

impl FromCompoundNbt for KVPair<simdnbt::owned::NbtCompound> {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let mut map = HashMap::new();

        for (key, value) in nbt.iter() {
            let key = key.to_string();
            let value = match value.compound() {
                Some(compound) => compound.to_owned(),
                None => continue,
            };

            map.insert(key, value);
        }

        Ok(KVPair::new(map))
    }
}
