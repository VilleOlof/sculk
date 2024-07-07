use serde::Deserialize;

/// The version of Minecraft that this library is designed to work with.  
/// Formatted exactly as minecraft versions are.
pub const MC_VERSION: &str = "1.21";

/// Used to get a default value when deserializing a boolean.
pub fn default_as_true() -> bool {
    false
}

pub fn i8_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let i = i8::deserialize(deserializer)?;
    Ok(i != 0)
}
