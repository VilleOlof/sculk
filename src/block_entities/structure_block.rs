use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct StructureBlock<'a> {
    /// Author of the structure; only set to "?" for most vanilla structures.
    pub author: Cow<'a, str>,

    /// Whether entities should be ignored in the structure.
    #[serde(rename = "ignoreEntities")]
    pub ignore_entities: bool,

    /// How complete the structure is that gets placed.
    pub integrity: f32,

    /// Value of the data structure block field.
    pub metadata: Cow<'a, str>,

    /// How the structure is mirrored, one of "NONE", "LEFT_RIGHT" (mirrored over X axis when not rotated), or "FRONT_BACK" (mirrored over Z axis when not rotated).
    pub mirror: StructureBlockMirror,

    /// The current mode of this structure block, one of "SAVE", "LOAD", "CORNER", or "DATA".
    pub mode: StructureBlockMode,

    /// Name of the structure.
    pub name: Cow<'a, str>,

    /// X-position of the structure.
    #[serde(rename = "posX")]
    pub pos_x: i32,

    /// Y-position of the structure.
    #[serde(rename = "posY")]
    pub pos_y: i32,

    /// Z-position of the structure.
    #[serde(rename = "posZ")]
    pub pos_z: i32,

    /// Whether this structure block is being powered by redstone.
    pub powered: bool,

    /// Rotation of the structure, one of "NONE", "CLOCKWISE_90", "CLOCKWISE_180", or "COUNTERCLOCKWISE_90".
    pub rotation: StructureBlockRotation,

    /// The seed to use for the structure integrity, 0 means random.
    pub seed: i64,

    /// Whether to show the structure's bounding box to players in Creative mode.
    #[serde(rename = "showboundingbox")]
    pub show_bounding_box: bool,

    /// X-size of the structure, its length.
    #[serde(rename = "sizeX")]
    pub size_x: i32,

    /// Y-size of the structure, its height.
    #[serde(rename = "sizeY")]
    pub size_y: i32,

    /// Z-size of the structure, its depth.
    #[serde(rename = "sizeZ")]
    pub size_z: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(from = "&str")]
#[serde(rename_all = "UPPERCASE")]
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

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(from = "&str")]
#[serde(rename_all = "UPPERCASE")]
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

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(from = "&str")]
#[serde(rename_all = "UPPERCASE")]
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
