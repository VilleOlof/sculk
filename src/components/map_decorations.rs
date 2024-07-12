use std::{borrow::Cow, collections::HashMap};

use crate::{error::SculkParseError, kv::KVPair, traits::FromCompoundNbt};

#[derive(Debug, Clone, PartialEq)]
pub struct MapDecorations<'a>(KVPair<'a, MapIcon>);

/// The key-value pair of a single icon, where the key is an arbitrary unique string identifying the decoration.
#[derive(Debug, Clone, PartialEq)]
pub struct MapIcon {
    pub r#type: MapIconType,

    pub x: f64,
    pub z: f64,

    pub rotation: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MapIconType {
    Player,
    Frame,
    RedMarker,
    BlueMarker,
    TargetX,
    TargetPoint,
    PlayerOffMap,
    PlayerOffLimits,
    Mansion,
    Monument,
    BannerWhite,
    BannerOrange,
    BanneMagenta,
    BannerLightBlue,
    BannerYellow,
    BannerLime,
    BannerPink,
    BannerGray,
    BannerLightGray,
    BannerCyan,
    BannerPurple,
    BannerBlue,
    BannerBrown,
    BannerGreen,
    BannerRed,
    BannerBlack,
    RedX,
    VillageDesert,
    VillagePlains,
    VillageSavanna,
    VillageSnowy,
    VillageTaiga,
    JungleTemple,
    SwampHut,
    Unknown(String),
}

impl MapIconType {
    pub fn from_str(s: &str) -> Self {
        match s {
            "player" => Self::Player,
            "frame" => Self::Frame,
            "red_marker" => Self::RedMarker,
            "blue_marker" => Self::BlueMarker,
            "target_x" => Self::TargetX,
            "target_point" => Self::TargetPoint,
            "player_off_map" => Self::PlayerOffMap,
            "player_off_limits" => Self::PlayerOffLimits,
            "mansion" => Self::Mansion,
            "monument" => Self::Monument,
            "banner_white" => Self::BannerWhite,
            "banner_orange" => Self::BannerOrange,
            "banner_magenta" => Self::BanneMagenta,
            "banner_light_blue" => Self::BannerLightBlue,
            "banner_yellow" => Self::BannerYellow,
            "banner_lime" => Self::BannerLime,
            "banner_pink" => Self::BannerPink,
            "banner_gray" => Self::BannerGray,
            "banner_light_gray" => Self::BannerLightGray,
            "banner_cyan" => Self::BannerCyan,
            "banner_purple" => Self::BannerPurple,
            "banner_blue" => Self::BannerBlue,
            "banner_brown" => Self::BannerBrown,
            "banner_green" => Self::BannerGreen,
            "banner_red" => Self::BannerRed,
            "banner_black" => Self::BannerBlack,
            "red_x" => Self::RedX,
            "village_desert" => Self::VillageDesert,
            "village_plains" => Self::VillagePlains,
            "village_savanna" => Self::VillageSavanna,
            "village_snowy" => Self::VillageSnowy,
            "village_taiga" => Self::VillageTaiga,
            "jungle_temple" => Self::JungleTemple,
            "swamp_hut" => Self::SwampHut,
            _ => Self::Unknown(s.to_string()),
        }
    }
}

impl<'a> FromCompoundNbt for MapDecorations<'a> {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        Ok(MapDecorations(KVPair::from_compound_nbt(&nbt)?))
    }
}

impl<'a> FromCompoundNbt for KVPair<'a, MapIcon> {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let mut map = HashMap::new();

        for (key, value) in nbt.iter() {
            let key = key.to_string();
            let value = MapIcon::from_compound_nbt(&value.compound().ok_or(
                SculkParseError::InvalidField("minecraft:map_decorations".into()),
            )?)?;

            map.insert(Cow::Owned(key), value);
        }

        Ok(KVPair::new(map))
    }
}

impl FromCompoundNbt for MapIcon {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let r#type = nbt
            .string("type")
            .map(|s| MapIconType::from_str(s.to_str().as_ref()))
            .ok_or(SculkParseError::MissingField("type".into()))?;

        let x = nbt
            .double("x")
            .ok_or(SculkParseError::MissingField("x".into()))?;
        let z = nbt
            .double("z")
            .ok_or(SculkParseError::MissingField("z".into()))?;

        let rotation = nbt
            .float("rotation")
            .ok_or(SculkParseError::MissingField("rotation".into()))?;

        Ok(MapIcon {
            r#type,
            x,
            z,
            rotation,
        })
    }
}
