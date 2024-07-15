//! Contains the `Container` component.

use crate::{item::ItemWithNoSlot, traits::FromCompoundNbt};

/// The items contained in this container.
#[derive(Debug, Clone, PartialEq)]
pub struct Container {
    /// A single item.
    pub item: ItemWithNoSlot,

    ///  A slot in this container. Can be between 0 and 255 (inclusive).
    pub slot: i32,
}

impl FromCompoundNbt for Container {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let item = ItemWithNoSlot::from_compound_nbt(&nbt)?;
        let slot = nbt
            .int("slot")
            .ok_or(crate::error::SculkParseError::MissingField("slot".into()))?;

        Ok(Container { item, slot })
    }
}
