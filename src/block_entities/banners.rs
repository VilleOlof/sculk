use simdnbt::Mutf8Str;

use crate::{
    banner_patterns::BannerPattern,
    traits::FromCompoundNbt,
    util::{get_optional_name, get_t_compound_vec},
};
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq)]
pub struct Banner<'a> {
    /// Optional. The name of this banner in JSON text component, which is used for showing the banner on a map.
    ///
    /// `CustomName`
    pub custom_name: Option<Cow<'a, Mutf8Str>>,

    /// List of all patterns applied to the banner.
    pub patterns: Vec<BannerPattern<'a>>,
}

impl<'a> FromCompoundNbt for Banner<'a> {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let custom_name = get_optional_name(&nbt);

        let patterns: Vec<BannerPattern<'a>> =
            get_t_compound_vec(&nbt, "patterns", BannerPattern::from_compound_nbt)?;

        Ok(Banner {
            custom_name,
            patterns,
        })
    }
}
