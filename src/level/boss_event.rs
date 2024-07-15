//! Data about custom boss events.

use crate::{
    error::SculkParseError,
    kv::KVPair,
    traits::FromCompoundNbt,
    util::{get_bool, get_owned_string},
    uuid::Uuid,
};

use std::collections::HashMap;

/// A custom boss bar event.
#[derive(Debug, Clone, PartialEq)]
pub struct CustomBossEvent {
    ///  A list of players that may see this boss bar.  
    /// `Players`
    pub players: Vec<Uuid>,

    /// ID of the color of the bossbar. Also sets the color of the display text of the bossbar, provided that the display text does not explicitly define a color for itself. See [color codes](https://minecraft.wiki/w/Formatting_codes#Color_codes) for accepted values.  
    /// `Color`
    pub color: String,

    /// If the bossbar should create fog.  
    /// `CreateWorldFog`
    pub create_world_fog: bool,

    /// If the bossbar should darken the sky.  
    /// `DarkenScreen`
    pub darken_screen: bool,

    /// The maximum health of the bossbar.  
    /// `Max`
    pub max: i32,

    /// The current health of the bossbar.  
    /// `Value`
    pub value: i32,

    /// The display name of the bossbar as a [JSON text component](https://minecraft.wiki/w/Commands#Raw_JSON_text).  
    /// `Name`
    pub name: String,

    /// The ID of the overlay to be shown over the health bar.  
    /// `Overlay`
    pub overlay: BossEventOverlay,

    /// If the bossbar should initiate boss music.  
    /// `PlayBossMusic`
    pub play_boss_music: bool,

    /// If the bossbar should be visible to the listed players.  
    /// `Visible`
    pub visible: bool,
}

/// The overlay type to use for the boss bar.
#[derive(Debug, Clone, PartialEq)]
pub enum BossEventOverlay {
    /// A smooth progress bar.
    Progress,
    /// A notched progress bar with 6 segments.
    Notched6,
    /// A notched progress bar with 10 segments.
    Notched10,
    /// A notched progress bar with 12 segments.
    Notched12,
    /// A notched progress bar with 20 segments.
    Notched20,
    /// An unknown overlay type.
    Unknown(String),
}

impl From<&str> for BossEventOverlay {
    fn from(value: &str) -> Self {
        match value {
            "progress" => BossEventOverlay::Progress,
            "notched_6" => BossEventOverlay::Notched6,
            "notched_10" => BossEventOverlay::Notched10,
            "notched_12" => BossEventOverlay::Notched12,
            "notched_20" => BossEventOverlay::Notched20,
            _ => BossEventOverlay::Unknown(value.into()),
        }
    }
}

impl FromCompoundNbt for KVPair<CustomBossEvent> {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let mut map = HashMap::new();

        for (key, value) in nbt.iter() {
            let key = key.to_string();
            let value = match value.compound() {
                Some(string) => CustomBossEvent::from_compound_nbt(&string)?,
                None => continue,
            };

            map.insert(key, value);
        }

        Ok(KVPair::new(map))
    }
}

impl FromCompoundNbt for CustomBossEvent {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let players = if let Some(nbt) = nbt.list("Players") {
            let list = nbt
                .int_arrays()
                .ok_or(SculkParseError::InvalidField("Players".into()))?;

            let mut vec: Vec<Uuid> = vec![];

            for item in list.into_iter() {
                let int_array = item.to_vec();
                vec.push(Uuid::from(int_array));
            }

            vec
        } else {
            vec![]
        };

        let color = get_owned_string(&nbt, "Color")?;
        let create_world_fog = get_bool(&nbt, "CreateWorldFog");
        let darken_screen = get_bool(&nbt, "DarkenScreen");
        let max = nbt
            .int("Max")
            .ok_or(SculkParseError::MissingField("Max".into()))?;
        let value = nbt
            .int("Value")
            .ok_or(SculkParseError::MissingField("Value".into()))?;

        let name = get_owned_string(&nbt, "Name")?;

        let overlay = nbt
            .string("Overlay")
            .map(|nbt| BossEventOverlay::from(nbt.to_str().as_ref()))
            .ok_or(SculkParseError::MissingField("Overlay".into()))?;

        let play_boss_music = get_bool(&nbt, "PlayBossMusic");
        let visible = get_bool(&nbt, "Visible");

        Ok(Self {
            players,
            color,
            create_world_fog,
            darken_screen,
            max,
            value,
            name,
            overlay,
            play_boss_music,
            visible,
        })
    }
}
