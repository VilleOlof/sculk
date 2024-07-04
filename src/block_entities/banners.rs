use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::banner_patterns::BannerPattern;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Banners<'a> {
    /// Optional. The name of this banner in JSON text component, which is used for showing the banner on a map.
    #[serde(borrow)]
    #[serde(rename = "CustomName")]
    pub custom_name: Option<Cow<'a, str>>,

    /// List of all patterns applied to the banner.
    #[serde(borrow)]
    pub patterns: Vec<BannerPattern<'a>>,
}
