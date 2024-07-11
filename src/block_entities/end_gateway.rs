use crate::{error::SculkParseError, traits::FromCompoundNbt, util::get_bool};

#[derive(Debug, Clone, PartialEq)]
pub struct EndGateway {
    /// Age of the portal, in ticks. This is used to determine when the beam is rendered.
    ///
    /// `Age`
    pub age: i32,

    ///T eleports entities directly to the ExitPortal coordinates instead of near them.
    ///
    /// `ExactTeleport`
    pub exact_teleport: bool,

    /// Location entities are teleported to when entering the portal.
    ///
    /// `ExitPortal`
    pub exit_portal: ExitPortal,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExitPortal {
    /// X coordinate of target location.
    ///
    /// `X`
    pub x: i32,

    /// Y coordinate of target location.
    ///
    /// `Y`
    pub y: i32,

    /// Z coordinate of target location.
    ///
    /// `Z`
    pub z: i32,
}

impl FromCompoundNbt for ExitPortal {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        Ok(ExitPortal {
            x: nbt
                .int("X")
                .ok_or(SculkParseError::MissingField("X".into()))?,
            y: nbt
                .int("Y")
                .ok_or(SculkParseError::MissingField("Y".into()))?,
            z: nbt
                .int("Z")
                .ok_or(SculkParseError::MissingField("Z".into()))?,
        })
    }
}

impl FromCompoundNbt for EndGateway {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let age = nbt
            .int("Age")
            .ok_or(SculkParseError::MissingField("Age".into()))?;
        let exact_teleport = get_bool(&nbt, "ExactTeleport");
        let exit_portal = nbt
            .compound("ExitPortal")
            .map(|nbt| ExitPortal::from_compound_nbt(&nbt))
            .ok_or(SculkParseError::MissingField("ExitPortal".into()))??;

        Ok(EndGateway {
            age,
            exact_teleport,
            exit_portal,
        })
    }
}
