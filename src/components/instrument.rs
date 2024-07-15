//! Instrument component.

use crate::{error::SculkParseError, traits::FromCompoundNbt, util::get_owned_string};

/// (referenced by ID or inlined)
#[derive(Debug, Clone, PartialEq)]
pub enum Instrument {
    /// The ID of the instrument.
    ID(String),
    /// The instrument data.
    Inline(InstrumentData),
}

/// Instrument data.
#[derive(Debug, Clone, PartialEq)]
pub struct InstrumentData {
    /// sound event (referenced by ID or inlined)
    pub sound_event: SoundEvent,

    /// A non-negative integer for how long the use duration is.
    pub use_duration: i32,

    /// A non-negative float for the range of the sound.
    pub range: f32,
}

/// (referenced by ID or inlined)
#[derive(Debug, Clone, PartialEq)]
pub enum SoundEvent {
    /// The ID of the sound event.
    ID(String),
    /// The sound event data.
    Inline(SoundEventData),
}

/// Sound event data.
#[derive(Debug, Clone, PartialEq)]
pub struct SoundEventData {
    /// A resource location that points to a sound file.
    pub sound_id: String,

    /// Optional. The fixed range of the sound. If omitted, the sound will have a variable range.
    pub range: Option<f32>,
}

impl FromCompoundNbt for Instrument {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        if let Some(id) = nbt.string("minecraft:instrument") {
            return Ok(Instrument::ID(id.to_string()));
        } else if let Some(compound) = nbt.compound("minecraft:instrument") {
            let data = InstrumentData::from_compound_nbt(&compound)?;

            return Ok(Instrument::Inline(data));
        } else {
            return Err(SculkParseError::MissingField("minecraft:instrument".into()));
        }
    }
}

impl FromCompoundNbt for InstrumentData {
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

impl FromCompoundNbt for SoundEvent {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        if let Some(id) = nbt.string("sound_event") {
            return Ok(SoundEvent::ID(id.to_string()));
        } else if let Some(compound) = nbt.compound("sound_event") {
            let data = SoundEventData::from_compound_nbt(&compound)?;

            return Ok(SoundEvent::Inline(data));
        } else {
            return Err(SculkParseError::MissingField("sound_event".into()));
        }
    }
}

impl FromCompoundNbt for SoundEventData {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let sound_id = get_owned_string(&nbt, "sound_id")?;
        let range = nbt.float("range");

        Ok(SoundEventData { sound_id, range })
    }
}
