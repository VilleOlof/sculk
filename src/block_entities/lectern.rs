use crate::{item::ItemWithNoSlot, traits::FromCompoundNbt};

#[derive(Debug, Clone, PartialEq)]
pub struct Lectern<'a> {
    /// The book item, without the slot tag, currently on the lectern, may not exist.
    ///
    /// `Book`
    pub book: Option<ItemWithNoSlot<'a>>,

    /// The page the book is currently on, starting from 0, does not exist if there's no book. Value is clamped between 0 and the last page - 1.
    ///
    /// `Page`
    pub page: Option<i32>,
}

impl<'a> FromCompoundNbt for Lectern<'a> {
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
