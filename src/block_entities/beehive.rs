use serde::{Deserialize, Serialize};

use crate::bees::Bee;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Beehive<'a> {
    /// Entities currently in the hive.
    #[serde(borrow)]
    bees: Vec<Bee<'a>>,

    /// Stores the flower block location, as 3 integers, so other bees can go to it.
    flower_pos: [i32; 3],
}
