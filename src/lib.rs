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
pub use serialize::*;
pub use util::MC_VERSION;

// LIVE SAVER: https://github.com/feather-rs/feather/blob/main/feather/base/src/anvil/block_entity.rs
