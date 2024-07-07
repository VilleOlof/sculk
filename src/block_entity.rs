use serde::{Deserialize, Serialize};

use crate::{
    block_entities::{variant::BlockEntityVariant, BlockEntityData},
    ComponentMap,
};

/// Refer to `BlockEntity` for documentation.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct BlockEntityBase<'a> {
    #[serde(rename = "keepPacked")]
    #[serde(deserialize_with = "crate::util::i8_to_bool")]
    pub keep_packed: bool,

    pub x: i32,
    pub y: i32,
    pub z: i32,

    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    // TODO: This should be a HashMap here and a custom deserializer.
    pub components: Option<ComponentMap<'a>>,
}

/// Represents a block entity.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct BlockEntity<'a> {
    #[serde(flatten)]
    pub base: BlockEntityBase<'a>,

    #[serde(borrow)]
    #[serde(flatten)]
    pub kind: BlockEntityData<'a>,
}

impl<'a> BlockEntity<'a> {
    pub fn variant(&self) -> BlockEntityVariant {
        self.kind.variant()
    }
}
