//! Jukebox Playable component.

use crate::{traits::FromCompoundNbt, util::get_owned_string};

/// A Jukebox Playable component.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct JukeboxPlayable {
    /// The song to play.
    pub song: String,

    /// If `true`, the song is shown in the tooltip of the item. Defaults to `true`.
    pub show_in_tooltip: bool,
}

impl FromCompoundNbt for JukeboxPlayable {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let song = get_owned_string(&nbt, "song")?;

        let show_in_tooltip = nbt.byte("show_in_tooltip").map(|b| b != 0).unwrap_or(true);

        Ok(JukeboxPlayable {
            song,
            show_in_tooltip,
        })
    }
}
