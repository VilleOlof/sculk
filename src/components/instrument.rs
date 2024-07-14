//! Instrument component.

use crate::{error::SculkParseError, traits::FromCompoundNbt, util::get_owned_mutf8str};
use simdnbt::Mutf8Str;
use std::borrow::Cow;

/// (referenced by ID or inlined)
#[derive(Debug, Clone, PartialEq)]
pub enum Instrument<'a> {
    /// The ID of the instrument.
    ID(Cow<'a, Mutf8Str>),
    /// The instrument data.
    Inline(InstrumentData<'a>),
}

/// Instrument data.
#[derive(Debug, Clone, PartialEq)]
pub struct InstrumentData<'a> {
    /// sound event (referenced by ID or inlined)
    pub sound_event: SoundEvent<'a>,

    /// A non-negative integer for how long the use duration is.
    pub use_duration: i32,

    /// A non-negative float for the range of the sound.
    pub range: f32,
}

/// (referenced by ID or inlined)
#[derive(Debug, Clone, PartialEq)]
pub enum SoundEvent<'a> {
    /// The ID of the sound event.
    ID(Cow<'a, Mutf8Str>),
    /// The sound event data.
    Inline(SoundEventData<'a>),
}

/// Sound event data.
#[derive(Debug, Clone, PartialEq)]
pub struct SoundEventData<'a> {
    /// A resource location that points to a sound file.
    pub sound_id: Cow<'a, Mutf8Str>,

    /// Optional. The fixed range of the sound. If omitted, the sound will have a variable range.
    pub range: Option<f32>,
}

impl<'a> FromCompoundNbt for Instrument<'a> {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        if let Some(id) = nbt.string("minecraft:instrument") {
            return Ok(Instrument::ID(Cow::Owned(id.to_owned())));
        } else if let Some(compound) = nbt.compound("minecraft:instrument") {
            let data = InstrumentData::from_compound_nbt(&compound)?;

            return Ok(Instrument::Inline(data));
        } else {
            return Err(SculkParseError::MissingField("minecraft:instrument".into()));
        }
    }
}

impl<'a> FromCompoundNbt for InstrumentData<'a> {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let sound_event = SoundEvent::from_compound_nbt(&nbt)?;

        let use_duration = nbt
            .int("use_duration")
            .ok_or(SculkParseError::MissingField("use_duration".into()))?;
        let range = nbt
            .float("range")
            .ok_or(SculkParseError::MissingField("range".into()))?;

        Ok(InstrumentData {
            sound_event,
            use_duration,
            range,
        })
    }
}

impl<'a> FromCompoundNbt for SoundEvent<'a> {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        if let Some(id) = nbt.string("sound_event") {
            return Ok(SoundEvent::ID(Cow::Owned(id.to_owned())));
        } else if let Some(compound) = nbt.compound("sound_event") {
            let data = SoundEventData::from_compound_nbt(&compound)?;

            return Ok(SoundEvent::Inline(data));
        } else {
            return Err(SculkParseError::MissingField("sound_event".into()));
        }
    }
}

impl<'a> FromCompoundNbt for SoundEventData<'a> {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let sound_id = get_owned_mutf8str(&nbt, "sound_id")?;
        let range = nbt.float("range");

        Ok(SoundEventData { sound_id, range })
    }
}
