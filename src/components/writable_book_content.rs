//! A component that represents the content of a writable book.

use crate::{
    error::SculkParseError,
    traits::FromCompoundNbt,
    util::{get_owned_mutf8str, get_owned_optional_mutf8str},
};
use simdnbt::Mutf8Str;
use std::borrow::Cow;

/// A book that can be written in-game.
#[derive(Debug, Clone, PartialEq)]
pub struct WritableBookContent<'a> {
    /// A list of the pages in the book.
    pub pages: PageType<'a>,
}

/// A page in a book.
#[derive(Debug, Clone, PartialEq)]
pub enum PageType<'a> {
    /// Alternatively, a single page can be represented as follows:
    /// The plain text content of a page.
    Single(Cow<'a, Mutf8Str>),

    /// A list of pages.
    Multiple(Vec<BookTextData<'a>>),
}

/// The content of a page in a book.
#[derive(Debug, Clone, PartialEq)]
pub struct BookTextData<'a> {
    /// The plain text content of the page. The escape sequence \n is used for line breaks. However, the command parser does not accept \n, so line breaks need to be set by a player or using loot tables.  
    pub raw: Cow<'a, Mutf8Str>,

    /// The filtered text of the page. Optional. Shown only to players with chat filter enabled, instead of text.
    pub filtered: Option<Cow<'a, Mutf8Str>>,
}

impl<'a> FromCompoundNbt for WritableBookContent<'a> {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let pages = PageType::from_compound_nbt(nbt)?;

        Ok(WritableBookContent { pages })
    }
}

impl<'a> FromCompoundNbt for PageType<'a> {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        if let Some(single) = nbt.string("pages") {
            return Ok(PageType::Single(Cow::Owned(single.to_owned())));
        } else if let Some(multiple) = nbt.list("pages") {
            let mut pages = Vec::new();

            for page in multiple
                .compounds()
                .ok_or(SculkParseError::InvalidField("pages".into()))?
            {
                pages.push(BookTextData::from_compound_nbt(&page)?);
            }

            return Ok(PageType::Multiple(pages));
        } else {
            return Err(SculkParseError::MissingField("pages".into()));
        }
    }
}

impl<'a> FromCompoundNbt for BookTextData<'a> {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let raw = get_owned_mutf8str(&nbt, "raw")?;
        let filtered = get_owned_optional_mutf8str(&nbt, "filtered");

        Ok(BookTextData { raw, filtered })
    }
}
