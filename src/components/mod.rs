use std::{borrow::Cow, collections::HashMap};

use fastnbt::Value;
use serde::{Deserialize, Serialize};

use crate::item::Item;

pub mod attribute_modifiers;
pub mod banner_patterns;
pub mod base_color;
pub mod bees;
pub mod block_state;
pub mod bucket_entity_data;
pub mod can_break;

pub type ComponentMap<'a> = HashMap<Cow<'a, str>, Component<'a>>;

/// Represents a component in a block entity.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum Component<'a> {
    #[serde(borrow)]
    #[serde(rename = "minecraft:attribute_modifiers")]
    AttributeModifiers(attribute_modifiers::AttributeModifiers<'a>),

    #[serde(borrow)]
    #[serde(rename = "minecraft:banner_patterns")]
    BannerPatterns(Vec<banner_patterns::BannerPattern<'a>>),

    #[serde(borrow)]
    #[serde(rename = "minecraft:base_color")]
    BaseColor(base_color::BaseColor<'a>),

    #[serde(borrow)]
    #[serde(rename = "minecraft:bees")]
    Bees(Vec<bees::Bee<'a>>),

    // TODO: This should mimic `BlockEntity` except no x, y, z fields.
    // Would probably require a post-processing step for BlockEntityData.
    #[serde(rename = "minecraft:block_entity_data")]
    BlockEntityData(Value),

    #[serde(borrow)]
    #[serde(rename = "minecraft:block_state")]
    BlockState(block_state::BlockState<'a>),

    #[serde(rename = "minecraft:bucket_entity_data")]
    BucketEntityData(bucket_entity_data::BucketEntityData),

    #[serde(borrow)]
    #[serde(rename = "minecraft:bundle_contents")]
    BundleContents(Vec<Item<'a>>),

    #[serde(borrow)]
    #[serde(rename = "minecraft:can_break")]
    CanBreak(can_break::CanBreak<'a>),
}
