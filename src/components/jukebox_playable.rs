//! Jukebox Playable component.

use crate::{traits::FromCompoundNbt, util::get_owned_mutf8str};
use simdnbt::Mutf8Str;
use std::borrow::Cow;

/// A Jukebox Playable component.
#[derive(Debug, Clone, PartialEq)]
pub struct JukeboxPlayable<'a> {
    /// The song to play.
    pub song: Cow<'a, Mutf8Str>,

    /// If `true`, the song is shown in the tooltip of the item. Defaults to `true`.
    pub show_in_tooltip: bool,
}

impl<'a> FromCompoundNbt for JukeboxPlayable<'a> {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let song = get_owned_mutf8str(&nbt, "song")?;

        let show_in_tooltip = nbt.byte("show_in_tooltip").map(|b| b != 0).unwrap_or(true);

        Ok(JukeboxPlayable {
            song,
            show_in_tooltip,
        })
    }
}
