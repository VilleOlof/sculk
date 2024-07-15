use crate::{
    error::SculkParseError,
    traits::FromCompoundNbt,
    util::{get_owned_optional_string, get_owned_string},
    uuid::Uuid,
};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Skull {
    /// Optional. The custom name of the skull.
    pub custom_name: Option<String>,

    /// Optional. The sound event this skull plays when played with a note block.
    pub note_block_sound: Option<String>,

    /// Information about the owner of this player head. If defined as a string, corresponds to  name.
    pub profile: Option<SkullProfile>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SkullProfile {
    Name(String),

    Profile(Profile),
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Profile {
    /// The name of a player profile, i.e. its username. If this is the only tag provided, it will be resolved into the other ones below. Optional.
    pub name: Option<String>,

    /// The UUID of the owner. Used to update the other tags when the chunk loads or the holder logs in, in case the owner's name has changed. Optional.
    pub id: Option<Uuid>,

    /// A list of properties. Optional.
    pub properties: Option<Vec<Property>>,
}

/// A single property.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Property {
    /// The name of the property. Can be textures.
    pub name: String,

    /// The texture data, encoded in base64.
    pub value: String,

    /// The signature. Optional.
    pub signature: Option<String>,
}

impl FromCompoundNbt for Skull {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let custom_name = get_owned_optional_string(&nbt, "custom_name");
        let note_block_sound = get_owned_optional_string(&nbt, "note_block_sound");

        let profile = match SkullProfile::from_compound_nbt(&nbt) {
            Ok(profile) => Some(profile),
            Err(SculkParseError::InvalidField(_)) => None,
            Err(e) => return Err(e),
        };

        Ok(Skull {
            custom_name,
            note_block_sound,
            profile,
        })
    }
}

impl FromCompoundNbt for SkullProfile {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        if let Some(name) = get_owned_optional_string(&nbt, "profile") {
            return Ok(SkullProfile::Name(name));
        } else if let Some(compound) = nbt.compound("profile") {
            let profile = Profile::from_compound_nbt(&compound)?;
            return Ok(SkullProfile::Profile(profile));
        } else {
            return Err(SculkParseError::InvalidField("profile".into()));
        }
    }
}

impl SkullProfile {
    pub fn from_component_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        if let Some(name) = get_owned_optional_string(&nbt, "minecraft:profile") {
            return Ok(SkullProfile::Name(name));
        } else if let Some(compound) = nbt.compound("minecraft:profile") {
            let profile = Profile::from_compound_nbt(&compound)?;
            return Ok(SkullProfile::Profile(profile));
        } else {
            return Err(SculkParseError::InvalidField("minecraft:profile".into()));
        }
    }
}

impl FromCompoundNbt for Profile {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let name = get_owned_optional_string(&nbt, "name");

        let id = nbt.int_array("id").map(Uuid::from);

        let properties = if let Some(props) = nbt.list("properties") {
            let mut properties = vec![];

            for prop in props
                .compounds()
                .ok_or(SculkParseError::InvalidField("properties".into()))?
            {
                let prop_item = Property::from_compound_nbt(&prop)?;
                properties.push(prop_item);
            }

            Some(properties)
        } else {
            None
        };

        Ok(Profile {
            name,
            id,
            properties,
        })
    }
}

impl FromCompoundNbt for Property {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let name = get_owned_string(&nbt, "name")?;
        let value = get_owned_string(&nbt, "value")?;
        let signature = get_owned_optional_string(&nbt, "signature");

        Ok(Property {
            name,
            value,
            signature,
        })
    }
}
