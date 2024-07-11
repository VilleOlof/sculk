use crate::{error::SculkParseError, traits::FromCompoundNbt, Components};
use simdnbt::{borrow::NbtCompound, Mutf8Str};
use std::{borrow::Cow, collections::HashMap};

/// The version of Minecraft that this library is designed to work with.  
/// Formatted exactly as minecraft versions are.
pub const MC_VERSION: &str = "1.21";

pub type KeyValuePair<'a> = HashMap<Cow<'a, String>, Cow<'a, Mutf8Str>>;

pub struct LootTableData<'a> {
    pub loot_table: Option<Cow<'a, Mutf8Str>>,
    pub loot_table_seed: Option<i64>,
}

pub fn get_loot_table_data<'a>(nbt: &NbtCompound) -> LootTableData<'a> {
    let loot_table = get_owned_optional_mutf8str(&nbt, "LootTable");

    let loot_table_seed = nbt.long("LootTableSeed");

    LootTableData {
        loot_table,
        loot_table_seed,
    }
}

pub fn get_owned_mutf8str<'a>(
    nbt: &NbtCompound,
    key: &'static str,
) -> Result<Cow<'a, Mutf8Str>, SculkParseError> {
    Ok(nbt
        .string(key)
        .map(|s| Cow::Owned(s.to_owned()))
        .ok_or(SculkParseError::InvalidField(key.into()))?)
}

pub fn get_owned_optional_mutf8str<'a>(
    nbt: &NbtCompound,
    key: &'static str,
) -> Option<Cow<'a, Mutf8Str>> {
    nbt.string(key).map(|s| Cow::Owned(s.to_owned()))
}

pub fn get_optional_lock<'a>(nbt: &NbtCompound) -> Option<Cow<'a, Mutf8Str>> {
    nbt.string("Lock").map(|s| Cow::Owned(s.to_owned()))
}

pub fn get_optional_name<'a>(nbt: &NbtCompound) -> Option<Cow<'a, Mutf8Str>> {
    nbt.string("CustomName").map(|s| Cow::Owned(s.to_owned()))
}

pub fn get_bool(nbt: &NbtCompound, key: &'static str) -> bool {
    nbt.byte(key).map(|b| b != 0).unwrap_or(false)
}

pub fn get_int_array(nbt: &NbtCompound, key: &'static str) -> Result<Vec<i32>, SculkParseError> {
    let list = nbt
        .list(key)
        .ok_or(SculkParseError::MissingField(key.into()))?;

    let mut arr = vec![];

    for item in list
        .ints()
        .ok_or(SculkParseError::InvalidField(key.into()))?
        .iter()
    {
        arr.push(*item);
    }

    Ok(arr)
}

pub fn get_doubles_array(
    nbt: &NbtCompound,
    key: &'static str,
) -> Result<Vec<f64>, SculkParseError> {
    let list = nbt
        .list(key)
        .ok_or(SculkParseError::MissingField(key.into()))?;

    let mut arr = vec![];

    for item in list
        .doubles()
        .ok_or(SculkParseError::InvalidField(key.into()))?
        .iter()
    {
        arr.push(*item);
    }

    Ok(arr)
}

pub fn get_t_compound_vec<T>(
    nbt: &NbtCompound,
    key: &'static str,
    nbt_conversion: fn(nbt: &NbtCompound) -> Result<T, SculkParseError>,
) -> Result<Vec<T>, SculkParseError> {
    let list = nbt
        .list(key)
        .ok_or(SculkParseError::MissingField(key.into()))?;

    let mut vec = vec![];

    for item in list
        .compounds()
        .ok_or(SculkParseError::InvalidField(key.into()))?
    {
        vec.push(nbt_conversion(&item)?);
    }

    Ok(vec)
}

pub fn get_optional_components<'a>(
    nbt: &NbtCompound,
) -> Result<Option<Components<'a>>, SculkParseError> {
    match Components::from_compound_nbt(&nbt) {
        Ok(components) => Ok(Some(components)),
        // Only return None if the field is missing
        Err(SculkParseError::MissingField(_)) => Ok(None),
        // Return the error if it's anything else
        Err(e) => return Err(e),
    }
}
