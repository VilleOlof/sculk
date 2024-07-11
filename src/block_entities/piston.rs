use std::{borrow::Cow, collections::HashMap};

use simdnbt::Mutf8Str;

use crate::{
    error::SculkParseError,
    traits::FromCompoundNbt,
    util::{get_bool, get_owned_mutf8str},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Piston<'a> {
    /// The moving block represented by this block entity.
    ///
    /// `blockState`
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

#[derive(Debug, Clone, PartialEq)]
pub struct BlockState<'a> {
    /// The identifier of the block to use.
    pub name: Cow<'a, Mutf8Str>,

    /// (Optional, can be empty) Block properties. Unspecified properties of the specified block will be set to their default values.
    pub properties: Option<HashMap<Cow<'a, String>, Cow<'a, Mutf8Str>>>,
}

#[derive(Debug, Clone, PartialEq)]
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

impl<'a> FromCompoundNbt for Piston<'a> {
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

impl<'a> FromCompoundNbt for BlockState<'a> {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let name = get_owned_mutf8str(&nbt, "name")?;

        let properties = if let Some(props) = nbt.compound("properties") {
            let mut map = HashMap::new();

            for (key, value) in props.iter() {
                let key: Cow<'a, String> = Cow::Owned(key.to_string());
                let value: Cow<'a, Mutf8Str> = Cow::Owned(
                    value
                        .string()
                        .map(|s| s.to_owned())
                        .ok_or(SculkParseError::InvalidField("properties".into()))?,
                );

                map.insert(key, value);
            }

            Some(map)
        } else {
            None
        };

        Ok(BlockState { name, properties })
    }
}
