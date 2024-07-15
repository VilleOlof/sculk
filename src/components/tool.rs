//! Tool component.

use crate::{error::SculkParseError, traits::FromCompoundNbt};

/// A tool that can be used to mine blocks.
#[derive(Debug, Clone, PartialEq)]
pub struct Tool {
    /// The default mining speed of this tool, used if no rules override it. Defaults to 1.0.
    pub default_mining_speed: f32,

    /// The amount of durability to remove each time a block is broken with this tool. Must be a non-negative integer.
    pub damage_per_block: i32,

    ///  A list of rules for the blocks that this tool has a special behavior with.
    pub rules: Vec<ToolRules>,
}

/// Rules on how a tool behaves with certain blocks.
#[derive(Debug, Clone, PartialEq)]
pub struct ToolRules {
    /// The blocks to match with. Can be a block ID or a block tag with a #, or a list of block IDs.  
    /// **NOTE:** This is always a list here, even if it only contains one block.
    pub blocks: Vec<String>,

    ///  If the blocks match, overrides the default mining speed. Optional.
    pub speed: Option<f32>,

    /// If the blocks match, overrides whether or not this tool is considered correct to mine at its most efficient speed, and to drop items if the block's loot table requires it. Optional.
    pub correct_for_drops: Option<bool>,
}

impl FromCompoundNbt for Tool {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let default_mining_speed = nbt.float("default_mining_speed").unwrap_or(1.0);
        let damage_per_block = nbt
            .int("damage_per_block")
            .ok_or(SculkParseError::MissingField("damage_per_block".into()))?;

        let rules = if let Some(rules) = nbt.list("rules") {
            let nbt_rules = rules
                .compounds()
                .ok_or(SculkParseError::InvalidField("rules".into()))?;

            let mut rules = vec![];

            for rule in nbt_rules {
                rules.push(ToolRules::from_compound_nbt(&rule)?);
            }

            rules
        } else {
            return Err(SculkParseError::MissingField("rules".into()));
        };

        Ok(Tool {
            default_mining_speed,
            damage_per_block,
            rules,
        })
    }
}

impl FromCompoundNbt for ToolRules {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let blocks = if let Some(string) = nbt.string("blocks") {
            // single block id
            vec![string.to_string()]
        } else if let Some(list) = nbt.list("blocks") {
            // multiple block ids
            let list = list
                .strings()
                .ok_or(SculkParseError::InvalidField("blocks".into()))?;

            let mut blocks = vec![];

            for string in list {
                blocks.push((*string).to_string());
            }

            blocks
        } else {
            return Err(SculkParseError::MissingField("blocks".into()));
        };

        let speed = nbt.float("speed");
        let correct_for_drops = nbt.byte("correct_for_drops").map(|b| b != 0);

        Ok(ToolRules {
            blocks,
            speed,
            correct_for_drops,
        })
    }
}
