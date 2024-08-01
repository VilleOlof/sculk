use boss_event::CustomBossEvent;
use datapacks::Datapacks;
use dimension_data::DragonFight;
use world_gen_settings::WorldGenSettings;

use crate::{
    error::SculkParseError,
    kv::KVPair,
    player::{game_type::GameType, Player},
    traits::FromCompoundNbt,
    util::{get_bool, get_owned_string},
    uuid::Uuid,
};

pub mod boss_event;
pub mod datapacks;
pub mod dimension_data;
pub mod world_gen_settings;

/// The level.dat file contains global information about the world such as the time of day, the singleplayer player, he level generator used, and the seed.
///
/// The actual file begins with a root tag and then a `Data` tag, this data tag is flattened to this struct.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Level {
    /// true if cheats are enabled.  
    /// `allowCommands`
    pub allow_commands: bool,

    /// Center of the world border on the X coordinate. Defaults to 0  
    /// `BorderCenterX`
    pub border_center_x: f64,

    /// Center of the world border on the Z coordinate. Defaults to 0  
    /// `BorderCenterZ`
    pub border_center_z: f64,

    ///  Defaults to 0.2.  
    /// `BorderDamagePerBlock`
    pub border_damage_per_block: f64,

    /// Width and length of the border of the border. Defaults to 60000000.  
    /// `BorderSize`
    pub border_size: f64,

    /// Defaults to 5.  
    /// `BorderSafeZone`
    pub border_safe_zone: f64,

    /// Defaults to 60000000.
    /// `BorderSizeLerpTarget`
    pub border_size_lerp_target: f64,

    /// Defaults to 0.  
    /// `BorderSizeLerpTime`
    pub border_size_lerp_time: i64,

    /// Defaults to 5.  
    /// `BorderWarningBlocks`
    pub border_warning_blocks: f64,

    /// Defaults to 15.  
    /// `BorderWarningTime`
    pub border_warning_time: f64,

    /// The number of ticks until "clear weather" has ended.  
    /// `clearWeatherTime`
    pub clear_weather_time: i32,

    /// A collection of bossbars.  
    /// `CustomBossEvents`
    pub custom_boss_events: KVPair<CustomBossEvent>,

    /// Options for datapacks.  
    /// `DataPacks`
    pub datapacks: Datapacks,

    /// An integer displaying the [data version](https://minecraft.wiki/w/Data_version).  
    /// `DataVersion`
    pub data_version: i32,

    /// The time of day. 0 is sunrise, 6000 is mid day, 12000 is sunset, 18000 is mid night, 24000 is the next day's 0. This value keeps counting past 24000 and does not reset to 0.  
    /// `DayTime`
    pub day_time: i64,

    /// The current difficulty setting.  
    /// `Difficulty`
    pub difficulty: Difficulty,

    /// True if the difficulty has been locked. Defaults to true.  
    /// `DifficultyLocked`
    pub difficulty_locked: bool,

    /// This tag contains level data specific to certain dimensions.  
    /// This one is weird, wiki says it should be nested 2 times but its a direct tag.  
    /// `DimensionData`
    pub dimension_data: DragonFight,

    /// List of experimental features enabled for this world. Empty if there are no expirimental features enabled.  
    /// `enabled_features`
    pub enabled_features: Vec<String>,

    /// The gamerules used in the world.  
    ///
    /// The value for the given rule. This is always an NBT string, which is either true or false for the majority of rules (with it being a number for some other rules, and any arbitrary string for a user-defined rule)  
    ///
    /// `GameRules`
    pub game_rules: KVPair<String>,

    /// The default game mode for the singleplayer player when they initially spawn.  
    /// `GameType`
    pub game_type: GameType,

    ///  the generation settings for each dimension.  
    /// `WorldGenSettings`
    pub world_gen_settings: WorldGenSettings,

    /// If true, the player respawns in Spectator on death in singleplayer. Affects all three game modes.  
    /// `hardcore`
    pub hardcore: bool,

    /// Normally true after a world has been initialized properly after creation. If the initial simulation was canceled somehow, this can be false and the world is re-initialized on next load.
    /// `initialized`
    pub initialized: bool,

    /// The Unix time in milliseconds when the level was last loaded.  
    /// `LastPlayed`
    pub last_played: i64,

    /// The name of the level.  
    /// `LevelName`
    pub level_name: String,

    ///  true if the map generator should place structures such as villages, strongholds, and mineshafts. Defaults to 1. Always 1 if the world type is Customized.  
    /// `MapFeatures`
    pub map_features: bool,

    /// The state of the Singleplayer player. This overrides the <player>.dat file with the same name as the Singleplayer player. This is saved by Servers only if it already exists, otherwise it is not saved for server worlds. See [Player.dat Format](https://minecraft.wiki/w/Player.dat_format#NBT_Structure).  
    /// `Player`
    pub player: Option<Player>,

    /// true if the level is currently experiencing rain, snow, and cloud cover.  
    /// `raining`
    pub raining: bool,

    /// The number of ticks before "raining" is toggled and this value gets set to another random value.  
    /// `rainTime`
    pub rain_time: i32,

    /// The [random level seed](https://minecraft.wiki/w/Seed_(world_generation)) used to generate consistent terrain.  
    /// `RandomSeed`
    pub random_seed: Option<i64>,

    /// The estimated size in bytes of the level. Currently not modified or used by Minecraft, but was previously.  
    /// `SizeOnDisk`
    pub size_on_disk: Option<i64>,

    /// The X coordinate of the world spawn.  
    /// `SpawnX`
    pub spawn_x: i32,

    /// The Y coordinate of the world spawn.  
    /// `SpawnY`
    pub spawn_y: i32,

    /// The Z coordinate of the world spawn.  
    /// `SpawnZ`
    pub spawn_z: i32,

    ///  If "raining" is true : true if the rain/snow/cloud cover is a lightning storm and dark enough for mobs to spawn under the sky. If "raining" is false, this has no effect.  
    /// `thundering`
    pub thundering: bool,

    /// The number of ticks before "thundering" is toggled and this value gets set to another random value.  
    /// `thunderTime`
    pub thunder_time: i32,

    /// The number of ticks since the start of the level.  
    /// `Time`
    pub time: i64,

    /// The NBT version of the level, with 1.14.4 being 19133.  
    /// `version`
    pub version: i32,

    /// Information about the Minecraft version the world was saved in.
    /// `Version`
    pub version_data: VersionData,

    /// The UUID of the current wandering trader in the world saved as four ints.  
    /// `WanderingTraderId`
    pub wandering_trader_id: Option<Uuid>,

    /// The current chance of the wandering trader spawning next attempt; this value is the percentage and is divided by 10 when loaded by the game, for example a value of 50 means 5.0% chance.  
    /// `WanderingTraderSpawnChance`
    pub wandering_trader_spawn_chance: i32,

    /// The amount of ticks until another wandering trader is attempted to spawn  
    /// `WanderingTraderSpawnDelay`
    pub wandering_trader_spawn_delay: i32,

    ///  true if the world was opened in a modified version.  
    /// `WasModded`
    pub was_modded: bool,
}

/// More detailed information about the Minecraft version the world was saved in.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VersionData {
    /// An integer displaying the data version.  
    /// `Id`
    pub id: i32,

    /// The version name as a string, e.g. "15w32b".  
    /// `Name`
    pub name: String,

    /// Developing series. In 1.18 experimental snapshots, it was set to "ccpreview". In others, set to "main".  
    /// `Series`
    pub series: String,

    /// Whether the version is a snapshot or not.  
    /// `Snapshot`
    pub snapshot: bool,
}

impl FromCompoundNbt for VersionData {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let id = nbt
            .int("Id")
            .ok_or(SculkParseError::MissingField("Id".into()))?;
        let name = get_owned_string(&nbt, "Name")?;
        let series = get_owned_string(&nbt, "Series")?;
        let snapshot = get_bool(&nbt, "Snapshot");

        Ok(VersionData {
            id,
            name,
            series,
            snapshot,
        })
    }
}

/// The difficulty of the world.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Difficulty {
    /// Peaceful difficulty.
    Peaceful,
    /// Easy difficulty.
    Easy,
    /// Normal difficulty.
    Normal,
    /// Hard difficulty.
    Hard,
    /// An unknown difficulty. (how tf)
    Unknown(i8),
}

impl Difficulty {
    /// Converts an i8 to a Difficulty.
    pub fn from_i8(b: i8) -> Difficulty {
        match b {
            0 => Difficulty::Peaceful,
            1 => Difficulty::Easy,
            2 => Difficulty::Normal,
            3 => Difficulty::Hard,
            _ => Difficulty::Unknown(b),
        }
    }
}

impl FromCompoundNbt for Level {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        // The level.dat file is a compound tag with a single compound tag called "Data"
        let nbt = nbt
            .compound("Data")
            .ok_or(SculkParseError::MissingField("Data".into()))?;

        let allow_commands = get_bool(&nbt, "allowCommands");

        let border_center_x = nbt.double("BorderCenterX").unwrap_or(0.0);
        let border_center_z = nbt.double("BorderCenterZ").unwrap_or(0.0);
        let border_damage_per_block = nbt.double("BorderDamagePerBlock").unwrap_or(0.2);
        let border_size = nbt.double("BorderSize").unwrap_or(60000000.0);
        let border_safe_zone = nbt.double("BorderSafeZone").unwrap_or(5.0);
        let border_size_lerp_target = nbt.double("BorderSizeLerpTarget").unwrap_or(60000000.0);
        let border_size_lerp_time = nbt.long("BorderSizeLerpTime").unwrap_or(0);
        let border_warning_blocks = nbt.double("BorderWarningBlocks").unwrap_or(5.0);
        let border_warning_time = nbt.double("BorderWarningTime").unwrap_or(15.0);

        let clear_weather_time = nbt
            .int("clearWeatherTime")
            .ok_or(SculkParseError::InvalidField("clearWeatherTime".into()))?;

        let custom_boss_events: KVPair<CustomBossEvent> = nbt
            .compound("CustomBossEvents")
            .map(|nbt| KVPair::from_compound_nbt(&nbt))
            .ok_or(SculkParseError::MissingField("CustomBossEvents".into()))??;

        let datapacks = nbt
            .compound("DataPacks")
            .map(|nbt| Datapacks::from_compound_nbt(&nbt))
            .ok_or(SculkParseError::MissingField("DataPacks".into()))??;

        let data_version = nbt
            .int("DataVersion")
            .ok_or(SculkParseError::MissingField("DataVersion".into()))?;

        let day_time = nbt
            .long("DayTime")
            .ok_or(SculkParseError::MissingField("DayTime".into()))?;

        let difficulty = nbt
            .byte("Difficulty")
            .map(|b| Difficulty::from_i8(b))
            .unwrap_or(Difficulty::Normal);
        let difficulty_locked = nbt.byte("DifficultyLocked").map(|b| b != 0).unwrap_or(true);

        let dimension_data = nbt
            .compound("DragonFight")
            .map(|nbt| DragonFight::from_compound_nbt(&nbt))
            .ok_or(SculkParseError::MissingField("DragonFight".into()))??;

        let enabled_features = if let Some(nbt) = nbt.list("enabled_features") {
            let list = nbt
                .strings()
                .ok_or(SculkParseError::InvalidField("enabled_features".into()))?;

            let mut vec: Vec<String> = vec![];

            for item in list.iter() {
                vec.push((*item).to_string());
            }

            vec
        } else {
            vec![]
        };

        let game_rules: KVPair<String> = nbt
            .compound("GameRules")
            .map(|nbt| KVPair::from_compound_nbt(&nbt))
            .ok_or(SculkParseError::MissingField("GameRules".into()))??;

        let world_gen_settings = nbt
            .compound("WorldGenSettings")
            .map(|nbt| WorldGenSettings::from_compound_nbt(&nbt))
            .ok_or(SculkParseError::MissingField("WorldGenSettings".into()))??;

        let game_type = nbt
            .int("GameType")
            .map(|i| GameType::from_i32(i))
            .ok_or(SculkParseError::MissingField("GameType".into()))?;

        let hardcore = get_bool(&nbt, "hardcore");
        let initialized = get_bool(&nbt, "initialized");

        let last_played = nbt
            .long("LastPlayed")
            .ok_or(SculkParseError::MissingField("LastPlayed".into()))?;

        let level_name = get_owned_string(&nbt, "LevelName")?;
        let map_features = nbt.byte("MapFeatures").map(|b| b != 0).unwrap_or(true);

        let player = if let Some(nbt) = nbt.compound("Player") {
            Some(Player::from_compound_nbt(&nbt)?)
        } else {
            None
        };

        let raining = get_bool(&nbt, "raining");
        let rain_time = nbt
            .int("rainTime")
            .ok_or(SculkParseError::MissingField("rainTime".into()))?;

        let random_seed = nbt.long("RandomSeed");
        let size_on_disk = nbt.long("SizeOnDisk");

        let spawn_x = nbt
            .int("SpawnX")
            .ok_or(SculkParseError::MissingField("SpawnX".into()))?;
        let spawn_y = nbt
            .int("SpawnY")
            .ok_or(SculkParseError::MissingField("SpawnY".into()))?;
        let spawn_z = nbt
            .int("SpawnZ")
            .ok_or(SculkParseError::MissingField("SpawnZ".into()))?;

        let thundering = get_bool(&nbt, "thundering");
        let thunder_time = nbt
            .int("thunderTime")
            .ok_or(SculkParseError::MissingField("thunderTime".into()))?;

        let time = nbt
            .long("Time")
            .ok_or(SculkParseError::MissingField("Time".into()))?;

        let version = nbt
            .int("version")
            .ok_or(SculkParseError::MissingField("version".into()))?;

        let version_data = nbt
            .compound("Version")
            .map(|nbt| VersionData::from_compound_nbt(&nbt))
            .ok_or(SculkParseError::MissingField("Version".into()))??;

        let wandering_trader_id = nbt.int_array("WanderingTraderId").map(Uuid::from);

        let wandering_trader_spawn_chance =
            nbt.int("WanderingTraderSpawnChance")
                .ok_or(SculkParseError::MissingField(
                    "WanderingTraderSpawnChance".into(),
                ))?;

        let wandering_trader_spawn_delay =
            nbt.int("WanderingTraderSpawnDelay")
                .ok_or(SculkParseError::MissingField(
                    "WanderingTraderSpawnDelay".into(),
                ))?;

        let was_modded = get_bool(&nbt, "WasModded");

        Ok(Level {
            allow_commands,
            border_center_x,
            border_center_z,
            border_damage_per_block,
            border_size,
            border_safe_zone,
            border_size_lerp_target,
            border_size_lerp_time,
            border_warning_blocks,
            border_warning_time,
            clear_weather_time,
            custom_boss_events,
            datapacks,
            data_version,
            day_time,
            difficulty,
            difficulty_locked,
            dimension_data,
            enabled_features,
            game_rules,
            world_gen_settings,
            game_type,
            hardcore,
            initialized,
            last_played,
            level_name,
            map_features,
            player,
            raining,
            rain_time,
            random_seed,
            size_on_disk,
            spawn_x,
            spawn_y,
            spawn_z,
            thundering,
            thunder_time,
            time,
            version,
            version_data,
            wandering_trader_id,
            wandering_trader_spawn_chance,
            wandering_trader_spawn_delay,
            was_modded,
        })
    }
}

impl Level {
    /// Get the data version from the level.dat file.
    pub fn get_data_version(nbt: &simdnbt::borrow::NbtCompound) -> Result<i32, SculkParseError> {
        nbt.int("DataVersion")
            .ok_or(SculkParseError::MissingField("DataVersion".into()))
    }
}

#[cfg(test)]
#[test]
fn simple_level_test() {
    use flate2::read::GzDecoder;
    use std::io::{Cursor, Read};

    let mut file = std::fs::File::open("test_data/level.dat").unwrap();
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

    let _ = Level::from_compound_nbt(&nbt).unwrap();
}
