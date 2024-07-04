use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// The base dye color of the banner applied on a shield.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct BaseColor<'a>(Cow<'a, str>);
