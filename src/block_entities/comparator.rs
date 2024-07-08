use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Comparator {
    /// Represents the strength of the analog signal output of this redstone comparator.
    #[serde(rename = "OutputSignal")]
    pub output_signal: i32,
}

#[cfg(test)]
#[test]
fn test() {
    use fastnbt::nbt;

    let nbt = nbt!({
        "OutputSignal": 5i32
    });

    let comparator: Comparator = fastnbt::from_value(&nbt).unwrap();

    assert_eq!(comparator.output_signal, 5);

    let serialized_nbt = fastnbt::to_value(&comparator).unwrap();

    assert_eq!(nbt, serialized_nbt);
}
