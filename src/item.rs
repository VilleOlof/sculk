use crate::{
    components::Components,
    traits::FromCompoundNbt,
    util::{get_optional_components, get_owned_string},
};

/// Represents an item in an inventory slot.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Item {
    /// The inventory slot the item is in.  
    /// `Slot`
    pub slot: i8,

    /// The resource location of the item. Must not be `air`.
    pub id: String,

    /// Number of items stacked in this inventory slot. Any item can be stacked, even if unstackable through normal means. Defaults to 1.  
    ///
    /// `Count`
    pub count: i32,

    /// Optional map of data components. Additional information about the item.
    pub components: Option<Components>,
}

/// Represents an item in an inventory slot, without the slot number.  
/// Often used for items in single-item inventories.  
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ItemWithNoSlot {
    /// The resource location of the item. Must not be `air`.
    pub id: String,

    /// Number of items stacked in this inventory slot. Any item can be stacked, even if unstackable through normal means. Defaults to 1.  
    /// Actual name: `Count`
    pub count: i32,

    /// Optional map of data components. Additional information about the item.
    pub components: Option<Components>,
}

impl FromCompoundNbt for Item {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let slot = nbt
            .byte("Slot")
            .ok_or(crate::error::SculkParseError::MissingField("Slot".into()))?;

        let id = get_owned_string(&nbt, "id")?;

        let count = nbt.int("Count").unwrap_or(1);

        let components = get_optional_components(&nbt)?;

        Ok(Item {
            slot,
            id,
            count,
            components,
        })
    }
}

impl FromCompoundNbt for ItemWithNoSlot {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let id = get_owned_string(&nbt, "id")?;

        let count = nbt.int("Count").unwrap_or(1);

        let components = get_optional_components(&nbt)?;

        Ok(ItemWithNoSlot {
            id,
            count,
            components,
        })
    }
}
