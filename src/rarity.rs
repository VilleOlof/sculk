use crate::error::SculkParseError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
}

impl Rarity {
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
