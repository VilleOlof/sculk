use crate::{traits::FromCompoundNbt, util::get_optional_name};

#[derive(Debug, Clone, PartialEq)]
pub struct EnchantingTable {
    /// Optional. The name of this container in JSON text component, which appears in its GUI where the default name ordinarily appears.
    ///
    /// `CustomName`
    pub custom_name: Option<String>,
}

impl FromCompoundNbt for EnchantingTable {
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
