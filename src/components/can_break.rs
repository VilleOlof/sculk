use super::block_state::BlockState;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq)]
pub enum CanBreak<'a> {
    List {
        /// A list of block predicates to match.
        predicates: Vec<Predicate<'a>>,

        /// Show or hide the "Can break" line on this item's tooltip. Defaults to true.
        show_in_tooltip: bool,
    },

    Single {
        /// Can be a block ID or a block tag with a #, or a list of block IDs.
        blocks: Blocks<'a>,

        /// Block entity NBT to match.
        // TODO: BlockEntity nbt
        nbt: Option<simdnbt::owned::NbtCompound>,

        /// The block state properties to match.
        state: Option<BlockState<'a>>,

        /// Show or hide the "Can break" line on this item's tooltip. Defaults to true.
        show_in_tooltip: bool,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Predicate<'a> {
    /// Can be a block ID or a block tag with a #, or a list of block IDs.
    pub blocks: Blocks<'a>,

    /// Block entity NBT to match.
    // TODO: BlockEntity nbt
    pub nbt: Option<simdnbt::owned::NbtCompound>,

    /// The block state properties to match.
    pub state: Option<BlockState<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Blocks<'a> {
    Block(Cow<'a, str>),

    Blocks(Vec<Cow<'a, str>>),
}
