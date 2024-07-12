use crate::{
    bees::Bee,
    traits::FromCompoundNbt,
    util::{get_int_array, get_t_compound_vec},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Beehive<'a> {
    /// Entities currently in the hive.
    bees: Vec<Bee<'a>>,

    /// Stores the flower block location, as 3 integers, so other bees can go to it.
    flower_pos: Vec<i32>,
}

impl<'a> FromCompoundNbt for Beehive<'a> {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let bees = get_t_compound_vec(&nbt, "bees", Bee::from_compound_nbt)?;
        let flower_pos = get_int_array(&nbt, "flower_pos")?;

        Ok(Beehive { bees, flower_pos })
    }
}
