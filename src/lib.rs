mod block_entities;
mod block_entity;
mod color;
mod components;
mod entity;
mod item;
mod serialize;
mod util;

// Re-export the modules.
pub use block_entity::BlockEntity;
pub use components::*;
pub use serialize::*;
pub use util::MC_VERSION;

#[cfg(test)]
mod tests {
    use fastnbt::nbt;
    use serialize::{deserialize_from_value, serialize_to_value};

    use super::*;

    /// TODO: Move like block entity & components test to their own seperate module or something.
    #[test]
    fn test() {
        let end_gateway_nbt = nbt!({
            "id": "minecraft:end_gateway",
            "keepPacked": false,
            "x": 52,
            "y": 14,
            "z": 62,
            "Age": 5,
            "ExactTeleport": false,
            "ExitPortal": {
                "X": 2,
                "Y": 4,
                "Z": 8
            },
            "components": {
                "minecraft:attribute_modifiers": {
                    "show_in_tooltip": true,
                    "modifiers": [
                        {
                            "type": "minecraft:attack_damage",
                            "slot": "armor",
                            "id": "custom:example",
                            "amount": 0.5f64,
                            "operation": "add_value"
                        }
                    ]
                }
            }
        });

        let block_entity = deserialize_from_value(&end_gateway_nbt).unwrap();

        println!("{:#?}", block_entity);

        println!();

        let serialized = serialize_to_value(&block_entity).unwrap();

        println!("{:#?}", serialized);
    }
}
