use crate::error::SculkParseError;

#[derive(Debug, Clone, PartialEq)]
pub enum ChunkStatus {
    Empty,
    StructureStarts,
    StructureReferences,
    Biomes,
    Noise,
    Surface,
    Carvers,
    Features,
    Light,
    Spawn,
    Full,
    Unknown(String),
}

impl ChunkStatus {
    pub fn from_str(v: &str) -> Result<Self, SculkParseError> {
        match v {
            "minecraft:empty" => Ok(ChunkStatus::Empty),
            "minecraft:structure_starts" => Ok(ChunkStatus::StructureStarts),
            "minecraft:structure_references" => Ok(ChunkStatus::StructureReferences),
            "minecraft:biomes" => Ok(ChunkStatus::Biomes),
            "minecraft:noise" => Ok(ChunkStatus::Noise),
            "minecraft:surface" => Ok(ChunkStatus::Surface),
            "minecraft:carvers" => Ok(ChunkStatus::Carvers),
            "minecraft:features" => Ok(ChunkStatus::Features),
            "minecraft:light" => Ok(ChunkStatus::Light),
            "minecraft:spawn" => Ok(ChunkStatus::Spawn),
            "minecraft:full" => Ok(ChunkStatus::Full),
            _ => Ok(ChunkStatus::Unknown(v.to_string())),
        }
    }
}
