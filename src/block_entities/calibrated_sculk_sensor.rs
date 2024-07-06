use std::borrow::Cow;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct CalibratedSculkSensor<'a> {
    /// The frequency of the last vibration.
    pub last_vibration_frequency: i32,

    /// The vibration event listener for this sculk shrieker, sculk sensor, or calibrated sculk sensor.
    #[serde(borrow)]
    pub listener: Listener<'a>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Listener<'a> {
    /// Only exists if there is an incoming vibration.
    #[serde(borrow)]
    pub event: Option<Event<'a>>,

    /// How many ticks remain until triggered by the vibration. Set to 0 if there is no incoming vibration
    pub event_delay: i32,

    /// The data of the vibration selector.​
    #[serde(borrow)]
    pub selector: Selector<'a>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Event<'a> {
    /// The distance between this vibration's source and the block.
    pub distance: f32,

    /// The resource location of the vibration event that caused the current incoming vibration.
    #[serde(borrow)]
    pub game_event: Cow<'a, str>,

    /// The coordinates of the source of this vibration.  
    ///
    /// X coordinate.  
    /// Y coordinate.  
    /// Z coordinate.  
    pub pos: [f64; 3],

    /// If the vibration was caused by a projectile, this is the UUID of the entity that launched the projectile. Does not exist if vibration was not caused by a projectile.
    pub projectile_owner: Option<i128>,

    /// The UUID of the entity that caused the vibration. Does not exist if vibration was not caused by an entity.
    pub source: Option<i128>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Selector<'a> {
    /// The game time when the vibration occurs, or -1 if there is no vibration to choose from.​
    pub tick: i64,

    /// Candidate game event
    #[serde(borrow)]
    pub event: Event<'a>,
}
