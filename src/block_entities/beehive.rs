use serde::{Deserialize, Serialize};

use crate::bees::Bee;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Beehive<'a> {
    /// Entities currently in the hive.
    #[serde(borrow)]
    #[serde(default)]
    bees: Vec<Bee<'a>>,

    /// Stores the flower block location, as 3 integers, so other bees can go to it.
    flower_pos: [i32; 3],
}

#[cfg(test)]
#[test]
fn test() {
    use std::borrow::Cow;

    use fastnbt::nbt;

    let nbt = nbt!({
        "bees": [
            {
                "entity_data": {
                    "Air": 32i16,
                    "CustomName": "Cool Bee",
                    "FallDistance": 0.0f32,
                    "Fire": -1i16,
                    "Glowing": false,
                    "HasVisualFire": true,
                    "id": "minecraft:bee",
                    "Invulnerable": false,
                    "Motion": [0.0f64, 0.0f64, 0.0f64],
                    "NoGravity": false,
                    "OnGround": true,
                    "PortalCooldown": 0,
                    "Pos": [5.5f64, 6.3f64, 2.1f64],
                    "Rotation": [0.0f32, 0.0f32],
                    "UUID": [I; 1, 2, 3, 4]
                },
                "min_ticks_in_hive": 5,
                "ticks_in_hive": 10
            }
        ],
        "flower_pos": [1, 2, 3]
    });

    let beehive: Beehive = fastnbt::from_value(&nbt).unwrap();

    assert_eq!(beehive.bees.len(), 1);
    assert_eq!(beehive.bees[0].min_ticks_in_hive, 5);
    assert_eq!(beehive.bees[0].ticks_in_hive, 10);
    assert_eq!(
        beehive.bees[0].entity_data.custom_name,
        Some(Cow::Borrowed("Cool Bee"))
    );
    assert_eq!(beehive.bees[0].entity_data.id, "minecraft:bee");
    assert_eq!(
        beehive.bees[0].entity_data.uuid,
        79228162551157825753847955460
    );
    assert_eq!(beehive.flower_pos, [1, 2, 3]);

    let serialized_nbt = fastnbt::to_value(&beehive).unwrap();

    assert_eq!(nbt, serialized_nbt);
}
