//! Lists what datapacks are enabled and disabled in a world.  
//! You may think at first this is the actual datapacks but nah nah.

use crate::{error::SculkParseError, traits::FromCompoundNbt};

/// What datapacks are enabled and disabled in a world.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Datapacks {
    /// List of disabled datapacks.  
    /// `Disabled`
    pub disabled: Vec<String>,

    /// List of enabled datapacks. By default, this is populated with a single string "vanilla".  
    /// `Enabled`
    pub enabled: Vec<String>,
}

impl FromCompoundNbt for Datapacks {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let disabled = if let Some(nbt) = nbt.list("Disabled") {
            let list = nbt.strings().unwrap_or_default();

            let mut vec: Vec<String> = vec![];

            for item in list.iter() {
                vec.push((*item).to_string());
            }

            vec
        } else {
            vec![]
        };

        let enabled = if let Some(nbt) = nbt.list("Enabled") {
            let list = nbt.strings().unwrap_or_default();

            let mut vec: Vec<String> = vec![];

            for item in list.iter() {
                vec.push((*item).to_string());
            }

            vec
        } else {
            vec![]
        };

        Ok(Self { disabled, enabled })
    }
}
