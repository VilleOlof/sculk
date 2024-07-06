use std::borrow::Cow;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct EnchantingTable<'a> {
    /// Optional. The name of this container in JSON text component, which appears in its GUI where the default name ordinarily appears.
    #[serde(borrow)]
    pub custom_name: Option<Cow<'a, str>>,
}
