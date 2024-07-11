use std::borrow::Cow;

use simdnbt::Mutf8Str;

use crate::{
    error::SculkParseError,
    traits::FromCompoundNbt,
    util::{get_bool, get_owned_mutf8str},
};

#[derive(Debug, Clone, PartialEq)]
pub struct StructureBlock<'a> {
    /// Author of the structure; only set to "?" for most vanilla structures.
    pub author: Cow<'a, Mutf8Str>,

    /// Whether entities should be ignored in the structure.
    ///
    /// `ignoreEntities`
    pub ignore_entities: bool,

    /// How complete the structure is that gets placed.
    pub integrity: f32,

    /// Value of the data structure block field.
    pub metadata: Cow<'a, Mutf8Str>,

    /// How the structure is mirrored, one of "NONE", "LEFT_RIGHT" (mirrored over X axis when not rotated), or "FRONT_BACK" (mirrored over Z axis when not rotated).
    pub mirror: StructureBlockMirror,

    /// The current mode of this structure block, one of "SAVE", "LOAD", "CORNER", or "DATA".
    pub mode: StructureBlockMode,

    /// Name of the structure.
    pub name: Cow<'a, Mutf8Str>,

    /// X-position of the structure.
    ///
    /// `posX`
    pub pos_x: i32,

    /// Y-position of the structure.
    ///
    /// `posY`
    pub pos_y: i32,

    /// Z-position of the structure.
    ///
    /// `posZ`
    pub pos_z: i32,

    /// Whether this structure block is being powered by redstone.
    pub powered: bool,

    /// Rotation of the structure, one of "NONE", "CLOCKWISE_90", "CLOCKWISE_180", or "COUNTERCLOCKWISE_90".
    pub rotation: StructureBlockRotation,

    /// The seed to use for the structure integrity, 0 means random.
    pub seed: i64,

    /// Whether to show the structure's bounding box to players in Creative mode.
    ///
    /// `showboundingbox`
    pub show_bounding_box: bool,

    /// X-size of the structure, its length.
    ///
    /// `sizeX`
    pub size_x: i32,

    /// Y-size of the structure, its height.
    ///
    /// `sizeY`
    pub size_y: i32,

    /// Z-size of the structure, its depth.
    ///
    /// `sizeZ`
    pub size_z: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StructureBlockMirror {
    None,
    LeftRight,
    FrontBack,
}

impl From<&str> for StructureBlockMirror {
    fn from(s: &str) -> Self {
        match s {
            "NONE" => Self::None,
            "LEFT_RIGHT" => Self::LeftRight,
            "FRONT_BACK" => Self::FrontBack,
            _ => panic!("Invalid value for StructureBlockMirror: {}", s),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum StructureBlockMode {
    Save,
    Load,
    Corner,
    Data,
}

impl From<&str> for StructureBlockMode {
    fn from(s: &str) -> Self {
        match s {
            "SAVE" => Self::Save,
            "LOAD" => Self::Load,
            "CORNER" => Self::Corner,
            "DATA" => Self::Data,
            _ => panic!("Invalid value for StructureBlockMode: {}", s),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum StructureBlockRotation {
    None,
    Clockwise90,
    Clockwise180,
    CounterClockwise90,
}

impl From<&str> for StructureBlockRotation {
    fn from(s: &str) -> Self {
        match s {
            "NONE" => Self::None,
            "CLOCKWISE_90" => Self::Clockwise90,
            "CLOCKWISE_180" => Self::Clockwise180,
            "COUNTERCLOCKWISE_90" => Self::CounterClockwise90,
            _ => panic!("Invalid value for StructureBlockRotation: {}", s),
        }
    }
}

impl<'a> FromCompoundNbt for StructureBlock<'a> {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let author = get_owned_mutf8str(&nbt, "author")?;
        let ignore_entities = get_bool(&nbt, "ignoreEntities");

        let integrity = nbt
            .float("integrity")
            .ok_or(SculkParseError::MissingField("integrity".into()))?;

        let metadata = get_owned_mutf8str(&nbt, "metadata")?;

        let mirror = nbt
            .string("mirror")
            .map(|string| StructureBlockMirror::from(string.to_str().as_ref()))
            .ok_or(SculkParseError::MissingField("mirror".into()))?;

        let mode = nbt
            .string("mode")
            .map(|string| StructureBlockMode::from(string.to_str().as_ref()))
            .ok_or(SculkParseError::MissingField("mode".into()))?;

        let name = get_owned_mutf8str(&nbt, "name")?;

        let pos_x = nbt
            .int("posX")
            .ok_or(SculkParseError::MissingField("posX".into()))?;
        let pos_y = nbt
            .int("posY")
            .ok_or(SculkParseError::MissingField("posY".into()))?;
        let pos_z = nbt
            .int("posZ")
            .ok_or(SculkParseError::MissingField("posZ".into()))?;

        let powered = get_bool(&nbt, "powered");

        let rotation = nbt
            .string("rotation")
            .map(|string| StructureBlockRotation::from(string.to_str().as_ref()))
            .ok_or(SculkParseError::MissingField("rotation".into()))?;

        let seed = nbt
            .long("seed")
            .ok_or(SculkParseError::MissingField("seed".into()))?;
        let show_bounding_box = get_bool(&nbt, "showboundingbox");

        let size_x = nbt
            .int("sizeX")
            .ok_or(SculkParseError::MissingField("sizeX".into()))?;
        let size_y = nbt
            .int("sizeY")
            .ok_or(SculkParseError::MissingField("sizeY".into()))?;
        let size_z = nbt
            .int("sizeZ")
            .ok_or(SculkParseError::MissingField("sizeZ".into()))?;

        Ok(StructureBlock {
            author,
            ignore_entities,
            integrity,
            metadata,
            mirror,
            mode,
            name,
            pos_x,
            pos_y,
            pos_z,
            powered,
            rotation,
            seed,
            show_bounding_box,
            size_x,
            size_y,
            size_z,
        })
    }
}
