use crate::{item::ItemWithNoSlot, traits::FromCompoundNbt};

#[derive(Debug, Clone, PartialEq)]
pub struct Jukebox {
    /// The item, without the Slot tag.
    ///
    /// `RecordItem`
    pub record_item: Option<ItemWithNoSlot>,

    /// The number of ticks since the song started playing. Automatically stops the sound event of the song once it reaches the song's length in ticks, and controls when particles are created. Does not exist when there is no song playing or no item.
    pub ticks_since_song_started: Option<i64>,
}

impl FromCompoundNbt for Jukebox {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        Ok(Jukebox {
            record_item: if let Some(nbt) = nbt.compound("RecordItem") {
                Some(ItemWithNoSlot::from_compound_nbt(&nbt)?)
            } else {
                None
            },

            ticks_since_song_started: nbt.long("ticks_since_song_started"),
        })
    }
}
