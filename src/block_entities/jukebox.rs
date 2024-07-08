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

#[cfg(test)]
#[test]
fn test() {
    use fastnbt::nbt;

    let nbt = nbt!({
        "RecordItem": {
            "id": "minecraft:record_13",
            "Count": 1
        },
        "ticks_since_song_started": 0i64
    });

    let jukebox: Jukebox = fastnbt::from_value(&nbt).unwrap();

    assert_eq!(jukebox.record_item.id, "minecraft:record_13");
    assert_eq!(jukebox.record_item.count, 1);
    assert_eq!(jukebox.ticks_since_song_started, 0);

    let serialized_nbt = fastnbt::to_value(&jukebox).unwrap();

    assert_eq!(nbt, serialized_nbt);
}
