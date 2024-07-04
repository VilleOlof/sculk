use std::borrow::Cow;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Barrel<'a> {
    #[serde(borrow)]
    #[serde(rename = "CustomName")]
    pub custom_name: Option<Cow<'a, str>>,
    // pub items:
}
