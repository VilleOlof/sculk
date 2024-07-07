use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Conduit {
    #[serde(rename = "Target")]
    pub target: i128,
}

#[cfg(test)]
#[test]
fn test() {
    use fastnbt::nbt;

    let nbt = nbt!({
        "Target": [I; 1, 2, 3, 4]
    });

    let conduit: Conduit = fastnbt::from_value(&nbt).unwrap();

    assert_eq!(conduit.target, 79228162551157825753847955460);

    let nbt = fastnbt::to_value(&conduit).unwrap();

    assert_eq!(nbt, nbt);
}
