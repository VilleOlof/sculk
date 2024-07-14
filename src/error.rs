use thiserror::Error;

/// Any error that can occur while parsing Nbt data.
#[derive(Error, Debug)]
pub enum SculkParseError {
    /// Error when a field is missing.
    #[error("Missing field: {0}")]
    MissingField(String),

    /// Error when a fields value is invalid.
    #[error("Invalid field: {0}")]
    InvalidField(String),

    /// Internal simdnbt error.
    #[error("Simdnbt error: {0}")]
    NbtError(#[from] simdnbt::Error),

    /// Internal simdnbt deserialize error.
    #[error("Deserialize error: {0}")]
    DeserializeError(#[from] simdnbt::DeserializeError),

    /// Error when the root nbt is none.
    #[error("Missing Nbt data")]
    NoNbt,

    /// Error when the block entity is unsupported.
    #[error("Unsupported block entity: {0}")]
    UnsupportedBlockEntity(String),
}
