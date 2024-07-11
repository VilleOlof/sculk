use std::{collections::HashMap, ops::Deref};

use attribute_modifiers::AttributeModifier;
use banner_patterns::BannerPattern;
use base_color::BaseColor;

use crate::{error::SculkParseError, item::Item, traits::FromCompoundNbt};

pub mod attribute_modifiers;
pub mod banner_patterns;
pub mod base_color;
pub mod bees;
pub mod block_state;
pub mod bucket_entity_data;
pub mod can_break;

type InternalMap<'a> = HashMap<String, Component<'a>>;

#[derive(Debug, Clone, PartialEq)]
pub struct Components<'a>(InternalMap<'a>);

impl<'a> Deref for Components<'a> {
    type Target = InternalMap<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> FromCompoundNbt for Components<'a> {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let nbt_components = nbt
            .compound("components")
            .ok_or(SculkParseError::MissingField("components".into()))?;

        let mut map: InternalMap = HashMap::new();

        for (key, value) in nbt_components.iter() {
            let key = key.to_string();

            let component: Component<'a> = match key.as_str() {
                "minecraft:attribute_modifiers" => {
                    // since the root value is either list or compound, we need to pass parent nbt.
                    Component::AttributeModifiers(AttributeModifier::from_compound_nbt(&nbt)?)
                }
                "minecraft:banner_patterns" => {
                    let patterns = value
                        .list()
                        .ok_or(SculkParseError::InvalidField(
                            "minecraft:banner_patterns".into(),
                        ))?
                        .compounds()
                        .ok_or(SculkParseError::InvalidField(
                            "minecraft:banner_patterns".into(),
                        ))?
                        .into_iter()
                        .map(|nbt| BannerPattern::from_compound_nbt(&nbt))
                        .collect::<Result<Vec<BannerPattern>, SculkParseError>>()?;

                    Component::BannerPatterns(patterns)
                }
                "minecraft:base_color" => Component::BaseColor(BaseColor::from_compound_nbt(&nbt)?),
                _ => Component::Unknown(value.to_owned()),
            };

            map.insert(key, component);
        }

        Ok(Components(map))
    }
}

/// Represents a component in a block entity.
#[derive(Debug, Clone, PartialEq)]
// #[serde(untagged)]
pub enum Component<'a> {
    AttributeModifiers(attribute_modifiers::AttributeModifier<'a>),

    BannerPatterns(Vec<banner_patterns::BannerPattern<'a>>),

    BaseColor(base_color::BaseColor<'a>),

    Bees(Vec<bees::Bee<'a>>),

    // TODO: This should mimic `BlockEntity` except no x, y, z fields.
    // Would probably require a post-processing step for BlockEntityData.
    // BlockEntityData(Value),
    BlockState(block_state::BlockState<'a>),

    BucketEntityData(bucket_entity_data::BucketEntityData),

    BundleContents(Vec<Item<'a>>),

    CanBreak(can_break::CanBreak<'a>),

    Unknown(simdnbt::owned::NbtTag),
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

// #[cfg(test)]
// #[test]
// fn component_test() {
//     use fastnbt::nbt;

//     let nbt = nbt!({
//         "id": "minecraft:white_banner",
//         "keepPacked": 0i8,
//         "x": 1,
//         "y": 2,
//         "z": 3,
//         "components": {
//             "minecraft:banner_patterns": [
//                 {
//                     "color": "red",
//                     "pattern": "cross"
//                 }
//             ],
//             // "minecraft:attribute_modifiers": {
//             //     "show_in_tooltip": true,
//             //     "modifiers": [
//             //         {
//             //             "type": "minecraft:generic.max_health",
//             //             "slot": "mainhand",
//             //             "amount": 2,
//             //             "operation": "add_value"
//             //         }
//             //     ]
//             // }
//         }
//     });

//     println!("{:#?}", nbt);

//     let block_entity: super::BlockEntity = fastnbt::from_value(&nbt).unwrap();

//     println!("{:#?}", block_entity);
// }
