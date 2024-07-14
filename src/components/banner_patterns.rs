//! Banner patterns are used in banners to determine the pattern of the banner.

use crate::{
    color::Color, error::SculkParseError, traits::FromCompoundNbt, util::get_owned_mutf8str,
};
use simdnbt::Mutf8Str;
use std::borrow::Cow;

/// Represents a banner pattern.
#[derive(Debug, Clone, PartialEq)]
pub struct BannerPattern<'a> {
    /// Dye color of the section.
    pub color: Color,

    /// Banner pattern (referenced by ID or inlined)
    pub pattern: Pattern<'a>,
}

/// Either an id reference to a pattern or an inlined pattern.
#[derive(Debug, Clone, PartialEq)]
pub enum Pattern<'a> {
    /// The resource location for the texture asset.
    ID(ResourceName),

    /// The translation key for displaying the banner tooltip.
    Pattern {
        /// The asset ID of the pattern.
        asset_id: Cow<'a, Mutf8Str>,
        /// The translation key for displaying the banner tooltip.
        translation_key: Cow<'a, Mutf8Str>,
    },
}

/// The list of available banner patterns.
#[derive(Debug, Clone, PartialEq)]
pub enum ResourceName {
    Base,
    StripeBottom,
    StripeTop,
    StripeLeft,
    StripeRight,
    StripeCenter,
    StripeMiddle,
    StripeDownright,
    StripeDownleft,
    SmallStripes,
    Cross,
    StraightCross,
    DiagonalLeft,
    DiagonalRight,
    DiagonalUpLeft,
    DiagonalUpRight,
    HalfVertical,
    HalfVerticalRight,
    HalfHorizontal,
    HalfHorizontalBottom,
    SquareBottomLeft,
    SquareBottomRight,
    SquareTopLeft,
    SquareTopRight,
    TrianglesBottom,
    TrianglesTop,
    Circle,
    Rhombus,
    Border,
    CurlyBorder,
    Bricks,
    Gradient,
    GradientUp,
    Creeper,
    Skull,
    Flower,
    Mojang,
    Globe,
    Piglin,
    Flow,
    Guster,
    Unknown(String),
}

impl From<&str> for ResourceName {
    fn from(value: &str) -> Self {
        match value {
            "minecraft:base" => Self::Base,
            "minecraft:stripe_bottom" => Self::StripeBottom,
            "minecraft:stripe_top" => Self::StripeTop,
            "minecraft:stripe_left" => Self::StripeLeft,
            "minecraft:stripe_right" => Self::StripeRight,
            "minecraft:stripe_center" => Self::StripeCenter,
            "minecraft:stripe_middle" => Self::StripeMiddle,
            "minecraft:stripe_downright" => Self::StripeDownright,
            "minecraft:stripe_downleft" => Self::StripeDownleft,
            "minecraft:small_stripes" => Self::SmallStripes,
            "minecraft:cross" => Self::Cross,
            "minecraft:straight_cross" => Self::StraightCross,
            "minecraft:diagonal_left" => Self::DiagonalLeft,
            "minecraft:diagonal_right" => Self::DiagonalRight,
            "minecraft:diagonal_up_left" => Self::DiagonalUpLeft,
            "minecraft:diagonal_up_right" => Self::DiagonalUpRight,
            "minecraft:half_vertical" => Self::HalfVertical,
            "minecraft:half_vertical_right" => Self::HalfVerticalRight,
            "minecraft:half_horizontal" => Self::HalfHorizontal,
            "minecraft:half_horizontal_bottom" => Self::HalfHorizontalBottom,
            "minecraft:square_bottom_left" => Self::SquareBottomLeft,
            "minecraft:square_bottom_right" => Self::SquareBottomRight,
            "minecraft:square_top_left" => Self::SquareTopLeft,
            "minecraft:square_top_right" => Self::SquareTopRight,
            "minecraft:triangles_bottom" => Self::TrianglesBottom,
            "minecraft:triangles_top" => Self::TrianglesTop,
            "minecraft:circle" => Self::Circle,
            "minecraft:rhombus" => Self::Rhombus,
            "minecraft:border" => Self::Border,
            "minecraft:curly_border" => Self::CurlyBorder,
            "minecraft:bricks" => Self::Bricks,
            "minecraft:gradient" => Self::Gradient,
            "minecraft:gradient_up" => Self::GradientUp,
            "minecraft:creeper" => Self::Creeper,
            "minecraft:skull" => Self::Skull,
            "minecraft:flower" => Self::Flower,
            "minecraft:mojang" => Self::Mojang,
            "minecraft:globe" => Self::Globe,
            "minecraft:piglin" => Self::Piglin,
            "minecraft:flow" => Self::Flow,
            "minecraft:guster" => Self::Guster,
            _ => Self::Unknown(value.into()),
        }
    }
}

impl<'a> FromCompoundNbt for BannerPattern<'a> {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let color = nbt
            .string("color")
            .map(|s| Color::from(s.to_str().as_ref()))
            .ok_or(crate::error::SculkParseError::MissingField("color".into()))?;

        let pattern = if let Some(id) = nbt.string("pattern") {
            // String Id
            let resource = ResourceName::from(id.to_str().as_ref());

            Pattern::ID(resource)
        } else if let Some(_) = nbt.compound("pattern") {
            // Inlined pattern
            let asset_id = get_owned_mutf8str(&nbt, "asset_id")?;
            let translation_key = get_owned_mutf8str(&nbt, "translation_key")?;

            Pattern::Pattern {
                asset_id,
                translation_key,
            }
        } else {
            return Err(SculkParseError::MissingField("pattern".into()));
        };

        Ok(BannerPattern { color, pattern })
    }
}
