use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::color::Color;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct BannerPattern<'a> {
    /// Dye color of the section.
    pub color: Color,

    /// Banner pattern (referenced by ID or inlined)
    #[serde(borrow)]
    pub pattern: Pattern<'a>,
}

/// Either an id reference to a pattern or an inlined pattern.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum Pattern<'a> {
    /// The resource location for the texture asset.
    ID(ResourceName),

    /// The translation key for displaying the banner tooltip.
    Pattern {
        asset_id: Cow<'a, str>,
        translation_key: Cow<'a, str>,
    },
}

/// The list of available banner patterns.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
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
    TriangleBottom,
    TriangleTop,
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
}
