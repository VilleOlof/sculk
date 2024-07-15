use crate::{
    components::banner_patterns::BannerPattern,
    traits::FromCompoundNbt,
    util::{get_optional_name, get_t_compound_vec},
};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Banner {
    /// Optional. The name of this banner in JSON text component, which is used for showing the banner on a map.
    ///
    /// `CustomName`
    pub custom_name: Option<String>,

    /// List of all patterns applied to the banner.
    pub patterns: Vec<BannerPattern>,
}

impl FromCompoundNbt for Banner {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let custom_name = get_optional_name(&nbt);

        let patterns: Vec<BannerPattern> =
            get_t_compound_vec(&nbt, "patterns", BannerPattern::from_compound_nbt)?;

        Ok(Banner {
            custom_name,
            patterns,
        })
    }
}
