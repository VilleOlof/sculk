use std::{borrow::Cow, collections::HashMap};

use attribute_modifiers::AttributeModifier;
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
// #[serde(untagged)]
pub enum Component<'a> {
    #[serde(borrow)]
    #[serde(alias = "minecraft:attribute_modifiers")]
    AttributeModifiers(attribute_modifiers::AttributeModifier<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:banner_patterns")]
    BannerPatterns(Vec<banner_patterns::BannerPattern<'a>>),

    #[serde(borrow)]
    #[serde(rename = "minecraft:base_color")]
    BaseColor(base_color::BaseColor<'a>),

    #[serde(borrow)]
    #[serde(rename = "minecraft:bees")]
    Bees(Vec<bees::Bee<'a>>),

    // TODO: This should mimic `BlockEntity` except no x, y, z fields.
    // Would probably require a post-processing step for BlockEntityData.
    // #[serde(rename = "minecraft:block_entity_data")]
    // BlockEntityData(Value),
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

// impl<'a> Component<'a> {
//     fn from_pair(key: &str, data: Value) -> Option<Self> {
//         match key {
//             "minecraft:attribute_modifiers" => {
//                 let data: AttributeModifier<'a> = match fastnbt::from_value(data) {
//                     Ok(data) => data,
//                     Err(_) => return None,
//                 };
//                 Some(Component::AttributeModifiers(data))
//             }
//             _ => unimplemented!(),
//         }
//     }
// }

// pub fn deserialize_components<'a, D>(
//     deserializer: D,
// ) -> Result<HashMap<Cow<'a, str>, Component<'a>>, D::Error>
// where
//     D: serde::Deserializer<'a> + 'a,
// {
//     let map = match Option::<HashMap<&'a str, Value>>::deserialize(deserializer)? {
//         Some(map) => map,
//         None => return Ok(HashMap::new()),
//     };

//     let mut components: HashMap<Cow<'a, str>, Component<'a>> = HashMap::new();
//     for (key, value) in map {
//         let component: Component<'a> = match Component::from_pair(key, value) {
//             Some(component) => component,
//             None => continue,
//         };
//         components.insert(Cow::Borrowed(key), component);
//     }

//     Ok(components)
// }

#[cfg(test)]
#[test]
fn component_test() {
    use fastnbt::nbt;

    let nbt = nbt!({
        "id": "minecraft:white_banner",
        "keepPacked": 0i8,
        "x": 1,
        "y": 2,
        "z": 3,
        "components": {
            "minecraft:banner_patterns": [
                {
                    "color": "red",
                    "pattern": "cross"
                }
            ],
            // "minecraft:attribute_modifiers": {
            //     "show_in_tooltip": true,
            //     "modifiers": [
            //         {
            //             "type": "minecraft:generic.max_health",
            //             "slot": "mainhand",
            //             "amount": 2,
            //             "operation": "add_value"
            //         }
            //     ]
            // }
        }
    });

    println!("{:#?}", nbt);

    let block_entity: super::BlockEntity = fastnbt::from_value(&nbt).unwrap();

    println!("{:#?}", block_entity);
}
