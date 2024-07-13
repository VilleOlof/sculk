#[derive(Debug, Clone, PartialEq)]
pub enum GameType {
    Survival,
    Creative,
    Adventure,
    Spectator,
    Unknown(i32),
}

impl GameType {
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
