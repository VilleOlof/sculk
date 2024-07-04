use fastnbt::Value;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Bee {
    /// The NBT data of the entity in the hive.
    // TODO: Make a EntityFormat enum/struct to handle this.
    // https://minecraft.wiki/w/Entity_format
    entity_data: Value,

    /// The minimum amount of time in ticks for this entity to stay in the hive.
    min_ticks_in_hive: i32,

    /// The amount of ticks the entity has stayed in the hive.
    ticks_in_hive: i32,
}
