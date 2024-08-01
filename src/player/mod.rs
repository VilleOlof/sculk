use crate::{
    components::food::EffectDetails,
    entity::Entity,
    error::SculkParseError,
    item::Item,
    traits::FromCompoundNbt,
    util::{get_bool, get_owned_optional_string, get_owned_string, get_t_compound_vec},
    uuid::Uuid,
};
use abilities::Abilities;
use game_type::GameType;
use recipe_book::RecipeBook;

pub mod abilities;
pub mod game_type;
pub mod recipe_book;

/// Represents a player's data.  
/// Often confined in a `[uuid].dat` file or in the `level.dat` file in singeplayer.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Player {
    /// Entity related data for the player.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub entity: PlayerEntity,
    // Entity, except for the tags: CustomName, CustomNameVisible, and Glowing.
    // Mobs, except for the tags: HandItems, ArmorItems, HandDropChances, ArmorDropChances, CanPickUpLoot, PersistenceRequired, Leash
    //
    /// The abilities this player has.
    pub abilities: Abilities,

    /// Version of the player NBT structure. Is increased with every new snapshot and release. See [Data version](https://minecraft.wiki/w/Data_version.  
    /// `DataVersion`
    pub data_version: i32,

    /// The ID of the dimension the player is in. Used to store the players last known location along with Pos.  
    /// `Dimension`
    pub dimension: String,

    ///  Each compound tag in this list is an item in the player's 27-slot ender chest inventory. When empty, list type may have unexpected value.  
    /// `EnderItems`
    pub ender_items: Vec<Item>,

    ///  May not exist. A compound of 3 doubles, describing the Overworld position from which the player entered the Nether. Used by the nether_travel advancement trigger. Set every time the player passes through a portal from the Overworld to the Nether. When entering a dimension other than the nether (not by respawning) this tag is removed. Entering the Nether without using a portal does not update this tag.  
    /// `enteredNetherPosition`
    pub entered_nether_position: Option<NetherPosition>,

    /// See [Hunger § Mechanics](https://minecraft.wiki/w/Hunger#Mechanics.  
    /// `foodExhaustionLevel`
    pub food_exhaustion_level: f32,

    /// The value of the hunger bar. Referred to as hunger. See [Hunger](https://minecraft.wiki/w/Hunger).  
    /// `foodLevel`
    pub food_level: i32,

    /// Referred to as saturation. See [Hunger § Mechanics](https://minecraft.wiki/w/Hunger#Mechanics).  
    /// `foodSaturationLevel`
    pub food_saturation_level: f32,

    /// See [Hunger](https://minecraft.wiki/w/Hunger).
    /// `foodTickTimer`
    pub food_tick_timer: i32,

    ///  Each compound tag in this list is an item in the player's inventory. (Note: when empty, list type may have [unexpected value](https://minecraft.wiki/w/NBT#As_used_in_Minecraft).)   
    /// See this image for slot numbers: [Inventory](https://minecraft.wiki/images/Items_slot_number.png?a8367).  
    /// `Inventory`
    pub inventory: Vec<Item>,

    /// May not exist. Location of the player's last death.
    /// `LastDeathLocation`
    pub last_death_location: Option<DeathLocation>,

    /// The current game mode of the player. 0 means Survival, 1 means Creative, 2 means Adventure, and 3 means Spectator.  
    /// `playerGameType`
    pub player_game_type: GameType,

    /// The previous game mode of the player.  
    /// `previousPlayerGameType`
    pub previous_player_game_type: Option<GameType>,

    /// Contains a JSON object detailing recipes the player has unlocked.  
    /// `recipeBook`
    pub recipe_book: RecipeBook,

    /// May not exist. The root entity that the player is riding.  
    /// `RootVehicle`
    pub root_vechile: Option<Vechile>,

    /// The score displayed upon death.
    /// `Score`
    pub score: i32,

    /// true if the player has entered the [exit portal](https://minecraft.wiki/w/Exit_portal) in the [End](https://minecraft.wiki/w/The_End) at least once.  
    /// `seenCredits`
    pub seen_credits: bool,

    /// The selected hotbar slot of the player.  
    /// `SelectedItemSlot`
    pub selected_item_slot: i32,

    /// The entity that is on the player's left shoulder. Always displays as a parrot.  
    /// `ShoulderEntityLeft`
    pub shoulder_entity_left: Option<Entity>,

    /// The entity that is on the player's right shoulder. Always displays as a parrot.  
    /// `ShoulderEntityRight`
    pub shoulder_entity_right: Option<Entity>,

    /// The number of game ticks the player had been in bed. 0 when the player is not sleeping. When in bed, increases up to 100 ticks, then stops. Skips the night after enough players in beds have reached 100 (see Bed § Passing the night). When getting out of bed, instantly changes to 100 ticks and then increases for another 9 ticks (up to 109 ticks) before returning to 0 ticks.
    /// `SleepTimer`
    pub sleep_timer: i16,

    /// May not exist. The dimension of the player's bed or respawn anchor. These tags are only removed if the player attempts to respawn with no valid bed or respawn anchor to spawn at at these coordinates. They are unaffected by breaking a bed or respawn anchor at these coordinates, and are unaffected by the player's death.
    /// `SpawnDimension`
    pub spawn_dimension: Option<String>,

    /// may not exist. true if the player should spawn at the below coordinates even if no bed can be found.
    /// `SpawnForced`
    pub spawn_forced: Option<bool>,

    /// May not exist. The X coordinate of the player's bed or respawn anchor. Removed when the player attempts to respawn with no valid bed or respawn anchor to spawn at at these coordinates. They are unaffected by breaking a bed or respawn anchor at these coordinates, and are unaffected by the player's death.  
    /// `SpawnX`
    pub spawn_x: Option<i32>,
    /// The Y coordinate of the spawn point.
    /// `SpawnY`
    pub spawn_y: Option<i32>,
    /// The Z coordinate of the spawn point.
    /// `SpawnZ`
    pub spawn_z: Option<i32>,

    /// Contains data about the warden spawning process for this player.
    pub warden_spawn_tracker: WardenTracker,

    /// The level shown on the experience bar.  
    /// `XpLevel`
    pub xp_level: i32,

    /// The progress across the experience bar to the next level, stored as a percentage.  
    /// `XpP`
    pub xp_p: f32,

    /// The seed used for the next enchantment in enchantment tables.  
    /// `XpSeed`
    pub xp_seed: i32,

    /// The total amount of experience the player has collected over time; used for the score upon death.  
    /// `XpTotal`
    pub xp_total: i32,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PlayerEntity {
    /// How much air the entity has, in game ticks. Decreases when unable to breathe (except suffocating in a block). Increases when it can breathe.  Air being <= -20 game ticks (while still unable to breathe) on a given game tick causes the entity to immediately lose 1 health to drowning damage. This resets  Air to 0 game ticks. Most mobs can have a maximum of 300 game ticks (15 seconds) of  Air, while dolphins can reach up to 4800 game ticks (240 seconds), and axolotls have 6000 game ticks (300 seconds).  
    /// `Air`
    pub air: i16,

    // Distance the entity has fallen. Larger values cause more damage when the entity lands.
    // `FallDistance`
    pub fall_distance: f32,

    // Number of game ticks until the fire is put out. Negative values reflect how long the entity can stand in fire before burning. Default -20 when not on fire.
    // `Fire`
    pub fire: i16,

    /// if true, the entity visually appears on fire, even if it is not actually on fire.
    ///
    /// `HasVisualFire`
    pub has_visual_fire: bool,

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
    pub passengers: Vec<Entity>,

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
    pub tags: Vec<String>,

    /// Optional. How many game ticks the entity has been freezing. Although this tag is defined for all entities, it is actually only used by mobs that are not in the freeze_immune_entity_types entity type tag. Increases while in powder snow, even partially, up to a maximum of 300 game ticks (15 seconds), and decreases at double speed while not in powder snow.
    ///
    /// `TicksFrozen`
    pub ticks_frozen: Option<i32>,

    /// This entity's Universally Unique IDentifier.
    /// `UUID`
    pub uuid: Uuid,

    // Mob fields
    /// number of extra health added by Absorption effect.  
    /// `AbsorptionAmount`
    pub absorption_amount: Option<f32>,

    /// The list of potion effects on this mob. May not exist.  
    #[cfg_attr(feature = "serde", serde(default))]
    pub active_effects: Vec<EffectDetails>,
    //
    // /// A list of Attributes for this mob. These are used for many purposes in internal calculations, and can be considered a mob's "statistics"
    // /// TODO: This is a bit more complex than other field so uh fix this at some point
    // pub attributes: Vec<Attribute>,
    //
    /// Number of ticks the mob has been dead for. Controls death animations. 0 when alive.  
    /// `DeathTime`
    pub death_time: i16,

    /// Setting to 1 for non-player entities causes the entity to glide as long as they are wearing elytra in the chest slot. Can be used to detect when the player is gliding without using scoreboard statistics.
    /// `FallFlying`
    pub fall_flying: bool,

    /// number of health the entity has.  
    /// `Health`
    pub health: f32,

    /// The last time the mob was damaged, measured in the number of ticks since the mob's creation. Updates to a new value whenever the mob is damaged, then updates again 101 ticks later for reasons unknown. Can be changed with commands, but the specified value does not affect natural updates in any way, and is overwritten if the mob receives damage.  
    /// `HurtByTimestamp`
    pub hurt_by_timestamp: i32,

    /// the mob renders the main hand as being left.
    /// `LeftHanded`
    pub left_handed: bool,
}

/// Contains data about the warden spawning process for this player.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WardenTracker {
    /// A warning level between 0, and 3 (inclusive). The warden spawns at level 3.
    pub warning_level: i32,

    /// The number of game ticks before the warning_level can be increased again. Decreases by 1 every tick. It is set to 200 game ticks (10 seconds) every time the warning level is increased.
    pub cooldown_ticks: i32,

    /// The number of game ticks since the player was warned for warden spawning. Increases by 1 every tick. After 12000 game ticks (10 minutes) it resets to level 3, and the warning_level decreases by 1 level.
    pub ticks_since_last_warning: i32,
}

/// The root entity that the player is riding.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Vechile {
    /// The UUID of the entity the player is riding, stored as four ints.  
    /// `Attach`
    pub attach: Uuid,

    /// The NBT data of the root vehicle.  
    /// `Entity`
    pub entity: Entity,
}

/// The location of the player's last death.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeathLocation {
    /// Dimension of last death.
    pub dimension: String,

    /// Coordinates of last death.
    pub pos: Vec<i32>,
}

/// The position the player entered the Nether from.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NetherPosition {
    /// The X coordinate in the Overworld.
    x: f64,
    /// The Y coordinate in the Overworld.
    y: f64,
    /// The Z coordinate in the Overworld.
    z: f64,
}

impl FromCompoundNbt for PlayerEntity {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let air = nbt
            .short("Air")
            .ok_or(SculkParseError::MissingField("Air".into()))?;
        let fall_distance = nbt
            .float("FallDistance")
            .ok_or(SculkParseError::MissingField("FallDistance".into()))?;
        let fire = nbt
            .short("Fire")
            .ok_or(SculkParseError::MissingField("Fire".into()))?;
        let has_visual_fire = get_bool(&nbt, "HasVisualFire");
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

        let passengers: Vec<Entity> =
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

        let tags = if let Some(tags_list) = nbt.list("Tags") {
            let mut tags: Vec<String> = vec![];
            for tag in tags_list
                .strings()
                .ok_or(SculkParseError::InvalidField("Tags".into()))?
            {
                tags.push((*tag).to_string());
            }

            tags
        } else {
            vec![]
        };

        let ticks_frozen = nbt.int("TicksFrozen");
        let uuid = nbt
            .int_array("UUID")
            .map(Uuid::from)
            .ok_or(SculkParseError::MissingField("UUID".into()))?;

        let absorption_amount = nbt.float("AbsorptionAmount");

        let active_effects =
            get_t_compound_vec(&nbt, "ActiveEffects", EffectDetails::from_compound_nbt)?;

        let death_time = nbt.short("DeathTime").unwrap_or(0);

        let fall_flying = get_bool(&nbt, "FallFlying");
        let health = nbt
            .float("Health")
            .ok_or(SculkParseError::MissingField("Health".into()))?;

        let hurt_by_timestamp = nbt
            .int("HurtByTimestamp")
            .ok_or(SculkParseError::MissingField("HurtByTimestamp".into()))?;

        let left_handed = get_bool(&nbt, "LeftHanded");

        Ok(PlayerEntity {
            air,
            fall_distance,
            fire,
            has_visual_fire,
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
            absorption_amount,
            active_effects,
            death_time,
            fall_flying,
            health,
            hurt_by_timestamp,
            left_handed,
        })
    }
}

impl FromCompoundNbt for Player {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let player_entity = PlayerEntity::from_compound_nbt(nbt)?;

        let abilities = nbt
            .compound("abilities")
            .map(|nbt| Abilities::from_compound_nbt(&nbt))
            .ok_or(SculkParseError::MissingField("abilities".into()))??;

        let data_version = nbt
            .int("DataVersion")
            .ok_or(SculkParseError::MissingField("DataVersion".into()))?;

        let dimension = get_owned_string(&nbt, "Dimension")?;
        let ender_items = get_t_compound_vec(&nbt, "EnderItems", Item::from_compound_nbt)?;

        let entered_nether_position = if let Some(nbt) = nbt.compound("enteredNetherPosition") {
            Some(NetherPosition::from_compound_nbt(&nbt)?)
        } else {
            None
        };

        let food_exhaustion_level = nbt
            .float("foodExhaustionLevel")
            .ok_or(SculkParseError::MissingField("foodExhaustionLevel".into()))?;

        let food_level = nbt
            .int("foodLevel")
            .ok_or(SculkParseError::MissingField("foodLevel".into()))?;
        let food_saturation_level = nbt
            .float("foodSaturationLevel")
            .ok_or(SculkParseError::MissingField("foodSaturationLevel".into()))?;
        let food_tick_timer = nbt
            .int("foodTickTimer")
            .ok_or(SculkParseError::MissingField("foodTickTimer".into()))?;

        let inventory = get_t_compound_vec(&nbt, "Inventory", Item::from_compound_nbt)?;

        let last_death_location = if let Some(nbt) = nbt.compound("LastDeathLocation") {
            Some(DeathLocation::from_compound_nbt(&nbt)?)
        } else {
            None
        };

        let player_game_type = nbt
            .int("playerGameType")
            .map(|i| GameType::from_i32(i))
            .ok_or(SculkParseError::MissingField("playerGameType".into()))?;

        let previous_player_game_type = if let Some(nbt) = nbt.int("previousPlayerGameType") {
            Some(GameType::from_i32(nbt))
        } else {
            None
        };

        let recipe_book = nbt
            .compound("recipeBook")
            .map(|nbt| RecipeBook::from_compound_nbt(&nbt))
            .ok_or(SculkParseError::MissingField("recipeBook".into()))??;

        let root_vechile = if let Some(nbt) = nbt.compound("RootVehicle") {
            Some(Vechile::from_compound_nbt(&nbt)?)
        } else {
            None
        };

        let score = nbt.int("Score").unwrap_or(0);

        let seen_credits = get_bool(&nbt, "seenCredits");
        let selected_item_slot = nbt
            .int("SelectedItemSlot")
            .ok_or(SculkParseError::MissingField("SelectedItemSlot".into()))?;

        let shoulder_entity_left = if let Some(nbt) = nbt.compound("ShoulderEntityLeft") {
            Some(Entity::from_compound_nbt(&nbt)?)
        } else {
            None
        };
        let shoulder_entity_right = if let Some(nbt) = nbt.compound("ShoulderEntityRight") {
            Some(Entity::from_compound_nbt(&nbt)?)
        } else {
            None
        };

        let sleep_timer = nbt
            .short("SleepTimer")
            .ok_or(SculkParseError::MissingField("SleepTimer".into()))?;

        let spawn_dimension = get_owned_optional_string(&nbt, "SpawnDimension");

        let spawn_forced = nbt.byte("SpawnForced").map(|b| b != 0);

        let spawn_x = nbt.int("SpawnX");
        let spawn_y = nbt.int("SpawnY");
        let spawn_z = nbt.int("SpawnZ");

        let warden_spawn_tracker = nbt
            .compound("warden_spawn_tracker")
            .map(|nbt| WardenTracker::from_compound_nbt(&nbt))
            .ok_or(SculkParseError::MissingField("warden_spawn_tracker".into()))??;

        let xp_level = nbt.int("XpLevel").unwrap_or(0);
        let xp_p = nbt.float("XpP").unwrap_or(0.0);
        let xp_seed = nbt.int("XpSeed").unwrap_or(0);
        let xp_total = nbt.int("XpTotal").unwrap_or(0);

        Ok(Player {
            entity: player_entity,
            abilities,
            data_version,
            dimension,
            ender_items,
            entered_nether_position,
            food_exhaustion_level,
            food_level,
            food_saturation_level,
            food_tick_timer,
            inventory,
            last_death_location,
            player_game_type,
            previous_player_game_type,
            recipe_book,
            root_vechile,
            score,
            seen_credits,
            selected_item_slot,
            shoulder_entity_left,
            shoulder_entity_right,
            sleep_timer,
            spawn_dimension,
            spawn_forced,
            spawn_x,
            spawn_y,
            spawn_z,
            warden_spawn_tracker,
            xp_level,
            xp_p,
            xp_seed,
            xp_total,
        })
    }
}

impl Player {
    /// Returns the data version of the player.
    pub fn get_data_version(nbt: &simdnbt::borrow::NbtCompound) -> Result<i32, SculkParseError> {
        nbt.int("DataVersion")
            .ok_or(SculkParseError::MissingField("DataVersion".into()))
    }
}

impl FromCompoundNbt for WardenTracker {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let warning_level = nbt
            .int("warning_level")
            .ok_or(SculkParseError::MissingField("warning_level".into()))?;

        let cooldown_ticks = nbt
            .int("cooldown_ticks")
            .ok_or(SculkParseError::MissingField("cooldown_ticks".into()))?;

        let ticks_since_last_warning =
            nbt.int("ticks_since_last_warning")
                .ok_or(SculkParseError::MissingField(
                    "ticks_since_last_warning".into(),
                ))?;

        Ok(WardenTracker {
            warning_level,
            cooldown_ticks,
            ticks_since_last_warning,
        })
    }
}

impl FromCompoundNbt for Vechile {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let attach = nbt
            .int_array("attach")
            .map(Uuid::from)
            .ok_or(SculkParseError::MissingField("attach".into()))?;

        let entity = nbt
            .compound("Entity")
            .map(|nbt| Entity::from_compound_nbt(&nbt))
            .ok_or(SculkParseError::MissingField("Entity".into()))??;

        Ok(Vechile { attach, entity })
    }
}

impl FromCompoundNbt for DeathLocation {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let dimension = get_owned_string(&nbt, "dimension")?;
        let pos = nbt
            .int_array("pos")
            .ok_or(SculkParseError::MissingField("pos".into()))?;

        Ok(DeathLocation { dimension, pos })
    }
}

impl FromCompoundNbt for NetherPosition {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let x = nbt
            .double("x")
            .ok_or(SculkParseError::MissingField("x".into()))?;
        let y = nbt
            .double("y")
            .ok_or(SculkParseError::MissingField("y".into()))?;
        let z = nbt
            .double("z")
            .ok_or(SculkParseError::MissingField("z".into()))?;

        Ok(NetherPosition { x, y, z })
    }
}

#[cfg(test)]
#[test]
fn complex_player_dat() {
    use flate2::read::GzDecoder;
    use std::io::{Cursor, Read};

    let mut file = std::fs::File::open("test_data/player_data.dat").unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    let mut src = &contents[..];

    let mut src_decoder = GzDecoder::new(&mut src);
    let mut input = Vec::new();
    if src_decoder.read_to_end(&mut input).is_err() {
        input = contents;
    }
    let mut input_stream = Cursor::new(&input[..]);

    let nbt = simdnbt::borrow::read(&mut input_stream).unwrap().unwrap();
    let nbt = nbt.as_compound();

    let _ = Player::from_compound_nbt(&nbt).unwrap();
}
