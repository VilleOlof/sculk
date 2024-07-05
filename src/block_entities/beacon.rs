use std::borrow::Cow;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Beacon<'a> {
    /// Optional. The name of this beacon in JSON text component, which appears when attempting to open it, while it is locked.
    #[serde(borrow)]
    #[serde(rename = "CustomName")]
    pub custom_name: Option<Cow<'a, str>>,

    /// Optional. When not blank, prevents the container from being opened unless the opener is holding an item whose name matches this string.
    #[serde(borrow)]
    #[serde(rename = "Lock")]
    pub lock: Option<Cow<'a, str>>,

    /// Optional. The primary effect selected, see Potion effects for resource locations. Cannot be set to an effect that beacons do not normally use. Although Regeneration cannot normally be chosen as the primary effect, setting this value to minecraft:regeneration works and even allows Regeneration II to be chosen as the secondary via the normal beacon GUI.
    #[serde(borrow)]
    pub primary_effect: Option<Cow<'a, str>>,

    /// Optional. The secondary effect selected, see Potion effects for resource locations. Cannot be set to an effect that beacons do not normally use. When set without a primary effect, does nothing. When set to the same as the primary, the effect is given at level 2 (the normally available behavior for 5 effects). When set to a different value than the primary (normally only Regeneration), gives the effect at level 1.
    #[serde(borrow)]
    pub secondary_effect: Option<Cow<'a, str>>,
}
