#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(from = "&str", into = "String"))]
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

impl From<&str> for ChunkStatus {
    fn from(value: &str) -> Self {
        match value {
            "minecraft:empty" => ChunkStatus::Empty,
            "minecraft:structure_starts" => ChunkStatus::StructureStarts,
            "minecraft:structure_references" => ChunkStatus::StructureReferences,
            "minecraft:biomes" => ChunkStatus::Biomes,
            "minecraft:noise" => ChunkStatus::Noise,
            "minecraft:surface" => ChunkStatus::Surface,
            "minecraft:carvers" => ChunkStatus::Carvers,
            "minecraft:features" => ChunkStatus::Features,
            "minecraft:light" => ChunkStatus::Light,
            "minecraft:spawn" => ChunkStatus::Spawn,
            "minecraft:full" => ChunkStatus::Full,
            _ => ChunkStatus::Unknown(value.to_string()),
        }
    }
}

impl From<ChunkStatus> for String {
    fn from(value: ChunkStatus) -> Self {
        match value {
            ChunkStatus::Empty => String::from("minecraft:empty"),
            ChunkStatus::StructureStarts => String::from("minecraft:structure_starts"),
            ChunkStatus::StructureReferences => String::from("minecraft:structure_references"),
            ChunkStatus::Biomes => String::from("minecraft:biomes"),
            ChunkStatus::Noise => String::from("minecraft:noise"),
            ChunkStatus::Surface => String::from("minecraft:surface"),
            ChunkStatus::Carvers => String::from("minecraft:carvers"),
            ChunkStatus::Features => String::from("minecraft:features"),
            ChunkStatus::Light => String::from("minecraft:light"),
            ChunkStatus::Spawn => String::from("minecraft:spawn"),
            ChunkStatus::Full => String::from("minecraft:full"),
            ChunkStatus::Unknown(value) => value,
        }
    }
}
