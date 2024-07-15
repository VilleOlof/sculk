use crate::{item::ItemWithNoSlot, traits::FromCompoundNbt};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Lectern {
    /// The book item, without the slot tag, currently on the lectern, may not exist.
    ///
    /// `Book`
    pub book: Option<ItemWithNoSlot>,

    /// The page the book is currently on, starting from 0, does not exist if there's no book. Value is clamped between 0 and the last page - 1.
    ///
    /// `Page`
    pub page: Option<i32>,
}

impl FromCompoundNbt for Lectern {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        Ok(Lectern {
            book: nbt
                .compound("Book")
                .map(|b| ItemWithNoSlot::from_compound_nbt(&b))
                .transpose()?,
            page: nbt.int("Page"),
        })
    }
}
