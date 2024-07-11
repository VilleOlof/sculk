use crate::{
    color::Color, error::SculkParseError, traits::FromCompoundNbt, util::get_owned_mutf8str,
};
use simdnbt::Mutf8Str;
use std::borrow::Cow;

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
        asset_id: Cow<'a, Mutf8Str>,
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

impl From<&str> for ResourceName {
    fn from(value: &str) -> Self {
        match value {
            "base" => Self::Base,
            "stripe_bottom" => Self::StripeBottom,
            "stripe_top" => Self::StripeTop,
            "stripe_left" => Self::StripeLeft,
            "stripe_right" => Self::StripeRight,
            "stripe_center" => Self::StripeCenter,
            "stripe_middle" => Self::StripeMiddle,
            "stripe_downright" => Self::StripeDownright,
            "stripe_downleft" => Self::StripeDownleft,
            "small_stripes" => Self::SmallStripes,
            "cross" => Self::Cross,
            "straight_cross" => Self::StraightCross,
            "diagonal_left" => Self::DiagonalLeft,
            "diagonal_right" => Self::DiagonalRight,
            "diagonal_up_left" => Self::DiagonalUpLeft,
            "diagonal_up_right" => Self::DiagonalUpRight,
            "half_vertical" => Self::HalfVertical,
            "half_vertical_right" => Self::HalfVerticalRight,
            "half_horizontal" => Self::HalfHorizontal,
            "half_horizontal_bottom" => Self::HalfHorizontalBottom,
            "square_bottom_left" => Self::SquareBottomLeft,
            "square_bottom_right" => Self::SquareBottomRight,
            "square_top_left" => Self::SquareTopLeft,
            "square_top_right" => Self::SquareTopRight,
            "triangle_bottom" => Self::TriangleBottom,
            "triangle_top" => Self::TriangleTop,
            "circle" => Self::Circle,
            "rhombus" => Self::Rhombus,
            "border" => Self::Border,
            "curly_border" => Self::CurlyBorder,
            "bricks" => Self::Bricks,
            "gradient" => Self::Gradient,
            "gradient_up" => Self::GradientUp,
            "creeper" => Self::Creeper,
            "skull" => Self::Skull,
            "flower" => Self::Flower,
            "mojang" => Self::Mojang,
            "globe" => Self::Globe,
            "piglin" => Self::Piglin,
            "flow" => Self::Flow,
            "guster" => Self::Guster,
            _ => panic!("Invalid value for ResourceName: {}", value),
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
