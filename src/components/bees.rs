use serde::{Deserialize, Serialize};

use crate::entity::Entity;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Bee<'a> {
    /// The NBT data of the entity in the hive.
    #[serde(borrow)]
    pub entity_data: Entity<'a>,

    /// The minimum amount of time in ticks for this entity to stay in the hive.
    pub min_ticks_in_hive: i32,

    /// The amount of ticks the entity has stayed in the hive.
    pub ticks_in_hive: i32,
}
