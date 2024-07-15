//! Players specific abilities.  
//! Like if they can fly, if they are invulnerable, etc.

use crate::{traits::FromCompoundNbt, util::get_bool};

/// A player's abilities.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Abilities {
    /// if the player is currently flying.
    pub flying: bool,

    /// The flying speed, set to 0.05.  
    /// `flySpeed`
    pub fly_speed: f32,

    /// If true, the player can place blocks without depleting them. This is true for Creative mode, and false for other game modes.  
    /// `instabuild`
    pub insta_build: bool,

    /// Behavior is not the same as the invulnerable tag on other entities. If true, the player is immune to all damage and harmful effects except for void damage and /kill. Also, all mobs, whether hostile or not, are passive to the player. true when in Creative or Spectator mode, and false when in Survival or Adventure mode.
    pub invulnerable: bool,

    /// If true, the player can place blocks. true when in Creative or Survival mode, and false when in Spectator or Adventure mode.  
    /// `mayBuild`
    pub may_build: bool,

    /// If true, the player can fly and doesn't take fall damage. true when in Creative and Spectator modes, and false when in Survival and Adventure modes.  
    /// `mayfly`
    pub may_fly: bool,

    /// The walking speed, set to 0.1.  
    /// `walkSpeed`
    pub walk_speed: f32,
}

impl FromCompoundNbt for Abilities {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let flying = get_bool(&nbt, "flying");
        let fly_speed = nbt.float("flySpeed").unwrap_or(0.05);
        let insta_build = get_bool(&nbt, "instabuild");
        let invulnerable = get_bool(&nbt, "invulnerable");
        let may_build = get_bool(&nbt, "mayBuild");
        let may_fly = get_bool(&nbt, "mayfly");
        let walk_speed = nbt.float("walkSpeed").unwrap_or(0.1);

        Ok(Abilities {
            flying,
            fly_speed,
            insta_build,
            invulnerable,
            may_build,
            may_fly,
            walk_speed,
        })
    }
}
