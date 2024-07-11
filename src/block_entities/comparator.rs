use crate::{error::SculkParseError, traits::FromCompoundNbt};

#[derive(Debug, Clone, PartialEq)]
pub struct Comparator {
    /// Represents the strength of the analog signal output of this redstone comparator.
    ///
    /// `OutputSignal`
    pub output_signal: i32,
}

impl FromCompoundNbt for Comparator {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        Ok(Comparator {
            output_signal: nbt
                .int("OutputSignal")
                .ok_or(SculkParseError::MissingField("OutputSignal".into()))?,
        })
    }
}
