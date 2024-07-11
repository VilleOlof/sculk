use std::borrow::Cow;

use simdnbt::Mutf8Str;

use crate::{traits::FromCompoundNbt, util::get_optional_name};

#[derive(Debug, Clone, PartialEq)]
pub struct EnchantingTable<'a> {
    /// Optional. The name of this container in JSON text component, which appears in its GUI where the default name ordinarily appears.
    ///
    /// `CustomName`
    pub custom_name: Option<Cow<'a, Mutf8Str>>,
}

impl<'a> FromCompoundNbt for EnchantingTable<'a> {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        Ok(EnchantingTable {
            custom_name: get_optional_name(&nbt),
        })
    }
}
