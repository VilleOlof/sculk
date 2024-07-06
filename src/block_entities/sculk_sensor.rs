use serde::{Deserialize, Serialize};

use super::calibrated_sculk_sensor::Listener;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct SculkSensor<'a> {
    /// The frequency of the last vibration.
    pub last_vibration_frequency: i32,

    /// The vibration event listener for this sculk shrieker, sculk sensor, or calibrated sculk sensor.
    #[serde(borrow)]
    pub listener: Listener<'a>,
}
