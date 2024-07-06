use fastnbt::Value;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::{borrow::Cow, collections::HashMap};

use crate::{block_entities::BlockEntityData, ComponentMap};

/// Refer to `BlockEntity` for documentation.
#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct LoseBlockEntity<'a> {
    #[serde(borrow)]
    pub id: Cow<'a, str>,

    #[serde(rename = "keepPacked")]
    #[serde(default)]
    pub keep_packed: bool,

    pub x: i32,
    pub y: i32,
    pub z: i32,

    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    // TODO: This should be a HashMap here and a custom deserializer.
    pub components: Option<ComponentMap<'a>>,
    // pub components: Vec<Component<'a>>,
    #[serde(flatten)]
    pub data: Option<HashMap<String, Value>>,
}

/// Represents a block entity.
#[derive(Debug)]
pub struct BlockEntity<'a> {
    /// The ID of the block entity.
    pub id: Cow<'a, str>,

    /// Whether the block entity should keep packed.  
    ///
    /// If `true` the block entity is an invalid block entity,  
    /// And will not immediately be placed.  
    ///
    /// If `false` the block entity is a valid block entity,  
    /// And will immediately be placed.
    pub keep_packed: bool,

    /// The X world coordinate of the block entity.
    pub x: i32,
    /// The Y world coordinate of the block entity.
    pub y: i32,
    /// The Z world coordinate of the block entity.
    pub z: i32,

    /// The components of the block entity.
    // pub components: Vec<Component<'a>>,
    pub components: Option<ComponentMap<'a>>,

    pub data: BlockEntityData<'a>,
}

impl<'a> From<LoseBlockEntity<'a>> for BlockEntity<'a> {
    fn from(value: LoseBlockEntity<'a>) -> Self {
        Self {
            id: value.id.clone(),
            keep_packed: value.keep_packed,
            x: value.x,
            y: value.y,
            z: value.z,
            components: value.components,
            data: BlockEntityData::new(&value.id, value.data),
        }
    }
}

impl<'a> From<&BlockEntity<'a>> for LoseBlockEntity<'a> {
    fn from(value: &BlockEntity<'a>) -> Self {
        Self {
            id: value.id.clone(),
            keep_packed: value.keep_packed,
            x: value.x,
            y: value.y,
            z: value.z,
            components: value.components.clone(),
            data: value.data.as_hashmap(),
        }
    }
}
