use serde::{Deserialize, Serialize};

use super::calibrated_sculk_sensor::Listener;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct SculkShrieker<'a> {
    /// The vibration event listener of the sculk shrieker, sculk sensor, and calibrated sculk sensor.
    // TODO: This one is hella sketch, wiki just says a bunch of `Unknown` and it seems old, gonna assume its like the other sculk blocks.
    #[serde(borrow)]
    pub listener: Listener<'a>,
}
