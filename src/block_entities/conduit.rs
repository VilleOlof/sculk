use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Conduit {
    #[serde(rename = "Target")]
    pub target: i128,
}
