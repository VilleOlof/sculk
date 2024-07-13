use std::io::Cursor;

use crate::{
    entity::Entity, error::SculkParseError, traits::FromCompoundNbt, util::get_t_compound_vec,
    BlockEntity,
};
use section::ChunkSection;
use status::ChunkStatus;
use structure::Structures;
use tile_tick::TileTick;

mod section;
mod status;
mod structure;
mod tile_tick;

#[derive(Debug, Clone, PartialEq)]
pub struct Chunk<'a> {
    /// Version of the chunk NBT structure.  
    /// `DataVersion`
    pub data_version: i32,

    /// X position of the chunk (in absolute chunks from world x, z origin, not relative to the region).  
    /// `xPos`
    pub x_pos: i32,
    /// Z position of the chunk (in absolute chunks from world x, z origin, not relative to the region).  
    /// `zPos`
    pub z_pos: i32,
    /// Lowest Y section position in the chunk (e.g. -4 in 1.18).  
    /// `yPos`
    pub y_pos: i32,

    /// Defines the world generation status of this chunk.  
    /// `Status`
    pub status: ChunkStatus,

    /// Tick when the chunk was last saved.  
    /// `LastUpdate`
    pub last_update: i64,

    /// List of Compounds, each tag is a section (also known as sub-chunk). All sections in the world's height are present in this list, even those who are empty (filled with air).  
    pub sections: Vec<ChunkSection<'a>>,

    /// Each Compound in this list defines a block entity in the chunk.
    pub block_entities: Vec<BlockEntity<'a>>,

    /// Only for proto-chunks (not confirmed for 1.18 format).   
    /// `CarvingMasks`
    pub carving_masks: Option<CarvingMasks>,

    /// Several different heightmaps corresponding to 256 values compacted at 9 bits per value (lowest being 0, highest being 384, both values inclusive). The 9-bit values are stored in an array of 37  Longs, each containing 7 values ( A Long = 64 bits, 7×9 = 63; the last bit is unused). In versions prior to 1.16 the heightmaps were stored in 36  Long values, where the bits were arranged in an uninterrupted "stream" across all values, resulting in all 2304 bits being used. The 9-bit values are unsigned, and indicate the amount of blocks above the bottom of the world. This means that converting a world to 1.18 adds 64 to every value.  
    /// `Heightmaps`
    pub height_maps: HeightMaps,

    ///  A List of 16 lists that store positions of light sources per chunk section as shorts, only for proto-chunks (not confirmed for 1.18 format).  
    /// `Lights`
    pub lights: Vec<Vec<i16>>,

    /// A list of entities in the proto-chunks, used when generating. As of 1.17, this list is not present for fully generated chunks and entities are moved to a separated region files once the chunk is generated, see Entity format for more details (not confirmed for 1.18 format).  
    /// `Entities`
    pub entities: Option<Vec<Entity<'a>>>,

    /// A list of Compounds  
    /// Each Compound in this list is an "active" liquid in this chunk waiting to be updated. See [Tile Tick Format](https://minecraft.wiki/w/Chunk_format#Tile_tick_format).
    pub fluid_ticks: Vec<TileTick<'a>>,

    /// A list of Compounds  
    /// Each Compound in this list is an "active" block in this chunk waiting to be updated. These are used to save the state of redstone machines or falling sand, and other activity. See [Tile Tick Format](https://minecraft.wiki/w/Chunk_format#Tile_tick_format).
    pub block_ticks: Vec<TileTick<'a>>,

    /// The cumulative number of ticks players have been in this chunk. Note that this value increases faster when more players are in the chunk. Used for [Regional Difficulty](https://minecraft.wiki/w/Difficulty#Regional_difficulty).  
    /// `InhabitedTime`
    pub inhabited_time: i64,

    /// This appears to be biome blending data, although more testing is needed to confirm. ​  
    /// `blending_data`
    ///
    /// *Ha funny wiki*
    pub blending_data: Option<BlendingData>,

    /// A List of 24  Lists that store the positions of blocks that need to receive an update when a proto-chunk turns into a full chunk, packed in  Shorts. Each list corresponds to specific section in the height of the chunk.  
    /// `PostProcessing`
    pub post_processing: Option<Vec<i16>>,

    /// Structure data in this chunk.
    pub structures: Structures<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlendingData {
    pub min_section: i32,
    pub max_section: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HeightMaps {
    /// MOTION_BLOCKING
    pub motion_blocking: Vec<i64>,
    /// MOTION_BLOCKING_NO_LEAVES
    pub motion_blocking_no_leaves: Vec<i64>,
    /// OCEAN_FLOOR
    pub ocean_floor: Vec<i64>,
    /// OCEAN_FLOOR_WG
    pub ocean_floor_wg: Vec<i64>,
    /// WORLD_SURFACE
    pub world_surface: Vec<i64>,
    /// WORLD_SURFACE_WG
    pub world_surface_wg: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CarvingMasks {
    /// A series of bits indicating whether a cave has been dug at a specific position, stored in a byte array.  
    /// `AIR`
    pub air: Vec<i8>,
    /// A series of bits indicating whether an underwater cave has been dug at a specific position, stored in a byte array.  
    /// `LIQUID`
    pub liquid: Vec<i8>,
}

//
impl<'a> FromCompoundNbt for Chunk<'a> {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let data_version = nbt
            .int("DataVersion")
            .ok_or(SculkParseError::MissingField("DataVersion".into()))?;

        let x_pos = nbt
            .int("xPos")
            .ok_or(SculkParseError::MissingField("xPos".into()))?;
        let z_pos = nbt
            .int("zPos")
            .ok_or(SculkParseError::MissingField("zPos".into()))?;
        let y_pos = nbt
            .int("yPos")
            .ok_or(SculkParseError::MissingField("yPos".into()))?;

        let status = nbt
            .string("Status")
            .map(|s| ChunkStatus::from_str(s.to_str().as_ref()))
            .ok_or(SculkParseError::MissingField("Status".into()))??;

        let last_update = nbt
            .long("LastUpdate")
            .ok_or(SculkParseError::MissingField("LastUpdate".into()))?;

        let sections = get_t_compound_vec(&nbt, "sections", ChunkSection::from_compound_nbt)?;
        let block_entities =
            get_t_compound_vec(&nbt, "block_entities", BlockEntity::from_compound_nbt)?;

        let carving_masks = if let Some(nbt) = nbt.compound("CarvingMasks") {
            Some(CarvingMasks::from_compound_nbt(&nbt)?)
        } else {
            None
        };

        let height_maps = nbt
            .compound("Heightmaps")
            .map(|nbt| HeightMaps::from_compound_nbt(&nbt))
            .ok_or(SculkParseError::MissingField("Heightmaps".into()))??;

        let lights = if let Some(lights) = nbt.list("Lights") {
            let lights = lights
                .lists()
                .ok_or(SculkParseError::InvalidField("Lights".into()))?;

            let mut lights_vec: Vec<Vec<i16>> = Vec::with_capacity(lights.len());

            for light in lights {
                let inner_lights = light
                    .shorts()
                    .ok_or(SculkParseError::InvalidField("Lights".into()))?;

                lights_vec.push(inner_lights.iter().map(|s| *s).collect());
            }

            lights_vec
        } else {
            Vec::new()
        };

        let entities = match get_t_compound_vec(&nbt, "Entities", Entity::from_compound_nbt) {
            Ok(entities) => Some(entities),
            Err(SculkParseError::MissingField(_)) => None,
            Err(e) => return Err(e),
        };
        let fluid_ticks = get_t_compound_vec(&nbt, "fluid_ticks", TileTick::from_compound_nbt)?;
        let block_ticks = get_t_compound_vec(&nbt, "block_ticks", TileTick::from_compound_nbt)?;

        let inhabited_time = nbt
            .long("InhabitedTime")
            .ok_or(SculkParseError::MissingField("InhabitedTime".into()))?;

        let blending_data = if let Some(nbt) = nbt.compound("blending_data") {
            Some(BlendingData::from_compound_nbt(&nbt)?)
        } else {
            None
        };

        // let post_processing = if let Some(list) = nbt.list("PostProcessing") {
        //     if list.empty() {
        //         None
        //     } else {
        //         let post_processing = list
        //             .shorts()
        //             .ok_or(SculkParseError::InvalidField("PostProcessing:Inner".into()))?
        //             .iter()
        //             .map(|l| *l)
        //             .collect();

        //         Some(post_processing)
        //     }
        // } else {
        //     None
        // };
        let post_processing = None;

        let structures = nbt
            .compound("structures")
            .map(|nbt| Structures::from_compound_nbt(&nbt))
            .ok_or(SculkParseError::MissingField("structures".into()))??;

        Ok(Chunk {
            data_version,
            x_pos,
            z_pos,
            y_pos,
            status,
            last_update,
            sections,
            block_entities,
            carving_masks,
            height_maps,
            lights,
            entities,
            fluid_ticks,
            block_ticks,
            inhabited_time,
            blending_data,
            post_processing,
            structures,
        })
    }
}

impl FromCompoundNbt for BlendingData {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let min_section = nbt
            .int("min_section")
            .ok_or(SculkParseError::MissingField("min_section".into()))?;
        let max_section = nbt
            .int("max_section")
            .ok_or(SculkParseError::MissingField("max_section".into()))?;

        Ok(BlendingData {
            min_section,
            max_section,
        })
    }
}

impl FromCompoundNbt for HeightMaps {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let motion_blocking = nbt.long_array("MOTION_BLOCKING").unwrap_or_default();
        let motion_blocking_no_leaves = nbt
            .long_array("MOTION_BLOCKING_NO_LEAVES")
            .unwrap_or_default();
        let ocean_floor = nbt.long_array("OCEAN_FLOOR").unwrap_or_default();
        let ocean_floor_wg = nbt.long_array("OCEAN_FLOOR_WG").unwrap_or_default();
        let world_surface = nbt.long_array("WORLD_SURFACE").unwrap_or_default();
        let world_surface_wg = nbt.long_array("WORLD_SURFACE_WG").unwrap_or_default();

        Ok(HeightMaps {
            motion_blocking,
            motion_blocking_no_leaves,
            ocean_floor,
            ocean_floor_wg,
            world_surface,
            world_surface_wg,
        })
    }
}

impl FromCompoundNbt for CarvingMasks {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let air = nbt
            .byte_array("AIR")
            .map(|nbt| nbt.iter().map(|b| *b as i8).collect())
            .unwrap_or_default();

        let liquid = nbt
            .byte_array("LIQUID")
            .map(|nbt| nbt.iter().map(|b| *b as i8).collect())
            .unwrap_or_default();

        Ok(CarvingMasks { air, liquid })
    }
}

impl<'a> Chunk<'a> {
    pub fn from_bytes(bytes: &'a [u8]) -> Result<Self, SculkParseError> {
        let nbt = simdnbt::borrow::read(&mut Cursor::new(bytes))?;

        let nbt = match nbt {
            simdnbt::borrow::Nbt::Some(nbt) => nbt,
            simdnbt::borrow::Nbt::None => return Err(SculkParseError::NoNbt),
        };
        let compound = nbt.as_compound();

        Chunk::from_compound_nbt(&compound)
    }
}
