use serde::{Deserialize, Serialize};

use crate::item::ItemWithNoSlot;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Jukebox<'a> {
    /// The item, without the Slot tag.
    #[serde(borrow)]
    #[serde(rename = "RecordItem")]
    pub record_item: ItemWithNoSlot<'a>,

    /// The number of ticks since the song started playing. Automatically stops the sound event of the song once it reaches the song's length in ticks, and controls when particles are created. Does not exist when there is no song playing or no item.
    pub ticks_since_song_started: i64,
}
