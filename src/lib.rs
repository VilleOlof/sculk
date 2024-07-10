//! # Sculk
//!
//! `sculk` is a crate for working with Minecraft data.  
//! Such as block entities, player inventories, entities and more.
//!
//! ## Examples
//! ```rust
//! use sculk::BlockEntity;
//! use sculk::BlockEntityVariant;
//! use sculk::block_entity::deserialize_from_bytes;
//!
//! let bytes = include_bytes!("structure_block.nbt");
//! let block_entity: BlockEntity = deserialize_from_bytes(bytes).unwrap();
//!
//! assert_eq!(block_entity.variant(), BlockEntityVariant::StructureBlock);
//! ```
//! ## IMPORTANT
//! When deserializing block entities, use the `deserialize_from_*` functions.  
//! Otherwise the `block_entity.base.id` field will be empty.

pub mod block_entities;
pub mod block_entity;
pub mod color;
pub mod components;
pub mod entity;
pub mod item;
mod serialize;
mod util;

// Re-export the modules.
pub use block_entities::variant::BlockEntityVariant;
pub use block_entities::BlockEntityData;
pub use block_entity::BlockEntity;
pub use components::*;
pub use util::MC_VERSION;

// LIVE SAVER: https://github.com/feather-rs/feather/blob/main/feather/base/src/anvil/block_entity.rs
