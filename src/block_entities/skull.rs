use std::borrow::Cow;

use simdnbt::Mutf8Str;

use crate::{
    error::SculkParseError,
    traits::FromCompoundNbt,
    util::{get_owned_mutf8str, get_owned_optional_mutf8str},
    uuid::Uuid,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Skull<'a> {
    /// Optional. The custom name of the skull.
    pub custom_name: Option<Cow<'a, Mutf8Str>>,

    /// Optional. The sound event this skull plays when played with a note block.
    pub note_block_sound: Option<Cow<'a, Mutf8Str>>,

    /// Information about the owner of this player head. If defined as a string, corresponds to  name.
    pub profile: SkullProfile<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SkullProfile<'a> {
    Name(Cow<'a, Mutf8Str>),

    Profile(Profile<'a>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Profile<'a> {
    /// The name of a player profile, i.e. its username. If this is the only tag provided, it will be resolved into the other ones below. Optional.
    pub name: Option<Cow<'a, Mutf8Str>>,

    /// The UUID of the owner. Used to update the other tags when the chunk loads or the holder logs in, in case the owner's name has changed. Optional.
    pub id: Option<Uuid>,

    /// A list of properties. Optional.
    pub properties: Option<Vec<Property<'a>>>,
}

/// A single property.
#[derive(Debug, Clone, PartialEq)]
pub struct Property<'a> {
    /// The name of the property. Can be textures.
    pub name: Cow<'a, Mutf8Str>,

    /// The texture data, encoded in base64.
    pub value: Cow<'a, Mutf8Str>,

    /// The signature. Optional.
    pub signature: Option<Cow<'a, Mutf8Str>>,
}

impl<'a> FromCompoundNbt for Skull<'a> {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let custom_name = get_owned_optional_mutf8str(&nbt, "custom_name");
        let note_block_sound = get_owned_optional_mutf8str(&nbt, "note_block_sound");

        let profile = nbt
            .compound("profile")
            // Here we use the parent nbt tag instead of the compound
            // since its an enum that depends on the type of the profile tag
            .map(|_| SkullProfile::from_compound_nbt(&nbt))
            .ok_or(SculkParseError::MissingField("profile".into()))??;

        Ok(Skull {
            custom_name,
            note_block_sound,
            profile,
        })
    }
}

impl<'a> FromCompoundNbt for SkullProfile<'a> {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        if let Some(name) = get_owned_optional_mutf8str(&nbt, "profile") {
            return Ok(SkullProfile::Name(name));
        } else if let Some(compound) = nbt.compound("profile") {
            let profile = Profile::from_compound_nbt(&compound)?;
            return Ok(SkullProfile::Profile(profile));
        } else {
            return Err(SculkParseError::InvalidField("profile".into()));
        }
    }
}

impl<'a> SkullProfile<'a> {
    pub fn from_component_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        if let Some(name) = get_owned_optional_mutf8str(&nbt, "minecraft:profile") {
            return Ok(SkullProfile::Name(name));
        } else if let Some(compound) = nbt.compound("minecraft:profile") {
            let profile = Profile::from_compound_nbt(&compound)?;
            return Ok(SkullProfile::Profile(profile));
        } else {
            return Err(SculkParseError::InvalidField("minecraft:profile".into()));
        }
    }
}

impl<'a> FromCompoundNbt for Profile<'a> {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let name = get_owned_optional_mutf8str(&nbt, "name");

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

impl<'a> FromCompoundNbt for Property<'a> {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let name = get_owned_mutf8str(&nbt, "name")?;
        let value = get_owned_mutf8str(&nbt, "value")?;
        let signature = get_owned_optional_mutf8str(&nbt, "signature");

        Ok(Property {
            name,
            value,
            signature,
        })
    }
}
