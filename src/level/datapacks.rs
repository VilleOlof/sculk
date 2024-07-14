//! Lists what datapacks are enabled and disabled in a world.  
//! You may think at first this is the actual datapacks but nah nah.

use crate::{error::SculkParseError, traits::FromCompoundNbt};
use simdnbt::Mutf8Str;
use std::borrow::Cow;

/// What datapacks are enabled and disabled in a world.
#[derive(Debug, Clone, PartialEq)]
pub struct Datapacks<'a> {
    /// List of disabled datapacks.  
    /// `Disabled`
    pub disabled: Vec<Cow<'a, Mutf8Str>>,

    /// List of enabled datapacks. By default, this is populated with a single string "vanilla".  
    /// `Enabled`
    pub enabled: Vec<Cow<'a, Mutf8Str>>,
}

impl<'a> FromCompoundNbt for Datapacks<'a> {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let disabled = if let Some(nbt) = nbt.list("Disabled") {
            let list = nbt
                .strings()
                .ok_or(SculkParseError::InvalidField("Disabled".into()))?;

            let mut vec: Vec<Cow<'a, Mutf8Str>> = vec![];

            for item in list.iter() {
                vec.push(Cow::Owned((*item).to_owned()));
            }

            vec
        } else {
            vec![]
        };

        let enabled = if let Some(nbt) = nbt.list("Enabled") {
            let list = nbt
                .strings()
                .ok_or(SculkParseError::InvalidField("Enabled".into()))?;

            let mut vec: Vec<Cow<'a, Mutf8Str>> = vec![];

            for item in list.iter() {
                vec.push(Cow::Owned((*item).to_owned()));
            }

            vec
        } else {
            vec![]
        };

        Ok(Self { disabled, enabled })
    }
}
