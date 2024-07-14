//! # Sculk
//!
//! `sculk` is a crate for working with Minecraft data.  
//! Such as block entities, player inventories, entities and more.
//!
//! ## Examples
//! ```rust
//! use sculk::BlockEntity;
//! use sculk::BlockEntityVariant;
//!
//! let bytes = include_bytes!("structure_block.nbt");
//! let block_entity: BlockEntity = BlockEntity::from_bytes(bytes).unwrap();
//!
//! assert_eq!(block_entity.variant(), BlockEntityVariant::StructureBlock);
//! ```

mod block_entities;
mod block_entity;
mod chunk;
mod color;
mod components;
mod entity;
mod error;
mod item;
mod kv;
mod level;
mod map;
mod player;
mod rarity;
mod traits;
mod util;
mod uuid;

#[cfg(feature = "stats")]
mod statistics;

// Re-export the modules.
