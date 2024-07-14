use crate::{error::SculkParseError, traits::FromCompoundNbt, util::get_bool};

#[derive(Debug, Clone, PartialEq)]
pub struct DimensionData {
    /// Data for The End  
    /// `1`
    pub end: EndData,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EndData {
    /// Data for the ender dragon fight. Appears only after the End is entered.  
    /// `DragonFight`
    pub dragon_fight: DragonFight,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DragonFight {
    /// Location of the End's exit portal that the ender dragon flies to upon its death.   
    /// `ExitPortalLocation`
    pub exit_portal_location: Option<ExitPortalLocation>,

    /// Contains a list of locations of the End gateway portals that haven't been spawned.  
    ///
    /// Entry: The angle of a future gateway, from 0 to 19. 0 is east of the exit portal, and numbers increase clockwise.  
    ///
    /// `Gateways`
    pub gateways: Vec<i32>,

    ///  If the dragon is currently alive.  
    /// `DragonKilled`
    pub dragon_killed: bool,

    /// The least significant bits of the current Ender Dragon's [Universally Unique IDentifier](http://docs.oracle.com/javase/6/docs/api/java/util/UUID.html). This is joined with DragonUUIDMost to form the dragon's unique ID.  
    /// `DragonUUIDLeast`
    pub dragon_uuid_least: Option<i64>,

    /// The most significant bits of the current Ender Dragon's [Universally Unique IDentifier](http://docs.oracle.com/javase/6/docs/api/java/util/UUID.html). This is joined with DragonUUIDLeast to form the dragon's unique ID.  
    /// `DragonUUIDMost`
    pub dragon_uuid_most: Option<i64>,

    /// If the ender dragon has ever been defeated. Used to determine EXP given by dragon.  
    /// `PreviouslyKilled`
    pub previously_killed: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExitPortalLocation {
    /// The X coordinate of the portal.  
    /// `X`
    pub x: i8,
    /// The Y coordinate of the portal.  
    /// `Y`
    pub y: i8,
    /// The Z coordinate of the portal.  
    /// `Z`
    pub z: i8,
}

impl FromCompoundNbt for DimensionData {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let end = nbt
            .compound("1")
            .map(|nbt| EndData::from_compound_nbt(&nbt))
            .ok_or(SculkParseError::MissingField("1".into()))??;

        Ok(Self { end })
    }
}

impl FromCompoundNbt for EndData {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let dragon_fight = nbt
            .compound("DragonFight")
            .map(|nbt| DragonFight::from_compound_nbt(&nbt))
            .ok_or(SculkParseError::MissingField("DragonFight".into()))??;

        Ok(Self { dragon_fight })
    }
}

impl FromCompoundNbt for DragonFight {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let exit_portal_location = if let Some(nbt) = nbt.compound("ExitPortalLocation") {
            Some(ExitPortalLocation::from_compound_nbt(&nbt)?)
        } else {
            None
        };

        let gateways = nbt
            .int_array("Gateways")
            .ok_or(SculkParseError::MissingField("Gateways".into()))?;
        let dragon_killed = get_bool(&nbt, "DragonKilled");

        let dragon_uuid_least = nbt.long("DragonUUIDLeast");
        let dragon_uuid_most = nbt.long("DragonUUIDMost");

        let previously_killed = get_bool(&nbt, "PreviouslyKilled");

        Ok(Self {
            exit_portal_location,
            gateways,
            dragon_killed,
            dragon_uuid_least,
            dragon_uuid_most,
            previously_killed,
        })
    }
}

impl FromCompoundNbt for ExitPortalLocation {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let x = nbt
            .byte("X")
            .ok_or(SculkParseError::MissingField("X".into()))?;
        let y = nbt
            .byte("Y")
            .ok_or(SculkParseError::MissingField("Y".into()))?;
        let z = nbt
            .byte("Z")
            .ok_or(SculkParseError::MissingField("Z".into()))?;

        Ok(Self { x, y, z })
    }
}
