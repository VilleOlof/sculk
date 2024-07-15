use crate::{
    error::SculkParseError,
    traits::FromCompoundNbt,
    util::{get_doubles_array, get_owned_string},
    uuid::Uuid,
};

#[derive(Debug, Clone, PartialEq)]
pub struct CalibratedSculkSensor {
    /// The frequency of the last vibration.
    pub last_vibration_frequency: i32,

    /// The vibration event listener for this sculk shrieker, sculk sensor, or calibrated sculk sensor.
    pub listener: Listener,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Listener {
    /// Only exists if there is an incoming vibration.
    pub event: Option<Event>,

    /// How many ticks remain until triggered by the vibration. Set to 0 if there is no incoming vibration
    pub event_delay: i32,

    /// The data of the vibration selector.​
    pub selector: Selector,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Event {
    /// The distance between this vibration's source and the block.
    pub distance: f32,

    /// The resource location of the vibration event that caused the current incoming vibration.
    pub game_event: String,

    /// The coordinates of the source of this vibration.  
    ///
    /// X coordinate.  
    /// Y coordinate.  
    /// Z coordinate.  
    pub pos: [f64; 3],

    /// If the vibration was caused by a projectile, this is the UUID of the entity that launched the projectile. Does not exist if vibration was not caused by a projectile.
    pub projectile_owner: Option<Uuid>,

    /// The UUID of the entity that caused the vibration. Does not exist if vibration was not caused by an entity.
    pub source: Option<Uuid>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Selector {
    /// The game time when the vibration occurs, or -1 if there is no vibration to choose from.​
    pub tick: i64,

    /// Candidate game event
    pub event: Option<Event>,
}

impl FromCompoundNbt for CalibratedSculkSensor {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let last_vibration_frequency =
            nbt.int("last_vibration_frequency")
                .ok_or(SculkParseError::MissingField(
                    "last_vibration_frequency".into(),
                ))?;

        let listener = nbt
            .compound("listener")
            .map(|nbt| Listener::from_compound_nbt(&nbt))
            .ok_or(SculkParseError::MissingField("listener".into()))??;

        Ok(CalibratedSculkSensor {
            last_vibration_frequency,
            listener,
        })
    }
}

impl FromCompoundNbt for Listener {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let event = if let Some(event) = nbt.compound("event") {
            Some(Event::from_compound_nbt(&event)?)
        } else {
            None
        };

        let event_delay = nbt
            .int("event_delay")
            .ok_or(SculkParseError::MissingField("event_delay".into()))?;

        let selector = nbt
            .compound("selector")
            .map(|nbt| Selector::from_compound_nbt(&nbt))
            .ok_or(SculkParseError::MissingField("selector".into()))??;

        Ok(Listener {
            event,
            event_delay,
            selector,
        })
    }
}

impl FromCompoundNbt for Selector {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let tick = nbt
            .long("tick")
            .ok_or(SculkParseError::MissingField("tick".into()))?;

        let event = if let Some(nbt) = nbt.compound("event") {
            Some(Event::from_compound_nbt(&nbt)?)
        } else {
            None
        };

        Ok(Selector { tick, event })
    }
}

impl FromCompoundNbt for Event {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let distance = nbt
            .float("distance")
            .ok_or(SculkParseError::MissingField("distance".into()))?;
        let game_event = get_owned_string(&nbt, "game_event")?;

        let pos = get_doubles_array(&nbt, "pos").and_then(|arr| {
            if arr.len() == 3 {
                Ok([arr[0], arr[1], arr[2]])
            } else {
                Err(SculkParseError::InvalidField("pos".into()))
            }
        })?;
        let projectile_owner = nbt.int_array("projectile_owner").map(Uuid::from);
        let source = nbt.int_array("source").map(Uuid::from);

        Ok(Event {
            distance,
            game_event,
            pos,
            projectile_owner,
            source,
        })
    }
}
