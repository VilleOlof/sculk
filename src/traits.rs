use crate::error::SculkParseError;
use simdnbt::borrow::{Nbt, NbtCompound};

/// Often used for the root struct of a deserialization.
pub trait FromNbt {
    #[allow(unused_variables)]
    fn from_nbt(nbt: Nbt) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}

/// Used on any field inside a struct that can be deserialized from an Nbt.
pub trait FromCompoundNbt<Output = Self> {
    #[allow(unused_variables)]
    fn from_compound_nbt(nbt: &NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}
