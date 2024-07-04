use std::borrow::Cow;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Item<'a> {
    #[serde(borrow)]
    pub id: Cow<'a, str>,
}
