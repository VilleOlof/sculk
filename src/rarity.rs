use crate::error::SculkParseError;

/// Represents the rarity of an item.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum Rarity {
    /// Common rarity.
    Common,
    /// Uncommon rarity.
    Uncommon,
    /// Rare rarity.
    Rare,
    /// Epic rarity.
    Epic,
}

impl Rarity {
    /// Converts a string to a `Rarity`.
    pub fn from_str(s: &str) -> Result<Self, SculkParseError> {
        match s {
            "common" => Ok(Self::Common),
            "uncommon" => Ok(Self::Uncommon),
            "rare" => Ok(Self::Rare),
            "epic" => Ok(Self::Epic),
            _ => Err(SculkParseError::InvalidField(s.into())),
        }
    }
}
