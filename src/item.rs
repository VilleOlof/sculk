use simdnbt::Mutf8Str;

use crate::{
    error::SculkParseError,
    traits::FromCompoundNbt,
    util::{get_optional_components, get_owned_mutf8str},
    Components,
};
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq)]
pub struct Item<'a> {
    /// The inventory slot the item is in.  
    /// `Slot`
    pub slot: i8,

    /// The resource location of the item. Must not be `air`.
    pub id: Cow<'a, Mutf8Str>,

    /// Number of items stacked in this inventory slot. Any item can be stacked, even if unstackable through normal means. Defaults to 1.  
    ///
    /// `Count`
    pub count: i32,

    /// Optional map of data components. Additional information about the item.
    pub components: Option<Components<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ItemWithNoSlot<'a> {
    /// The resource location of the item. Must not be `air`.
    pub id: Cow<'a, Mutf8Str>,

    /// Number of items stacked in this inventory slot. Any item can be stacked, even if unstackable through normal means. Defaults to 1.  
    /// Actual name: `Count`
    pub count: i32,

    /// Optional map of data components. Additional information about the item.
    pub components: Option<Components<'a>>,
}

impl<'a> FromCompoundNbt for Item<'a> {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let slot = nbt
            .byte("Slot")
            .ok_or(crate::error::SculkParseError::MissingField("Slot".into()))?;

        let id = get_owned_mutf8str(&nbt, "id")?;

        let count = nbt
            .int("Count")
            .ok_or(crate::error::SculkParseError::MissingField("Count".into()))?;

        let components = get_optional_components(&nbt)?;

        Ok(Item {
            slot,
            id,
            count,
            components,
        })
    }
}

impl<'a> FromCompoundNbt for ItemWithNoSlot<'a> {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let id = get_owned_mutf8str(&nbt, "id")?;

        let count = nbt
            .int("Count")
            .ok_or(crate::error::SculkParseError::MissingField("Count".into()))?;

        let components = get_optional_components(&nbt)?;

        Ok(ItemWithNoSlot {
            id,
            count,
            components,
        })
    }
}
