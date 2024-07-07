mod block_entities;
mod block_entity;
mod color;
mod components;
mod entity;
mod item;
mod serialize;
mod util;

// Re-export the modules.
pub use block_entities::variant::BlockEntityVariant;
pub use block_entities::BlockEntityData;
pub use block_entity::BlockEntity;
pub use components::*;
pub use serialize::*;
pub use util::MC_VERSION;

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use fastnbt::nbt;
    use serialize::{deserialize_from_value, serialize_to_value};

    use super::*;

    /// TODO: Move like block entity & components test to their own seperate module or something.
    #[test]
    fn test() {
        let test_data = nbt!({
            "id": "minecraft:chest",
            "keepPacked": false,
            "x": 12,
            "y": 163,
            "z": 2,
            "components": {},
            "CustomName": "MY Chest",
            "Items": [
                {
                    "Slot": 0,
                    "id": "minecraft:stone",
                    "Count": 1,
                }
            ],
            "LootTable": "minecraft:chests/simple_dungeon",
        });

        let start = Instant::now();
        let block_entity = deserialize_from_value(&test_data).unwrap();
        println!("Deserialization took: {:?}", start.elapsed());

        println!("{:#?}", block_entity);

        println!("Variant: {:?}", block_entity.variant());

        println!();

        let serialized = serialize_to_value(&block_entity).unwrap();

        println!("{:#?}", serialized);
    }
}

// LIVE SAVER: https://github.com/feather-rs/feather/blob/main/feather/base/src/anvil/block_entity.rs
