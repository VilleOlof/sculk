//! Written book content component.

use super::writable_book_content::{BookTextData, PageType};
use crate::{error::SculkParseError, traits::FromCompoundNbt, util::get_owned_string};

/// The content of a written book.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WrittenBookContent {
    /// A list of the pages in the book.
    pub pages: PageType,

    ///  The title of this written book.
    pub title: BookTextData,

    /// The author of this written book.
    pub author: String,

    /// The number of times this written book has been copied.  
    /// 0 = original, 1 = copy of original, 2 = copy of copy, 3 = tattered. Defaults to 0. If the value is greater than 1, the book cannot be copied.
    pub generation: i32,

    ///  If true, the JSON text components have already been resolved (such as entity selectors and scores). If false, they are resolved when this book is opened by a player for the first time. Defaults to false.
    #[cfg_attr(feature = "serde", serde(default = "bool::default"))]
    pub resolved: bool,
}

impl FromCompoundNbt for WrittenBookContent {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let pages = PageType::from_compound_nbt(&nbt)?;

        let tile = nbt
            .compound("title")
            .map(|nbt| BookTextData::from_compound_nbt(&nbt))
            .ok_or(SculkParseError::MissingField("title".into()))??;

        let author = get_owned_string(&nbt, "author")?;
        let generation = nbt.int("generation").unwrap_or(0);

        let resolved = nbt.byte("resolved").map(|b| b != 0).unwrap_or(false);

        Ok(WrittenBookContent {
            pages,
            title: tile,
            author,
            generation,
            resolved,
        })
    }
}
