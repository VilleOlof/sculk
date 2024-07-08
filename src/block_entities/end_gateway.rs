use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct EndGateway {
    #[serde(rename = "Age")]
    pub age: i32,

    #[serde(rename = "ExactTeleport")]
    #[serde(deserialize_with = "crate::util::i8_to_bool")]
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

#[cfg(test)]
#[test]
fn test() {
    use fastnbt::nbt;

    let nbt = nbt!({
        "Age": 5,
        "ExactTeleport": 1i8,
        "ExitPortal": {
            "X": 1,
            "Y": 2,
            "Z": 3
        }
    });

    let end_gateway: EndGateway = fastnbt::from_value(&nbt).unwrap();

    assert_eq!(end_gateway.age, 5);
    assert_eq!(end_gateway.exact_teleport, true);
    assert_eq!(end_gateway.exit_portal.x, 1);
    assert_eq!(end_gateway.exit_portal.y, 2);
    assert_eq!(end_gateway.exit_portal.z, 3);

    let serialized_nbt = fastnbt::to_value(&end_gateway).unwrap();

    assert_eq!(nbt, serialized_nbt);
}
