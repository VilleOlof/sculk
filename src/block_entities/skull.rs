use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Skull<'a> {
    /// Optional. The custom name of the skull.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_name: Option<Cow<'a, str>>,

    /// Optional. The sound event this skull plays when played with a note block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_block_sound: Option<Cow<'a, str>>,

    /// Information about the owner of this player head. If defined as a string, corresponds to  name.
    #[serde(borrow)]
    pub profile: SkullProfile<'a>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum SkullProfile<'a> {
    #[serde(borrow)]
    Name(Cow<'a, str>),

    #[serde(borrow)]
    Profile(Profile<'a>),
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Profile<'a> {
    /// The name of a player profile, i.e. its username. If this is the only tag provided, it will be resolved into the other ones below. Optional.
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Cow<'a, str>>,

    /// The UUID of the owner. Used to update the other tags when the chunk loads or the holder logs in, in case the owner's name has changed. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i128>,

    /// A list of properties. Optional.
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<Property<'a>>>,
}

/// A single property.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Property<'a> {
    /// The name of the property. Can be textures.
    #[serde(borrow)]
    pub namne: Cow<'a, str>,

    /// The texture data, encoded in base64.
    #[serde(borrow)]
    pub value: Cow<'a, str>,

    /// The signature. Optional.
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<Cow<'a, str>>,
}
