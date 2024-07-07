use std::borrow::Cow;

use serde::{Deserialize, Serialize};

fn default_fire() -> i16 {
    -20
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Entity<'a> {
    ///  How much air the entity has, in game ticks. Decreases when unable to breathe (except suffocating in a block). Increases when it can breathe.  Air being <= -20 game ticks (while still unable to breathe) on a given game tick causes the entity to immediately lose 1 health to drowning damage. This resets  Air to 0 game ticks. Most mobs can have a maximum of 300 game ticks (15 seconds) of  Air, while dolphins can reach up to 4800 game ticks (240 seconds), and axolotls have 6000 game ticks (300 seconds).
    #[serde(rename = "Air")]
    pub air: i16,

    /// The custom name JSON text component of this entity. Appears in player death messages and villager trading interfaces, as well as above the entity when the player's cursor is over it. May be empty or not exist. Cannot be removed using the data remove command,[1] but setting it to an empty string has the same effect.
    #[serde(borrow)]
    #[serde(rename = "CustomName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_name: Option<Cow<'a, str>>,

    /// if true, and this entity has a custom name, the name always appears above the entity, regardless of where the cursor points. If the entity does not have a custom name, a default name is shown. May not exist.
    #[serde(rename = "CustomNameVisible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_name_visible: Option<bool>,

    /// Distance the entity has fallen. Larger values cause more damage when the entity lands.
    #[serde(rename = "FallDistance")]
    pub fall_distance: f32,

    /// Number of game ticks until the fire is put out. Negative values reflect how long the entity can stand in fire before burning. Default -20 when not on fire.
    #[serde(default = "default_fire")]
    #[serde(rename = "Fire")]
    pub fire: i16,

    /// if true, the entity has a glowing outline
    #[serde(rename = "Glowing")]
    pub glowing: bool,

    /// if true, the entity visually appears on fire, even if it is not actually on fire.
    #[serde(rename = "HasVisualFire")]
    pub has_visual_fire: bool,

    /// String representation of the entity's ID. Does not exist for the Player entity.
    #[serde(borrow)]
    pub id: Cow<'a, str>,

    ///  if true, the entity should not take damage. This applies to living and nonliving entities alike: mobs should not take damage from any source (including potion effects), and cannot be moved by fishing rods, attacks, explosions, or projectiles, and objects such as vehicles and item frames cannot be destroyed unless their supports are removed. Invulnerable player entities are also ignored by any hostile mobs. Note that these entities can be damaged by players in Creative mode.
    #[serde(rename = "Invulnerable")]
    pub invulnerable: bool,

    /// List of 3 doubles describing the current dX, dY, and dZ velocity of the entity in meters per game tick. Only allows between 10.0 and -10.0 (inclusive), else resets to 0.
    #[serde(rename = "Motion")]
    pub motion: [f64; 3],

    /// if true, the entity does not fall down naturally. Set to true by striders in lav
    #[serde(rename = "NoGravity")]
    pub no_gravity: bool,

    /// if true, the entity is touching the ground.
    #[serde(rename = "OnGround")]
    pub on_ground: bool,

    /// The data of the entities that are riding this entity.
    #[serde(borrow)]
    #[serde(default)]
    #[serde(rename = "Passengers")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub passengers: Vec<Entity<'a>>,

    /// The number of game ticks before which the entity may be teleported back through a nether portal. Initially starts at 300 game ticks (15 seconds) after teleportation and counts down to 0.
    #[serde(rename = "PortalCooldown")]
    pub portal_cooldown: i32,

    /// List of 3 doubles describing the current X, Y, and Z position (coordinates) of the entity.
    #[serde(rename = "Pos")]
    pub pos: [f64; 3],

    /// List of 2 floats representing the rotation of the entity's facing direction, in degrees. Facing direction can also be described as a looking direction, for most entity's that have heads.
    ///
    /// 0 - The yaw of the entity's oritentation. Yaw is the rotation around the Y axis (called yaw). Values vary from -180 degrees to +180 degrees, rather than from 0 to 360. As the entity turns to the right, this value goes up, and as the entity turns right, this value does down  
    ///
    /// 1 - The pitch of the entity's oritentation. Pitch is the offset from the horizon. Pitch = 0 means the direction is horizontal. A positive pitch (pitch > 0) means the entity is facing downward to some degree, or that the facing direction is facing below the horizon (toward the ground). A negative pitch (pitch > 0) means the entity is facing above the horizon (toward higher ground of the sky). Pitch is always between -90 and +90 degrees, where pitch = -90 means facing directly down, and pitch = +90 means facing directly up
    #[serde(rename = "Rotation")]
    pub rotation: [f32; 2],

    /// if true, this entity is silenced. May not exis
    #[serde(rename = "Silent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent: Option<bool>,

    /// List of scoreboard tags of this entity.
    #[serde(borrow)]
    #[serde(default)]
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<Cow<'a, str>>,

    /// Optional. How many game ticks the entity has been freezing. Although this tag is defined for all entities, it is actually only used by mobs that are not in the freeze_immune_entity_types entity type tag. Increases while in powder snow, even partially, up to a maximum of 300 game ticks (15 seconds), and decreases at double speed while not in powder snow.
    #[serde(rename = "TicksFrozen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticks_frozen: Option<i32>,

    /// This entity's Universally Unique IDentifier.
    #[serde(rename = "UUID")]
    pub uuid: i128,
    // TODO: Add entity specific data field like block entites, low priority as it allows very specific narrow block entity -> entity data handling
}
