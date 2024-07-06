use serde::{Deserialize, Serialize};

use crate::bees::Bee;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Beehive {
    /// Entities currently in the hive.
    bees: Vec<Bee>,

    /// Stores the flower block location, as 3 integers, so other bees can go to it.
    flower_pos: [i32; 3],
}
