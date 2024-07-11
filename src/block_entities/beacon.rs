use std::borrow::Cow;

use simdnbt::Mutf8Str;

use crate::{
    traits::FromCompoundNbt,
    util::{get_optional_lock, get_optional_name, get_owned_optional_mutf8str},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Beacon<'a> {
    /// Optional. The name of this beacon in JSON text component, which appears when attempting to open it, while it is locked.
    ///
    /// `CustomName`
    pub custom_name: Option<Cow<'a, Mutf8Str>>,

    /// Optional. When not blank, prevents the container from being opened unless the opener is holding an item whose name matches this string.
    ///
    /// `Lock`
    pub lock: Option<Cow<'a, Mutf8Str>>,

    /// Optional. The primary effect selected, see Potion effects for resource locations. Cannot be set to an effect that beacons do not normally use. Although Regeneration cannot normally be chosen as the primary effect, setting this value to minecraft:regeneration works and even allows Regeneration II to be chosen as the secondary via the normal beacon GUI.
    pub primary_effect: Option<Cow<'a, Mutf8Str>>,

    /// Optional. The secondary effect selected, see Potion effects for resource locations. Cannot be set to an effect that beacons do not normally use. When set without a primary effect, does nothing. When set to the same as the primary, the effect is given at level 2 (the normally available behavior for 5 effects). When set to a different value than the primary (normally only Regeneration), gives the effect at level 1.
    pub secondary_effect: Option<Cow<'a, Mutf8Str>>,
}

impl<'a> FromCompoundNbt for Beacon<'a> {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let custom_name = get_optional_name(&nbt);
        let lock = get_optional_lock(&nbt);

        let primary_effect = get_owned_optional_mutf8str(&nbt, "primary_effect");
        let secondary_effect = get_owned_optional_mutf8str(&nbt, "secondary_effect");

        Ok(Beacon {
            custom_name,
            lock,
            primary_effect,
            secondary_effect,
        })
    }
}
