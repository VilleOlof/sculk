use std::borrow::Cow;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Beacon<'a> {
    /// Optional. The name of this beacon in JSON text component, which appears when attempting to open it, while it is locked.
    #[serde(borrow)]
    #[serde(rename = "CustomName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_name: Option<Cow<'a, str>>,

    /// Optional. When not blank, prevents the container from being opened unless the opener is holding an item whose name matches this string.
    #[serde(borrow)]
    #[serde(rename = "Lock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock: Option<Cow<'a, str>>,

    /// Optional. The primary effect selected, see Potion effects for resource locations. Cannot be set to an effect that beacons do not normally use. Although Regeneration cannot normally be chosen as the primary effect, setting this value to minecraft:regeneration works and even allows Regeneration II to be chosen as the secondary via the normal beacon GUI.
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_effect: Option<Cow<'a, str>>,

    /// Optional. The secondary effect selected, see Potion effects for resource locations. Cannot be set to an effect that beacons do not normally use. When set without a primary effect, does nothing. When set to the same as the primary, the effect is given at level 2 (the normally available behavior for 5 effects). When set to a different value than the primary (normally only Regeneration), gives the effect at level 1.
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_effect: Option<Cow<'a, str>>,
}

#[cfg(test)]
#[test]
fn test() {
    use fastnbt::nbt;

    let nbt = nbt!({
        "CustomName": "Hello, world!",
        "Lock": "key",
        "primary_effect": "minecraft:speed",
        "secondary_effect": "minecraft:regeneration"
    });

    let beacon: Beacon = fastnbt::from_value(&nbt).unwrap();

    assert_eq!(beacon.custom_name, Some(Cow::Borrowed("Hello, world!")));
    assert_eq!(beacon.lock, Some(Cow::Borrowed("key")));
    assert_eq!(
        beacon.primary_effect,
        Some(Cow::Borrowed("minecraft:speed"))
    );
    assert_eq!(
        beacon.secondary_effect,
        Some(Cow::Borrowed("minecraft:regeneration"))
    );

    let serialized_nbt = fastnbt::to_value(&beacon).unwrap();

    assert_eq!(nbt, serialized_nbt);
}
