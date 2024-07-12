use std::borrow::Cow;

use simdnbt::Mutf8Str;

use crate::{
    error::SculkParseError,
    traits::FromCompoundNbt,
    util::{get_optional_components, get_owned_mutf8str, get_t_compound_vec},
    Components,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Food<'a> {
    /// The number of food points restored by this item when eaten. Must be a non-negative integer.
    pub nutrition: i32,

    /// The amount of saturation restored by this item when eaten.
    pub saturation: f32,

    ///  If true, this item can be eaten even if the player is not hungry. Defaults to false.
    pub can_always_eat: bool,

    /// The number of seconds taken by this item to be eaten. Defaults to 1.6.
    pub eat_seconds: f32,

    /// The item to replace this item with when it is eaten.
    pub using_converts_to: Option<FoodConvertedItem<'a>>,

    ///  A list of effects applied by this item when eaten.
    pub effects: Vec<Effect<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FoodConvertedItem<'a> {
    /// The resource location of the item. Must not be air
    pub id: Cow<'a, Mutf8Str>,

    /// Optional map of data components. Additional information about the item.
    pub components: Option<Components<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Effect<'a> {
    /// A single effect.
    pub effect: EffectDetails<'a>,

    /// The chance for the effect to be applied. Must be a positive float between 0 and 1. Defaults to 1.
    pub probability: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EffectDetails<'a> {
    /// The ID of the effect.
    pub id: Cow<'a, Mutf8Str>,

    /// The amplifier of the effect, with level I having value 0. Optional, defaults to 0.
    pub amplifier: Option<i8>,

    ///  The duration of the effect in ticks. Values 0 or lower are treated as 1. Optional, and defaults to 1 tick.
    pub duration: Option<i32>,

    ///  Whether or not this is an effect provided by a beacon and therefore should be less intrusive on the screen. Optional, defaults to false.
    pub ambient: Option<bool>,

    /// Whether or not this effect produces particles. Optional, defaults to true.
    pub show_particles: Option<bool>,

    /// Whether or not an icon should be shown for this effect. Defaults to true.
    pub show_icon: Option<bool>,
}

impl<'a> FromCompoundNbt for Food<'a> {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let nutrition = nbt
            .int("nutrition")
            .ok_or(SculkParseError::MissingField("nutrition".into()))?;
        let saturation = nbt
            .float("saturation")
            .ok_or(SculkParseError::MissingField("saturation".into()))?;
        let can_always_eat = nbt.byte("can_always_eat").map(|b| b != 0).unwrap_or(false);
        let eat_seconds = nbt.float("eat_seconds").unwrap_or(1.6);

        let using_converts_to = if let Some(nbt) = nbt.compound("using_converts_to") {
            Some(FoodConvertedItem::from_compound_nbt(&nbt)?)
        } else {
            None
        };

        let effects = get_t_compound_vec(&nbt, "effects", Effect::from_compound_nbt)?;

        Ok(Food {
            nutrition,
            saturation,
            can_always_eat,
            eat_seconds,
            using_converts_to,
            effects,
        })
    }
}

impl<'a> FromCompoundNbt for FoodConvertedItem<'a> {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let id = get_owned_mutf8str(&nbt, "id")?;
        let components = get_optional_components(&nbt)?;

        Ok(FoodConvertedItem { id, components })
    }
}

impl<'a> FromCompoundNbt for Effect<'a> {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let effect = nbt
            .compound("effect")
            .map(|e| EffectDetails::from_compound_nbt(&e))
            .ok_or(SculkParseError::MissingField("effect".into()))??;

        let probability = nbt.float("probability").unwrap_or(1.0);

        Ok(Effect {
            effect,
            probability,
        })
    }
}

impl<'a> FromCompoundNbt for EffectDetails<'a> {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let id = get_owned_mutf8str(&nbt, "id")?;
        let amplifier = nbt.byte("amplifier");
        let duration = nbt.int("duration");
        let ambient = nbt.byte("ambient").map(|b| b != 0);
        let show_particles = nbt.byte("show_particles").map(|b| b != 0);
        let show_icon = nbt.byte("show_icon").map(|b| b != 0);

        Ok(EffectDetails {
            id,
            amplifier,
            duration,
            ambient,
            show_particles,
            show_icon,
        })
    }
}
