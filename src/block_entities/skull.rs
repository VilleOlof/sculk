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
#[serde(untagged)]
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

#[cfg(test)]
#[test]
fn basic_test() {
    use fastnbt::nbt;

    let nbt = nbt!({
        "custom_name": "Some name",
        "note_block_sound": "block.note_block.bell",
        "profile": "Grian"
    });

    let skull: Skull = fastnbt::from_value(&nbt).unwrap();

    assert_eq!(skull.custom_name.as_ref().unwrap(), "Some name");
    assert_eq!(
        skull.note_block_sound.as_ref().unwrap(),
        "block.note_block.bell"
    );
    assert_eq!(skull.profile, SkullProfile::Name("Grian".into()));

    let nbt = fastnbt::to_value(&skull).unwrap();

    assert_eq!(nbt, nbt);
}

#[cfg(test)]
#[test]
fn extended_test() {
    use fastnbt::nbt;

    let nbt = nbt!({
        "custom_name": "Some name",
        "note_block_sound": "block.note_block.bell",
        "profile": {
            "name": "Grian",
            // TODO: This for some reason hangs the test
            // "id": [I; 1, 2, 3, 4],
            "properties": [
                {
                    "namne": "textures",
                    "value": "<base64>",
                    "signature": "my signature"
                }
            ]
        }
    });

    let skull: Skull = fastnbt::from_value(&nbt).unwrap();

    assert_eq!(skull.custom_name.as_ref().unwrap(), "Some name");
    assert_eq!(
        skull.note_block_sound.as_ref().unwrap(),
        "block.note_block.bell"
    );

    let profile = match &skull.profile {
        SkullProfile::Profile(profile) => profile,
        _ => panic!("Expected profile"),
    };

    assert_eq!(profile.name.as_ref().unwrap(), "Grian");
    // See above
    // assert_eq!(profile.id.unwrap(), 79228162514264337593543950336);

    let property = profile.properties.as_ref().unwrap().get(0).unwrap();

    assert_eq!(property.namne.as_ref(), "textures");
    assert_eq!(property.value.as_ref(), "<base64>");
    assert_eq!(property.signature.as_ref().unwrap(), "my signature");

    let serialized_nbt = fastnbt::to_value(&skull).unwrap();

    assert_eq!(nbt, serialized_nbt);
}
