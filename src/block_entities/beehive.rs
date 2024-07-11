use crate::{
    bees::Bee,
    error::SculkParseError,
    traits::FromCompoundNbt,
    util::{get_int_array, get_t_compound_vec},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Beehive<'a> {
    /// Entities currently in the hive.
    bees: Vec<Bee<'a>>,

    /// Stores the flower block location, as 3 integers, so other bees can go to it.
    flower_pos: [i32; 3],
}

impl<'a> FromCompoundNbt for Beehive<'a> {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let bees = get_t_compound_vec(&nbt, "bees", Bee::from_compound_nbt)?;
        let flower_pos = get_int_array(&nbt, "flower_pos").and_then(|arr| {
            if arr.len() == 3 {
                Ok([arr[0], arr[1], arr[2]])
            } else {
                Err(SculkParseError::InvalidField("flower_pos".into()))
            }
        })?;

        Ok(Beehive { bees, flower_pos })
    }
}
