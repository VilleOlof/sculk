use crate::{
    color::Color,
    error::SculkParseError,
    traits::FromCompoundNbt,
    util::{get_bool, get_owned_optional_string, get_owned_string, get_t_compound_vec},
};

/// Represents a map in the game.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Map {
    /// How zoomed in the map is (it is in 2scale wide blocks square per pixel, even for 0, where the map is 1:1). Default 3, minimum 0 and maximum 4.   
    pub scale: i8,

    /// Resource location for a dimension.  
    pub dimension: String,

    /// true (default) indicates that a positional arrow should be shown when the map is near its center coords. false indicates that the position arrow should never be shown.  
    /// `trackingPosition`
    pub tracking_position: bool,

    /// true has been locked in a cartography table.  
    /// `unlimitedTracking`
    pub unlimited_tracking: bool,

    /// Center of map according to real world by X.  
    /// `xCenter`
    pub x_center: i32,

    /// Center of map according to real world by Z.  
    /// `zCenter`
    pub z_center: i32,

    /// List of banner markers added to this map. May be empty.  
    pub banners: Vec<MapBanner>,

    /// List map markers added to this map. May be empty.  
    pub frames: Vec<MapFrame>,

    /// Width * Height array of color values (16384 entries for a default 128×128 map). Color can be accessed via the following method: colorID = Colors[widthOffset + heightOffset * width], where (widthOffset==0, heightOffset==0) is left upper point.
    pub colors: Vec<u8>,

    /// The version the map was created. If not present, defaults to 1343 (1.12.2)
    /// `DataVersion`
    pub data_version: i32,
}

/// The position of a map marker/banner.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MapPos {
    /// The x-position
    /// `X`
    x: i32,
    /// The y-position
    /// `Y`
    y: i32,
    /// The z-position
    /// `Z`
    z: i32,
}

/// A banner on a map.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MapBanner {
    ///  The color of the banner.  
    /// `Color`
    pub color: Color,

    /// The custom name of the banner, in JSON text. May not exist.  
    /// `Name`
    pub name: Option<String>,

    /// The block position of the banner in the world.  
    /// `Pos`
    pub pos: MapPos,
}

/// A frame on a map.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MapFrame {
    /// Arbitrary unique value for the marker.  
    /// `EntityId`
    pub entity_id: i32,

    /// The rotation of the marker, ranging from 0 to 360.  
    /// `Rotation`
    pub rotation: i32,

    /// The block position of the marker in the world.  
    /// `Pos`
    pub pos: MapPos,
}

impl FromCompoundNbt for MapPos {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let x = nbt
            .int("X")
            .ok_or(SculkParseError::MissingField("X".into()))?;

        let y = nbt
            .int("Y")
            .ok_or(SculkParseError::MissingField("Y".into()))?;

        let z = nbt
            .int("Z")
            .ok_or(SculkParseError::MissingField("Z".into()))?;

        Ok(Self { x, y, z })
    }
}

impl FromCompoundNbt for MapFrame {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let entity_id = nbt
            .int("EntityId")
            .ok_or(SculkParseError::MissingField("EntityId".into()))?;

        let rotation = nbt
            .int("Rotation")
            .ok_or(SculkParseError::MissingField("Rotation".into()))?;

        let pos = nbt
            .compound("Pos")
            .map(|nbt| MapPos::from_compound_nbt(&nbt))
            .ok_or(SculkParseError::MissingField("Pos".into()))??;

        Ok(Self {
            entity_id,
            rotation,
            pos,
        })
    }
}

impl FromCompoundNbt for MapBanner {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let color = if let Some(s) = nbt.string("Color") {
            match Color::from_str(s.to_str().as_ref()) {
                Some(color) => color,
                None => return Err(SculkParseError::InvalidField("Color".into())),
            }
        } else {
            return Err(SculkParseError::MissingField("Color".into()));
        };

        let name = get_owned_optional_string(&nbt, "Name");

        let pos = nbt
            .compound("Pos")
            .map(|nbt| MapPos::from_compound_nbt(&nbt))
            .ok_or(SculkParseError::MissingField("Pos".into()))??;

        Ok(Self { color, name, pos })
    }
}

impl FromCompoundNbt for Map {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let data_version = nbt
            .int("DataVersion")
            .ok_or(SculkParseError::MissingField("DataVersion".into()))?;

        // we then just go one step deeper since the rest of all tags are inside of `data`
        let nbt = nbt
            .compound("data")
            .ok_or(SculkParseError::MissingField("data".into()))?;

        let scale = nbt
            .byte("scale")
            .ok_or(SculkParseError::MissingField("scale".into()))?;

        let dimension = get_owned_string(&nbt, "dimension")?;
        let tracking_position = nbt.byte("trackingPosition").map(|b| b != 0).unwrap_or(true);
        let unlimited_tracking = get_bool(&nbt, "unlimitedTracking");

        let x_center = nbt
            .int("xCenter")
            .ok_or(SculkParseError::MissingField("xCenter".into()))?;

        let z_center = nbt
            .int("zCenter")
            .ok_or(SculkParseError::MissingField("zCenter".into()))?;

        let banners = get_t_compound_vec(&nbt, "banners", MapBanner::from_compound_nbt)?;
        let frames = get_t_compound_vec(&nbt, "frames", MapFrame::from_compound_nbt)?;

        let colors = nbt
            .byte_array("colors")
            .map(|b| b.to_vec())
            .unwrap_or_default();

        Ok(Self {
            scale,
            dimension,
            tracking_position,
            unlimited_tracking,
            x_center,
            z_center,
            banners,
            frames,
            colors,
            data_version,
        })
    }
}

#[cfg(test)]
#[test]
fn map_test() {
    use flate2::read::GzDecoder;
    use std::io::{Cursor, Read};

    let mut file = std::fs::File::open("test_data/map_0.dat").unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    let mut src = &contents[..];

    let mut src_decoder = GzDecoder::new(&mut src);
    let mut input = Vec::new();
    if src_decoder.read_to_end(&mut input).is_err() {
        input = contents;
    }
    let mut input_stream = Cursor::new(&input[..]);

    let nbt = simdnbt::borrow::read(&mut input_stream).unwrap().unwrap();
    let nbt = nbt.as_compound();

    let _ = Map::from_compound_nbt(&nbt).unwrap();
}
