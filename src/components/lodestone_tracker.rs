use std::borrow::Cow;

use simdnbt::Mutf8Str;

use crate::{
    traits::FromCompoundNbt,
    util::{get_int_array, get_owned_mutf8str},
};

#[derive(Debug, Clone, PartialEq)]
pub struct LodestoneTracker<'a> {
    /// Information about the lodestone. Optional. If not set, this compass spins randomly.
    pub target: Option<LodestoneTarget<'a>>,

    ///  If `true`, the component is removed when the lodestone is broken. If `false`, the component is kept. Defaults to `true`.
    pub tracked: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LodestoneTarget<'a> {
    /// The integer coordinates of the lodestone.
    pub pos: [i32; 3],

    /// The ID of the dimension of the lodestone.
    pub dimension: Cow<'a, Mutf8Str>,
}

impl<'a> FromCompoundNbt for LodestoneTracker<'a> {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let target = if let Some(t) = nbt.compound("tracker") {
            Some(LodestoneTarget::from_compound_nbt(&t)?)
        } else {
            None
        };

        let tracked = nbt.byte("tracked").map(|b| b != 0).unwrap_or(true);

        Ok(LodestoneTracker { target, tracked })
    }
}

impl<'a> FromCompoundNbt for LodestoneTarget<'a> {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let pos = get_int_array(&nbt, "pos")?;
        let pos = [pos[0], pos[1], pos[2]];

        let dimension = get_owned_mutf8str(&nbt, "dimension")?;

        Ok(LodestoneTarget { pos, dimension })
    }
}
