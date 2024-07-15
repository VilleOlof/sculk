use crate::{error::SculkParseError, traits::FromCompoundNbt, util::get_int_array};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SculkCatalyst {
    /// List of sculk charge clusters associated with the sculk catalyst.
    cursors: Vec<Cursor>,
}

/// A sculk charge cluster. Each cluster is stored within a single sculk block.

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cursor {
    /// How many sculk charges are being carried in the cluster.
    pub charge: i32,

    /// List of three integers representing the coordinates of the cluster.
    pub pos: [i32; 3],

    /// 1 or 0 (true/false) - Not to be confused with update_delay. This controls the decay of the cluster. If this is true, then the cluster was spread from a sculk block or sculk vein, and this cluster can only spread to sculk, sculk veins, and blocks with the sculk_replaceable tag set to true. If this is false, then all the sculk charges disappear when the cluster discharges to a block that is not in the sculk family.
    ///
    /// *No clue why this is an i32, wiki says its an int.*  
    pub decay_delay: i32,

    /// Delay in game ticks until the cluster begins to discharge (or travel) after being created. Usually starts at 1 game tick when a cluster discharges to a new block.
    pub update_delay: i32,

    /// If the block to replace is an air or water block, the block is replaced with sculk veins, and the faces where the sculk veins are placed are also stored in their block state. The sculk veins never grow directly on the faces of a sculk block. The same thing is done to any air or water blocks that are adjacent to blocks that are adjacent to this sculk block, if sculk veins can't grow in the blocks adjacent to this sculk block without growing directly on the faces of sculk blocks.
    // TODO: Research what this value is.
    #[cfg_attr(feature = "serde", serde(skip))]
    pub facings: Vec<simdnbt::owned::NbtCompound>,
}

impl FromCompoundNbt for SculkCatalyst {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let cursors_list = nbt
            .list("cursors")
            .ok_or(SculkParseError::MissingField("cursors".into()))?;

        let cursors = if let Some(cursors) = cursors_list.compounds() {
            cursors
                .into_iter()
                .map(|c| Cursor::from_compound_nbt(&c))
                .collect::<Result<Vec<Cursor>, SculkParseError>>()?
        } else {
            vec![]
        };

        Ok(SculkCatalyst { cursors })
    }
}

impl FromCompoundNbt for Cursor {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let charge = nbt
            .int("charge")
            .ok_or(SculkParseError::MissingField("charge".into()))?;

        let pos = get_int_array(&nbt, "pos").and_then(|arr| {
            if arr.len() == 3 {
                Ok([arr[0], arr[1], arr[2]])
            } else {
                Err(SculkParseError::InvalidField("pos".into()))
            }
        })?;

        let decay_delay = nbt
            .int("decay_delay")
            .ok_or(SculkParseError::MissingField("decay_delay".into()))?;

        let update_delay = nbt
            .int("update_delay")
            .ok_or(SculkParseError::MissingField("update_delay".into()))?;

        let facings = if let Some(facings) = nbt.list("facings") {
            let compounds = facings
                .compounds()
                .ok_or(SculkParseError::InvalidField("facings".into()))?;

            compounds.into_iter().map(|c| c.to_owned()).collect()
        } else {
            vec![]
        };

        Ok(Cursor {
            charge,
            pos,
            decay_delay,
            update_delay,
            facings,
        })
    }
}
