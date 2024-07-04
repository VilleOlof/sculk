use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct EndGateway {
    #[serde(rename = "Age")]
    pub age: i32,
    #[serde(rename = "ExactTeleport")]
    pub exact_teleport: bool,
    #[serde(rename = "ExitPortal")]
    pub exit_portal: ExitPortal,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ExitPortal {
    #[serde(rename = "X")]
    pub x: i32,
    #[serde(rename = "Y")]
    pub y: i32,
    #[serde(rename = "Z")]
    pub z: i32,
}
