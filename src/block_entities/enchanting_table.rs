use std::borrow::Cow;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct EnchantingTable<'a> {
    /// Optional. The name of this container in JSON text component, which appears in its GUI where the default name ordinarily appears.
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_name: Option<Cow<'a, str>>,
}

#[cfg(test)]
#[test]
fn test() {
    use fastnbt::nbt;

    let nbt = nbt!({
        "CustomName": "Enchanting Table"
    });

    let enchanting_table: EnchantingTable = fastnbt::from_value(&nbt).unwrap();

    assert_eq!(
        enchanting_table.custom_name,
        Some(Cow::Borrowed("Enchanting Table"))
    );

    let nbt = fastnbt::to_value(&enchanting_table).unwrap();

    assert_eq!(nbt, nbt);
}
