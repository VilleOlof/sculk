use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::banner_patterns::BannerPattern;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Banner<'a> {
    /// Optional. The name of this banner in JSON text component, which is used for showing the banner on a map.
    #[serde(borrow)]
    #[serde(rename = "CustomName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_name: Option<Cow<'a, str>>,

    /// List of all patterns applied to the banner.
    #[serde(borrow)]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub patterns: Vec<BannerPattern<'a>>,
}

#[cfg(test)]
#[test]
fn basic_test() {
    use crate::{banner_patterns::Pattern, banner_patterns::ResourceName, color::Color};
    use fastnbt::nbt;

    let nbt = nbt!({
        "patterns": [
            {
                "color": "red",
                "pattern": "diagonal_left"
            },
            {
                "color": "light_blue",
                "pattern": "mojang"
            }
        ]
    });

    let banner: Banner = fastnbt::from_value(&nbt).unwrap();

    assert_eq!(banner.custom_name, None);
    assert_eq!(banner.patterns.len(), 2);
    assert_eq!(banner.patterns[0].color, Color::Red);
    assert_eq!(
        banner.patterns[0].pattern,
        Pattern::ID(ResourceName::DiagonalLeft)
    );
    assert_eq!(banner.patterns[1].color, Color::LightBlue);
    assert_eq!(
        banner.patterns[1].pattern,
        Pattern::ID(ResourceName::Mojang)
    );

    let nbt = fastnbt::to_value(&banner).unwrap();

    assert_eq!(nbt, nbt);
}

#[cfg(test)]
#[test]
fn empty_test() {
    use fastnbt::nbt;

    let nbt = nbt!({});

    let banner: Banner = fastnbt::from_value(&nbt).unwrap();

    assert_eq!(banner.custom_name, None);
    assert_eq!(banner.patterns.len(), 0);

    let nbt = fastnbt::to_value(&banner).unwrap();

    assert_eq!(nbt, nbt);
}

#[cfg(test)]
#[test]
fn extended_test() {
    use crate::{banner_patterns::Pattern, banner_patterns::ResourceName, color::Color};
    use fastnbt::nbt;

    let nbt = nbt!({
        "CustomName": "My banner",
        "patterns": [
            {
                "color": "black",
                "pattern": {
                    "asset_id": "minecraft:block/diagonal_left",
                    "translation_key": "block.minecraft.diagonal_left"
                }
            },
            {
                "color": "purple",
                "pattern": "cross"
            },
            {
                "color": "green",
                "pattern": {
                    "asset_id": "minecraft:block/stripe_right",
                    "translation_key": "block.minecraft.stripe_right"
                }
            }
        ]
    });

    let banner: Banner = fastnbt::from_value(&nbt).unwrap();

    assert_eq!(banner.custom_name, Some(Cow::Borrowed("My banner")));
    assert_eq!(banner.patterns.len(), 3);
    assert_eq!(banner.patterns[0].color, Color::Black);
    assert_eq!(
        banner.patterns[0].pattern,
        Pattern::Pattern {
            asset_id: Cow::Borrowed("minecraft:block/diagonal_left"),
            translation_key: Cow::Borrowed("block.minecraft.diagonal_left")
        }
    );
    assert_eq!(banner.patterns[1].color, Color::Purple);
    assert_eq!(banner.patterns[1].pattern, Pattern::ID(ResourceName::Cross));
    assert_eq!(banner.patterns[2].color, Color::Green);
    assert_eq!(
        banner.patterns[2].pattern,
        Pattern::Pattern {
            asset_id: Cow::Borrowed("minecraft:block/stripe_right"),
            translation_key: Cow::Borrowed("block.minecraft.stripe_right")
        }
    );

    let nbt = fastnbt::to_value(&banner).unwrap();

    assert_eq!(nbt, nbt);
}
