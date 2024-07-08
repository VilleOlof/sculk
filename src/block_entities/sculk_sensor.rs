use serde::{Deserialize, Serialize};

use super::calibrated_sculk_sensor::Listener;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct SculkSensor<'a> {
    /// The frequency of the last vibration.
    pub last_vibration_frequency: i32,

    /// The vibration event listener for this sculk shrieker, sculk sensor, or calibrated sculk sensor.
    #[serde(borrow)]
    pub listener: Listener<'a>,
}

#[cfg(test)]
#[test]
fn test() {
    use fastnbt::nbt;

    let nbt = nbt!({
        "last_vibration_frequency": 0i32,
        "listener": {
            "event": {
                "distance": 5.3f32,
                "game_event": "minecraft:entity_born",
                "pos": [1.0f64, 2.0f64, 3.0f64],
                "projectile_owner": [I; 1, 2, 3, 4],
                "source": [I; 1, 2, 3, 4]
            },
            "event_delay": 20,
            "selector": {
                "tick": 10i64,
                "event": {
                    "distance": 5.3f32,
                    "game_event": "minecraft:entity_born",
                    "pos": [1.0f64, 2.0f64, 3.0f64],
                    "projectile_owner": [I; 1, 2, 3, 4],
                    "source": [I; 1, 2, 3, 4]
                }
            }
        }
    });

    let sculk_sensor: SculkSensor = fastnbt::from_value(&nbt).unwrap();

    assert_eq!(sculk_sensor.last_vibration_frequency, 0);
    assert_eq!(sculk_sensor.listener.event_delay, 20);
    assert_eq!(sculk_sensor.listener.selector.tick, 10);
    assert_eq!(sculk_sensor.listener.event.as_ref().unwrap().distance, 5.3);
    assert_eq!(
        sculk_sensor.listener.event.as_ref().unwrap().game_event,
        "minecraft:entity_born"
    );
    assert_eq!(
        sculk_sensor.listener.event.as_ref().unwrap().pos,
        [1.0, 2.0, 3.0]
    );
    assert_eq!(
        sculk_sensor
            .listener
            .event
            .as_ref()
            .unwrap()
            .projectile_owner,
        Some(79228162551157825753847955460)
    );
    assert_eq!(
        sculk_sensor.listener.event.as_ref().unwrap().source,
        Some(79228162551157825753847955460)
    );
    assert_eq!(sculk_sensor.listener.selector.event.distance, 5.3);
    assert_eq!(
        sculk_sensor.listener.selector.event.game_event,
        "minecraft:entity_born"
    );
    assert_eq!(sculk_sensor.listener.selector.event.pos, [1.0, 2.0, 3.0]);
    assert_eq!(
        sculk_sensor.listener.selector.event.projectile_owner,
        Some(79228162551157825753847955460)
    );
    assert_eq!(
        sculk_sensor.listener.selector.event.source,
        Some(79228162551157825753847955460)
    );

    let serialized_nbt = fastnbt::to_value(&sculk_sensor).unwrap();

    assert_eq!(nbt, serialized_nbt);
}
