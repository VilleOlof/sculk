//! Can break block component.

use super::block_state::BlockState;
use crate::{
    block_entity::BlockEntity, error::SculkParseError, traits::FromCompoundNbt, util::get_t_list,
};

/// If an item can break blocks.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CanBreak {
    /// Multiple block predicates to match.
    List {
        /// A list of block predicates to match.
        predicates: Vec<Predicate>,

        /// Show or hide the "Can break" line on this item's tooltip. Defaults to true.
        show_in_tooltip: bool,
    },

    /// Single block predicate to match.
    Single {
        /// Can be a block ID or a block tag with a #, or a list of block IDs.
        blocks: Blocks,

        /// Block entity NBT to match.
        nbt: Option<BlockEntity>,

        /// The block state properties to match.
        state: Option<BlockState>,

        /// Show or hide the "Can break" line on this item's tooltip. Defaults to true.
        show_in_tooltip: bool,
    },
}

/// A block predicate to match.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Predicate {
    /// Can be a block ID or a block tag with a #, or a list of block IDs.
    pub blocks: Blocks,

    /// Block entity NBT to match.
    pub nbt: Option<BlockEntity>,

    /// The block state properties to match.
    pub state: Option<BlockState>,
}

/// A list of block IDs or block tags with a #.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Blocks {
    /// A single block ID or block tag with a #.
    Block(String),

    /// A list of block IDs or block tags with a #.
    Blocks(Vec<String>),
}

impl FromCompoundNbt for CanBreak {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        if let Some(_) = nbt.compound("state") {
            // Single
            let blocks = Blocks::from_compound_nbt(nbt)?;
            let struct_nbt = if let Some(nbt) = nbt.compound("nbt") {
                Some(BlockEntity::from_compound_nbt(&nbt)?)
            } else {
                None
            };
            let state = if let Some(nbt) = nbt.compound("state") {
                Some(BlockState::from_compound_nbt(&nbt)?)
            } else {
                None
            };
            let show_in_tooltip = nbt.byte("show_in_tooltip").map(|b| b != 0).unwrap_or(true);

            return Ok(CanBreak::Single {
                blocks,
                nbt: struct_nbt,
                state,
                show_in_tooltip,
            });
        } else if let Some(_) = nbt.list("predicates") {
            // List
            let predicates = get_t_list(
                &nbt.list("predicates")
                    .ok_or(SculkParseError::InvalidField("predicates".into()))?,
                "predicates",
                Predicate::from_compound_nbt,
            )?;

            let show_in_tooltip = nbt.byte("show_in_tooltip").map(|b| b != 0).unwrap_or(true);

            return Ok(CanBreak::List {
                predicates,
                show_in_tooltip,
            });
        } else {
            return Err(SculkParseError::MissingField("state or predicates".into()));
        }
    }
}

impl FromCompoundNbt for Predicate {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let blocks = Blocks::from_compound_nbt(nbt)?;

        let struct_nbt = if let Some(nbt) = nbt.compound("nbt") {
            Some(BlockEntity::from_compound_nbt(&nbt)?)
        } else {
            None
        };

        let state = if let Some(nbt) = nbt.compound("state") {
            Some(BlockState::from_compound_nbt(&nbt)?)
        } else {
            None
        };

        Ok(Predicate {
            blocks,
            nbt: struct_nbt,
            state,
        })
    }
}

impl FromCompoundNbt for Blocks {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        if let Some(string) = nbt.string("blocks") {
            return Ok(Blocks::Block(string.to_string()));
        } else if let Some(list) = nbt.list("blocks") {
            let blocks = list
                .strings()
                .ok_or(SculkParseError::InvalidField("blocks".into()))?
                .into_iter()
                .map(|string| (*string).to_string())
                .collect::<Vec<String>>();

            return Ok(Blocks::Blocks(blocks));
        } else {
            return Err(SculkParseError::MissingField("blocks".into()));
        };
    }
}
