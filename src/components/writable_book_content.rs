//! A component that represents the content of a writable book.

use crate::{
    error::SculkParseError,
    traits::FromCompoundNbt,
    util::{get_owned_optional_string, get_owned_string},
};

/// A book that can be written in-game.
#[derive(Debug, Clone, PartialEq)]
pub struct WritableBookContent {
    /// A list of the pages in the book.
    pub pages: PageType,
}

/// A page in a book.
#[derive(Debug, Clone, PartialEq)]
pub enum PageType {
    /// Alternatively, a single page can be represented as follows:
    /// The plain text content of a page.
    Single(String),

    /// A list of pages.
    Multiple(Vec<BookTextData>),
}

/// The content of a page in a book.
#[derive(Debug, Clone, PartialEq)]
pub struct BookTextData {
    /// The plain text content of the page. The escape sequence \n is used for line breaks. However, the command parser does not accept \n, so line breaks need to be set by a player or using loot tables.  
    pub raw: String,

    /// The filtered text of the page. Optional. Shown only to players with chat filter enabled, instead of text.
    pub filtered: Option<String>,
}

impl FromCompoundNbt for WritableBookContent {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let pages = PageType::from_compound_nbt(nbt)?;

        Ok(WritableBookContent { pages })
    }
}

impl FromCompoundNbt for PageType {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        if let Some(single) = nbt.string("pages") {
            return Ok(PageType::Single(single.to_string()));
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

impl FromCompoundNbt for BookTextData {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let raw = get_owned_string(&nbt, "raw")?;
        let filtered = get_owned_optional_string(&nbt, "filtered");

        Ok(BookTextData { raw, filtered })
    }
}
