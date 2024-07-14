//! Written book content component.

use super::writable_book_content::{BookTextData, PageType};
use crate::{error::SculkParseError, traits::FromCompoundNbt, util::get_owned_mutf8str};
use simdnbt::Mutf8Str;
use std::borrow::Cow;

/// The content of a written book.
#[derive(Debug, Clone, PartialEq)]
pub struct WrittenBookContent<'a> {
    /// A list of the pages in the book.
    pub pages: PageType<'a>,

    ///  The title of this written book.
    pub title: BookTextData<'a>,

    /// The author of this written book.
    pub author: Cow<'a, Mutf8Str>,

    /// The number of times this written book has been copied.  
    /// 0 = original, 1 = copy of original, 2 = copy of copy, 3 = tattered. Defaults to 0. If the value is greater than 1, the book cannot be copied.
    pub generation: i32,

    ///  If true, the JSON text components have already been resolved (such as entity selectors and scores). If false, they are resolved when this book is opened by a player for the first time. Defaults to false.
    pub resolved: bool,
}

impl<'a> FromCompoundNbt for WrittenBookContent<'a> {
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

        let author = get_owned_mutf8str(&nbt, "author")?;
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
