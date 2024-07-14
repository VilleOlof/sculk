//! All gamemode types.

/// A gamemode type.
#[derive(Debug, Clone, PartialEq)]
pub enum GameType {
    /// Survival mode.
    Survival,
    /// Creative mode.
    Creative,
    /// Adventure mode.
    Adventure,
    /// Spectator mode.
    Spectator,
    /// An unknown game mode.
    Unknown(i32),
}

impl GameType {
    /// Converts an `i32` to a `GameType`.
    pub fn from_i32(i: i32) -> GameType {
        match i {
            0 => GameType::Survival,
            1 => GameType::Creative,
            2 => GameType::Adventure,
            3 => GameType::Spectator,
            _ => GameType::Unknown(i),
        }
    }
}
