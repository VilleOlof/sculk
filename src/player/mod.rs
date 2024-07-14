use crate::{
    entity::Entity,
    error::SculkParseError,
    item::Item,
    traits::FromCompoundNbt,
    util::{get_bool, get_owned_mutf8str, get_owned_optional_mutf8str, get_t_compound_vec},
    uuid::Uuid,
};
use abilities::Abilities;
use game_type::GameType;
use recipe_book::RecipeBook;
use simdnbt::Mutf8Str;
use std::borrow::Cow;

pub mod abilities;
pub mod game_type;
pub mod recipe_book;

#[derive(Debug, Clone, PartialEq)]
pub struct Player<'a> {
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
    pub dimension: Cow<'a, Mutf8Str>,

    ///  Each compound tag in this list is an item in the player's 27-slot ender chest inventory. When empty, list type may have unexpected value.  
    /// `EnderItems`
    pub ender_items: Vec<Item<'a>>,

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
    pub inventory: Vec<Item<'a>>,

    /// May not exist. Location of the player's last death.
    /// `LastDeathLocation`
    pub last_death_location: Option<DeathLocation<'a>>,

    /// The current game mode of the player. 0 means Survival, 1 means Creative, 2 means Adventure, and 3 means Spectator.  
    /// `playerGameType`
    pub player_game_type: GameType,

    /// The previous game mode of the player.  
    /// `previousPlayerGameType`
    pub previous_player_game_type: Option<GameType>,

    /// Contains a JSON object detailing recipes the player has unlocked.  
    /// `recipeBook`
    pub recipe_book: RecipeBook<'a>,

    /// May not exist. The root entity that the player is riding.  
    /// `RootVehicle`
    pub root_vechile: Option<Vechile<'a>>,

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
    pub shoulder_entity_left: Option<Entity<'a>>,

    /// The entity that is on the player's right shoulder. Always displays as a parrot.  
    /// `ShoulderEntityRight`
    pub shoulder_entity_right: Option<Entity<'a>>,

    /// The number of game ticks the player had been in bed. 0 when the player is not sleeping. When in bed, increases up to 100 ticks, then stops. Skips the night after enough players in beds have reached 100 (see Bed § Passing the night). When getting out of bed, instantly changes to 100 ticks and then increases for another 9 ticks (up to 109 ticks) before returning to 0 ticks.
    /// `SleepTimer`
    pub sleep_timer: i16,

    /// May not exist. The dimension of the player's bed or respawn anchor. These tags are only removed if the player attempts to respawn with no valid bed or respawn anchor to spawn at at these coordinates. They are unaffected by breaking a bed or respawn anchor at these coordinates, and are unaffected by the player's death.
    /// `SpawnDimension`
    pub spawn_dimension: Option<Cow<'a, Mutf8Str>>,

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
pub struct WardenTracker {
    /// A warning level between 0, and 3 (inclusive). The warden spawns at level 3.
    pub warning_level: i32,

    /// The number of game ticks before the warning_level can be increased again. Decreases by 1 every tick. It is set to 200 game ticks (10 seconds) every time the warning level is increased.
    pub cooldown_ticks: i32,

    /// The number of game ticks since the player was warned for warden spawning. Increases by 1 every tick. After 12000 game ticks (10 minutes) it resets to level 3, and the warning_level decreases by 1 level.
    pub ticks_since_last_warning: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Vechile<'a> {
    /// The UUID of the entity the player is riding, stored as four ints.  
    /// `Attach`
    pub attach: Uuid,

    /// The NBT data of the root vehicle.  
    /// `Entity`
    pub entity: Entity<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeathLocation<'a> {
    /// Dimension of last death.
    pub dimension: Cow<'a, Mutf8Str>,

    /// Coordinates of last death.
    pub pos: Vec<i32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NetherPosition {
    /// The X coordinate in the Overworld.
    x: f64,
    /// The Y coordinate in the Overworld.
    y: f64,
    /// The Z coordinate in the Overworld.
    z: f64,
}

impl<'a> FromCompoundNbt for Player<'a> {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let abilities = nbt
            .compound("abilities")
            .map(|nbt| Abilities::from_compound_nbt(&nbt))
            .ok_or(SculkParseError::MissingField("abilities".into()))??;

        let data_version = nbt
            .int("DataVersion")
            .ok_or(SculkParseError::MissingField("DataVersion".into()))?;

        let dimension = get_owned_mutf8str(&nbt, "Dimension")?;
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

        let spawn_dimension = get_owned_optional_mutf8str(&nbt, "SpawnDimension");

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

impl<'a> FromCompoundNbt for Vechile<'a> {
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

impl<'a> FromCompoundNbt for DeathLocation<'a> {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let dimension = get_owned_mutf8str(&nbt, "dimension")?;
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

    let mut file = std::fs::File::open("test_data/player.dat").unwrap();
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
