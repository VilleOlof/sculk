use crate::{error::SculkParseError, traits::FromCompoundNbt, util::get_bool};

#[derive(Debug, Clone, PartialEq)]
pub struct FireworkExplosion {
    /// The shape of the explosion.
    pub shape: FireworkShape,

    ///  The colors of the initial particles of the explosion, randomly selected from.
    pub colors: Vec<i32>,

    /// The colors of the fading particles of the explosion, randomly selected from.
    pub fade_colors: Vec<i32>,

    ///  Whether or not the explosion has a trail effect (diamond).
    pub has_trail: bool,

    ///  Whether or not the explosion has a twinkle effect (glowstone dust).
    pub has_twinkle: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FireworkShape {
    SmallBall,
    LargeBall,
    Star,
    Creeper,
    Burst,
}

impl FromCompoundNbt for FireworkExplosion {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let shape = nbt
            .string("shape")
            .map(|s| FireworkShape::from_str(s.to_str().as_ref()))
            .ok_or(SculkParseError::MissingField("shape".into()))??;

        let colors = nbt.int_array("colors").unwrap_or_default();
        let fade_colors = nbt.int_array("fade_colors").unwrap_or_default();

        let has_trail = get_bool(&nbt, "has_trail");
        let has_twinkle = get_bool(&nbt, "has_twinkle");

        Ok(FireworkExplosion {
            shape,
            colors,
            fade_colors,
            has_trail,
            has_twinkle,
        })
    }
}

impl FireworkShape {
    fn from_str(value: &str) -> Result<Self, SculkParseError> {
        match value {
            "small_ball" => Ok(FireworkShape::SmallBall),
            "large_ball" => Ok(FireworkShape::LargeBall),
            "star" => Ok(FireworkShape::Star),
            "creeper" => Ok(FireworkShape::Creeper),
            "burst" => Ok(FireworkShape::Burst),
            _ => Err(SculkParseError::InvalidField(value.into())),
        }
    }
}
