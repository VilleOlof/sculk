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

// Pub mod declarations.
pub mod block_entities;
pub mod block_entity;
pub mod chunk;
pub mod color;
pub mod components;
pub mod entity;
pub mod error;
pub mod item;
pub mod level;
pub mod map;
pub mod player;
pub mod rarity;
pub mod traits;
pub mod uuid;

// Internal modules.
mod kv;
mod util;

#[cfg(feature = "stats")]
mod statistics;

// Re-export the modules.
pub use util::MC_VERSION;
