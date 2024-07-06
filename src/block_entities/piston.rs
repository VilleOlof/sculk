use std::{borrow::Cow, collections::HashMap};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Piston<'a> {
    /// The moving block represented by this block entity.
    #[serde(borrow)]
    #[serde(rename = "blockState")]
    pub block_state: BlockState<'a>,

    /// true if the piston is extending instead of withdrawing.
    pub extending: bool,

    /// Direction that the piston pushes
    pub facing: Facing,

    /// How far the block has been moved. Starts at 0.0, and increments by 0.5 each tick. If the value is 1.0 or higher at the start of a tick (before incrementing), then the block transforms into the stored blockState. Negative values can be used to increase the time until transformation.
    pub progress: f32,

    ///  true if the block represents the piston head itself, false if it represents a block being pushed.
    pub source: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct BlockState<'a> {
    /// The identifier of the block to use.
    #[serde(borrow)]
    pub name: Cow<'a, str>,

    /// (Optional, can be empty) Block properties. Unspecified properties of the specified block will be set to their default values.
    #[serde(borrow)]
    pub properties: Option<HashMap<Cow<'a, str>, Cow<'a, str>>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(from = "i32")]
pub enum Facing {
    Down = 0,
    Up = 1,
    North = 2,
    South = 3,
    West = 4,
    East = 5,
}

impl From<i32> for Facing {
    fn from(value: i32) -> Self {
        match value {
            0 => Facing::Down,
            1 => Facing::Up,
            2 => Facing::North,
            3 => Facing::South,
            4 => Facing::West,
            5 => Facing::East,
            _ => panic!("Invalid value for Facing: {}", value),
        }
    }
}
