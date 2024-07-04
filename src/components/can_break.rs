use std::borrow::Cow;

use fastnbt::Value;
use serde::{Deserialize, Serialize};

use super::block_state::BlockState;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum CanBreak<'a> {
    List {
        /// A list of block predicates to match.
        #[serde(borrow)]
        predicates: Vec<Predicate<'a>>,

        /// Show or hide the "Can break" line on this item's tooltip. Defaults to true.
        #[serde(default = "crate::util::default_as_true")]
        show_in_tooltip: bool,
    },

    Single {
        /// Can be a block ID or a block tag with a #, or a list of block IDs.
        #[serde(borrow)]
        blocks: Blocks<'a>,

        /// Block entity NBT to match.
        // TODO: BlockEntity nbt
        nbt: Option<Value>,

        /// The block state properties to match.
        state: Option<BlockState<'a>>,

        /// Show or hide the "Can break" line on this item's tooltip. Defaults to true.
        #[serde(default = "crate::util::default_as_true")]
        show_in_tooltip: bool,
    },
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Predicate<'a> {
    /// Can be a block ID or a block tag with a #, or a list of block IDs.
    #[serde(borrow)]
    pub blocks: Blocks<'a>,

    /// Block entity NBT to match.
    // TODO: BlockEntity nbt
    pub nbt: Option<Value>,

    /// The block state properties to match.
    pub state: Option<BlockState<'a>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum Blocks<'a> {
    #[serde(borrow)]
    Block(Cow<'a, str>),

    #[serde(borrow)]
    Blocks(Vec<Cow<'a, str>>),
}
