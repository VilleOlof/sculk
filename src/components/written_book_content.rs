use super::writable_book_content::{BookTextData, PageType};
use crate::{error::SculkParseError, traits::FromCompoundNbt, util::get_owned_mutf8str};
use simdnbt::Mutf8Str;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq)]
pub struct WrittenBookContent<'a> {
    pub pages: PageType<'a>,

    pub title: BookTextData<'a>,

    pub author: Cow<'a, Mutf8Str>,

    pub generation: i32,

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
