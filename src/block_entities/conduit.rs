use crate::{error::SculkParseError, traits::FromCompoundNbt, uuid::Uuid};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Conduit {
    /// `Target`
    pub target: Option<Uuid>,
}

impl FromCompoundNbt for Conduit {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        Ok(Conduit {
            target: nbt.int_array("target").map(Uuid::from),
        })
    }
}
