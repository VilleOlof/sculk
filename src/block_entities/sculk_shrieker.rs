use serde::{Deserialize, Serialize};

use super::calibrated_sculk_sensor::Listener;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct SculkShrieker<'a> {
    /// The vibration event listener of the sculk shrieker, sculk sensor, and calibrated sculk sensor.
    // TODO: This one is hella sketch, wiki just says a bunch of `Unknown` and it seems old, gonna assume its like the other sculk blocks.
    #[serde(borrow)]
    pub listener: Listener<'a>,
}

#[cfg(test)]
#[test]
fn test() {
    use fastnbt::nbt;

    let nbt = nbt!({
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

    let sculk_shrieker: SculkShrieker = fastnbt::from_value(&nbt).unwrap();

    assert_eq!(sculk_shrieker.listener.event_delay, 20);
    assert_eq!(sculk_shrieker.listener.selector.tick, 10);
    assert_eq!(
        sculk_shrieker.listener.event.as_ref().unwrap().distance,
        5.3
    );
    assert_eq!(
        sculk_shrieker.listener.event.as_ref().unwrap().game_event,
        "minecraft:entity_born"
    );
    assert_eq!(
        sculk_shrieker.listener.event.as_ref().unwrap().pos,
        [1.0, 2.0, 3.0]
    );
    assert_eq!(
        sculk_shrieker
            .listener
            .event
            .as_ref()
            .unwrap()
            .projectile_owner,
        Some(79228162551157825753847955460)
    );
    assert_eq!(
        sculk_shrieker.listener.event.as_ref().unwrap().source,
        Some(79228162551157825753847955460)
    );
    assert_eq!(sculk_shrieker.listener.selector.event.distance, 5.3);
    assert_eq!(
        sculk_shrieker.listener.selector.event.game_event,
        "minecraft:entity_born"
    );
    assert_eq!(sculk_shrieker.listener.selector.event.pos, [1.0, 2.0, 3.0]);
    assert_eq!(
        sculk_shrieker.listener.selector.event.projectile_owner,
        Some(79228162551157825753847955460)
    );
    assert_eq!(
        sculk_shrieker.listener.selector.event.source,
        Some(79228162551157825753847955460)
    );

    let nbt = fastnbt::to_value(&sculk_shrieker).unwrap();

    assert_eq!(nbt, nbt);
}
