/// The version of Minecraft that this library is designed to work with.  
/// Formatted exactly as minecraft versions are.
pub const MC_VERSION: &str = "1.21";

/// Used to get a default value when deserializing a boolean.
pub fn default_as_true() -> bool {
    false
}
