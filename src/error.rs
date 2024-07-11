use thiserror::Error;

#[derive(Error, Debug)]
pub enum SculkParseError {
    #[error("Missing field: {0}")]
    MissingField(String),

    #[error("Invalid field: {0}")]
    InvalidField(String),

    #[error("Simdnbt error: {0}")]
    NbtError(#[from] simdnbt::Error),

    #[error("Deserialize error: {0}")]
    DeserializeError(#[from] simdnbt::DeserializeError),

    #[error("Missing Nbt data")]
    NoNbt,

    #[error("Unsupported block entity: {0}")]
    UnsupportedBlockEntity(String),
}
