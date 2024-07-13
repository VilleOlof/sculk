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

pub mod block_entities;
pub mod block_entity;
pub mod chunk;
pub mod color;
pub mod components;
pub mod entity;
mod error;
pub mod item;
mod kv;
mod player;
mod rarity;
mod traits;
mod util;
mod uuid;

// Re-export the modules.
pub use block_entities::variant::BlockEntityVariant;
pub use block_entities::BlockEntityKind;
pub use block_entity::BlockEntity;
pub use components::*;
pub use util::MC_VERSION;
