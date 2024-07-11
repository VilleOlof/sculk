use std::borrow::Cow;

use simdnbt::Mutf8Str;

use crate::{
    error::SculkParseError,
    traits::FromCompoundNbt,
    util::{get_bool, get_optional_name, get_owned_mutf8str, get_t_compound_vec},
    uuid::Uuid,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Entity<'a> {
    ///  How much air the entity has, in game ticks. Decreases when unable to breathe (except suffocating in a block). Increases when it can breathe.  Air being <= -20 game ticks (while still unable to breathe) on a given game tick causes the entity to immediately lose 1 health to drowning damage. This resets  Air to 0 game ticks. Most mobs can have a maximum of 300 game ticks (15 seconds) of  Air, while dolphins can reach up to 4800 game ticks (240 seconds), and axolotls have 6000 game ticks (300 seconds).
    ///
    /// `Air`
    pub air: i16,

    /// The custom name JSON text component of this entity. Appears in player death messages and villager trading interfaces, as well as above the entity when the player's cursor is over it. May be empty or not exist. Cannot be removed using the data remove command,[1] but setting it to an empty string has the same effect.
    ///
    /// `CustomName`
    pub custom_name: Option<Cow<'a, Mutf8Str>>,

    /// if true, and this entity has a custom name, the name always appears above the entity, regardless of where the cursor points. If the entity does not have a custom name, a default name is shown. May not exist.
    ///
    /// `CustomNameVisible`
    pub custom_name_visible: Option<bool>,

    /// Distance the entity has fallen. Larger values cause more damage when the entity lands.
    ///
    /// `FallDistance`
    pub fall_distance: f32,

    /// Number of game ticks until the fire is put out. Negative values reflect how long the entity can stand in fire before burning. Default -20 when not on fire.
    ///
    /// `Fire`
    pub fire: i16,

    /// if true, the entity has a glowing outline
    ///
    /// `Glowing`
    pub glowing: bool,

    /// if true, the entity visually appears on fire, even if it is not actually on fire.
    ///
    /// `HasVisualFire`
    pub has_visual_fire: bool,

    /// String representation of the entity's ID. Does not exist for the Player entity.
    pub id: Cow<'a, Mutf8Str>,

    ///  if true, the entity should not take damage. This applies to living and nonliving entities alike: mobs should not take damage from any source (including potion effects), and cannot be moved by fishing rods, attacks, explosions, or projectiles, and objects such as vehicles and item frames cannot be destroyed unless their supports are removed. Invulnerable player entities are also ignored by any hostile mobs. Note that these entities can be damaged by players in Creative mode.
    ///
    /// `Invulnerable`
    pub invulnerable: bool,

    /// List of 3 doubles describing the current dX, dY, and dZ velocity of the entity in meters per game tick. Only allows between 10.0 and -10.0 (inclusive), else resets to 0.
    ///
    /// `Motion`
    pub motion: [f64; 3],

    /// if true, the entity does not fall down naturally. Set to true by striders in lav
    ///
    /// `NoGravity`
    pub no_gravity: bool,

    /// if true, the entity is touching the ground.
    ///
    /// `OnGround`
    pub on_ground: bool,

    /// The data of the entities that are riding this entity.
    ///
    /// `Passengers`
    pub passengers: Vec<Entity<'a>>,

    /// The number of game ticks before which the entity may be teleported back through a nether portal. Initially starts at 300 game ticks (15 seconds) after teleportation and counts down to 0.
    ///
    /// `PortalCooldown`
    pub portal_cooldown: i32,

    /// List of 3 doubles describing the current X, Y, and Z position (coordinates) of the entity.
    ///
    /// `Pos`
    pub pos: [f64; 3],

    /// List of 2 floats representing the rotation of the entity's facing direction, in degrees. Facing direction can also be described as a looking direction, for most entity's that have heads.
    ///
    /// 0 - The yaw of the entity's oritentation. Yaw is the rotation around the Y axis (called yaw). Values vary from -180 degrees to +180 degrees, rather than from 0 to 360. As the entity turns to the right, this value goes up, and as the entity turns right, this value does down  
    ///
    /// 1 - The pitch of the entity's oritentation. Pitch is the offset from the horizon. Pitch = 0 means the direction is horizontal. A positive pitch (pitch > 0) means the entity is facing downward to some degree, or that the facing direction is facing below the horizon (toward the ground). A negative pitch (pitch > 0) means the entity is facing above the horizon (toward higher ground of the sky). Pitch is always between -90 and +90 degrees, where pitch = -90 means facing directly down, and pitch = +90 means facing directly up
    ///
    /// `Rotation`
    pub rotation: [f32; 2],

    /// if true, this entity is silenced. May not exis
    ///
    /// `Silent`
    pub silent: Option<bool>,

    /// List of scoreboard tags of this entity.
    ///
    /// `Tags`
    pub tags: Vec<Cow<'a, Mutf8Str>>,

    /// Optional. How many game ticks the entity has been freezing. Although this tag is defined for all entities, it is actually only used by mobs that are not in the freeze_immune_entity_types entity type tag. Increases while in powder snow, even partially, up to a maximum of 300 game ticks (15 seconds), and decreases at double speed while not in powder snow.
    ///
    /// `TicksFrozen`
    pub ticks_frozen: Option<i32>,

    /// This entity's Universally Unique IDentifier.
    /// `UUID`
    pub uuid: Uuid,
    //
    // TODO: Add entity specific data field like block entites, low priority as it allows very specific narrow block entity -> entity data handling
}

impl<'a> FromCompoundNbt for Entity<'a> {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let air = nbt
            .short("Air")
            .ok_or(SculkParseError::MissingField("Air".into()))?;
        let custom_name = get_optional_name(&nbt);
        let custom_name_visible = nbt.byte("CustomNameVisible").map(|b| b != 0);
        let fall_distance = nbt
            .float("FallDistance")
            .ok_or(SculkParseError::MissingField("FallDistance".into()))?;
        let fire = nbt
            .short("Fire")
            .ok_or(SculkParseError::MissingField("Fire".into()))?;
        let glowing = get_bool(&nbt, "Glowing");
        let has_visual_fire = get_bool(&nbt, "HasVisualFire");
        let id = get_owned_mutf8str(&nbt, "id")?;
        let invulnerable = get_bool(&nbt, "Invulnerable");

        let motion_list = nbt
            .list("Motion")
            .ok_or(SculkParseError::MissingField("Motion".into()))?;
        let mut motion: [f64; 3] = [0.0; 3];
        if let Some(doubles) = motion_list.doubles() {
            for (i, double) in doubles.iter().enumerate() {
                motion[i] = *double;
            }
        } else {
            return Err(SculkParseError::InvalidField("Motion".into()));
        }

        let no_gravity = get_bool(&nbt, "NoGravity");
        let on_ground = get_bool(&nbt, "OnGround");

        // Keeping this incase get_t_vec is inproperly made
        // let passengers: Vec<Entity<'a>> = if let Some(passengers) = nbt.list("Passengers") {
        //     passengers
        //         .compounds()
        //         .ok_or(SculkParseError::InvalidField("Passengers".into()))?
        //         .into_iter()
        //         .map(|nbt_passenger| Entity::from_compound_nbt(&nbt_passenger))
        //         .collect::<Result<Vec<Entity<'a>>, SculkParseError>>()?
        // } else {
        //     vec![]
        // };
        let passengers: Vec<Entity<'a>> =
            get_t_compound_vec(&nbt, "passengers", Entity::from_compound_nbt)?;

        let portal_cooldown = nbt
            .int("PortalCooldown")
            .ok_or(SculkParseError::MissingField("PortalCooldown".into()))?;

        let pos_list = nbt
            .list("Pos")
            .ok_or(SculkParseError::MissingField("Pos".into()))?;
        let mut pos: [f64; 3] = [0.0; 3];
        if let Some(doubles) = pos_list.doubles() {
            for (i, double) in doubles.iter().enumerate() {
                pos[i] = *double;
            }
        } else {
            return Err(SculkParseError::InvalidField("Pos".into()));
        }

        let rotation_list = nbt
            .list("Rotation")
            .ok_or(SculkParseError::MissingField("Rotation".into()))?;
        let mut rotation: [f32; 2] = [0.0; 2];
        if let Some(floats) = rotation_list.floats() {
            for (i, float) in floats.iter().enumerate() {
                rotation[i] = *float;
            }
        } else {
            return Err(SculkParseError::InvalidField("Rotation".into()));
        }

        let silent = nbt.byte("Silent").map(|b| b != 0);

        let tags_list = nbt
            .list("Tags")
            .ok_or(SculkParseError::MissingField("Tags".into()))?;
        let mut tags: Vec<Cow<'a, Mutf8Str>> = vec![];
        for tag in tags_list
            .strings()
            .ok_or(SculkParseError::InvalidField("Tags".into()))?
        {
            tags.push(Cow::Owned((*tag).to_owned()));
        }

        let ticks_frozen = nbt.int("TicksFrozen");
        let uuid = nbt
            .int_array("UUID")
            .map(Uuid::from)
            .ok_or(SculkParseError::MissingField("UUID".into()))?;

        Ok(Entity {
            air,
            custom_name,
            custom_name_visible,
            fall_distance,
            fire,
            glowing,
            has_visual_fire,
            id,
            invulnerable,
            motion,
            no_gravity,
            on_ground,
            passengers,
            portal_cooldown,
            pos,
            rotation,
            silent,
            tags,
            ticks_frozen,
            uuid,
        })
    }
}

// FUTURE ENTITY PLAN

// Entity Enum
// EntityBase - Common fields
// EntityData - Specific data

// Maybe have these inside every entity and just flatten everything?
// EntityMob - Mob entity data (have an option of None)
// EntityAngry - Angry entity data (have an option of None)
// EntityBreed - Breed entity data (have an option of None)
// EntityTame - Tame entity data (have an option of None)
// EntityRaid - Raid entity data (have an option of None)
// EntityZombie - Zombie entity data (have an option of None)
// EntityHorse - Horse entity data (have an option of None)
