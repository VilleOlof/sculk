use std::collections::HashMap;

use crate::{
    error::SculkParseError,
    traits::FromCompoundNbt,
    util::{get_bool, get_owned_string},
};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Piston {
    /// The moving block represented by this block entity.
    ///
    /// `blockState`
    pub block_state: BlockState,

    /// true if the piston is extending instead of withdrawing.
    pub extending: bool,

    /// Direction that the piston pushes
    pub facing: Facing,

    /// How far the block has been moved. Starts at 0.0, and increments by 0.5 each tick. If the value is 1.0 or higher at the start of a tick (before incrementing), then the block transforms into the stored blockState. Negative values can be used to increase the time until transformation.
    pub progress: f32,

    ///  true if the block represents the piston head itself, false if it represents a block being pushed.
    pub source: bool,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BlockState {
    /// The identifier of the block to use.
    pub name: String,

    /// (Optional, can be empty) Block properties. Unspecified properties of the specified block will be set to their default values.
    pub properties: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(from = "i32", into = "i32"))]
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

impl From<Facing> for i32 {
    fn from(facing: Facing) -> Self {
        match facing {
            Facing::Down => 0,
            Facing::Up => 1,
            Facing::North => 2,
            Facing::South => 3,
            Facing::West => 4,
            Facing::East => 5,
        }
    }
}

impl FromCompoundNbt for Piston {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let block_state = BlockState::from_compound_nbt(
            &nbt.compound("blockState")
                .ok_or(SculkParseError::MissingField("blockState".into()))?,
        )?;

        let extending = get_bool(&nbt, "extending");
        let facing = if let Some(facing) = nbt.int("facing") {
            Ok(Facing::from(facing))
        } else {
            Err(SculkParseError::MissingField("facing".into()))
        }?;

        let progress = nbt
            .float("progress")
            .ok_or(SculkParseError::MissingField("progress".into()))?;

        let source = nbt
            .byte("source")
            .map(|b| b != 0)
            .ok_or(SculkParseError::MissingField("source".into()))?;

        Ok(Piston {
            block_state,
            extending,
            facing,
            progress,
            source,
        })
    }
}

impl FromCompoundNbt for BlockState {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let name = get_owned_string(&nbt, "name")?;

        let properties = if let Some(props) = nbt.compound("properties") {
            let mut map = HashMap::new();

            for (key, value) in props.iter() {
                let key: String = key.to_string();
                let value: String = value
                    .string()
                    .map(|s| s.to_string())
                    .ok_or(SculkParseError::InvalidField("properties".into()))?;

                map.insert(key, value);
            }

            Some(map)
        } else {
            None
        };

        Ok(BlockState { name, properties })
    }
}
