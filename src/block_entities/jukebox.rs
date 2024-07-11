use crate::{error::SculkParseError, item::ItemWithNoSlot, traits::FromCompoundNbt};

#[derive(Debug, Clone, PartialEq)]
pub struct Jukebox<'a> {
    /// The item, without the Slot tag.
    ///
    /// `RecordItem`
    pub record_item: ItemWithNoSlot<'a>,

    /// The number of ticks since the song started playing. Automatically stops the sound event of the song once it reaches the song's length in ticks, and controls when particles are created. Does not exist when there is no song playing or no item.
    pub ticks_since_song_started: i64,
}

impl<'a> FromCompoundNbt for Jukebox<'a> {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        Ok(Jukebox {
            record_item: nbt
                .compound("record_item")
                .map(|r| ItemWithNoSlot::from_compound_nbt(&r))
                .ok_or(SculkParseError::MissingField("record_item".into()))??,

            ticks_since_song_started: nbt.long("ticks_since_song_started").ok_or(
                SculkParseError::MissingField("ticks_since_song_started".into()),
            )?,
        })
    }
}
