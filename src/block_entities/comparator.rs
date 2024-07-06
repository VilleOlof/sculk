use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Comparator {
    /// Represents the strength of the analog signal output of this redstone comparator.
    #[serde(rename = "OutputSignal")]
    pub output_signal: i32,
}
