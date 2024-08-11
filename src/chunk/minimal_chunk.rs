use std::io::Cursor;

use crate::{
    block_entity::BlockEntity, error::SculkParseError, traits::FromCompoundNbt,
    util::get_t_compound_vec,
};

use super::{
    section::ChunkSection, status::ChunkStatus, structure::Structures, tile_tick::TileTick,
    BlendingData,
};

/// A chunk that has minimal amount of data fields to represent the physical chunk.  
/// Used on chunks that has been often force upgraded to newer versions but yet not loaded ingame.  
///
/// Like when you run `java.exe -jar server.jar --forceUpgrade --eraseCache`
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MinimalChunk {
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

    /// `PostProcessing`
    pub post_processing: Option<Vec<i16>>,

    /// Defines the world generation status of this chunk.  
    /// `Status`
    pub status: ChunkStatus,

    /// This appears to be biome blending data, although more testing is needed to confirm. ​  
    /// `blending_data`
    ///
    /// *Ha funny wiki*
    pub blending_data: Option<BlendingData>,

    /// Each Compound in this list defines a block entity in the chunk.
    pub block_entities: Vec<BlockEntity>,

    /// A list of Compounds  
    /// Each Compound in this list is an "active" liquid in this chunk waiting to be updated. See [Tile Tick Format](https://minecraft.wiki/w/Chunk_format#Tile_tick_format).
    pub fluid_ticks: Vec<TileTick>,

    /// A list of Compounds  
    /// Each Compound in this list is an "active" block in this chunk waiting to be updated. These are used to save the state of redstone machines or falling sand, and other activity. See [Tile Tick Format](https://minecraft.wiki/w/Chunk_format#Tile_tick_format).
    pub block_ticks: Vec<TileTick>,

    /// List of Compounds, each tag is a section (also known as sub-chunk). All sections in the world's height are present in this list, even those who are empty (filled with air).  
    pub sections: Vec<ChunkSection>,

    /// Structure data in this chunk.
    pub structures: Structures,
}

impl FromCompoundNbt for MinimalChunk {
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
            .map(|s| ChunkStatus::from(s.to_str().as_ref()))
            .ok_or(SculkParseError::MissingField("Status".into()))?;

        let sections = get_t_compound_vec(&nbt, "sections", ChunkSection::from_compound_nbt)?;
        let block_entities =
            get_t_compound_vec(&nbt, "block_entities", BlockEntity::from_compound_nbt)?;

        let fluid_ticks = get_t_compound_vec(&nbt, "fluid_ticks", TileTick::from_compound_nbt)?;
        let block_ticks = get_t_compound_vec(&nbt, "block_ticks", TileTick::from_compound_nbt)?;

        let blending_data = if let Some(nbt) = nbt.compound("blending_data") {
            Some(BlendingData::from_compound_nbt(&nbt)?)
        } else {
            None
        };

        let post_processing = None;

        let structures = nbt
            .compound("structures")
            .map(|nbt| Structures::from_compound_nbt(&nbt))
            .ok_or(SculkParseError::MissingField("structures".into()))??;

        Ok(MinimalChunk {
            data_version,
            x_pos,
            z_pos,
            y_pos,
            status,
            sections,
            block_entities,
            fluid_ticks,
            block_ticks,
            blending_data,
            post_processing,
            structures,
        })
    }
}

impl MinimalChunk {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, SculkParseError> {
        let nbt = simdnbt::borrow::read(&mut Cursor::new(bytes))?;

        let nbt = match nbt {
            simdnbt::borrow::Nbt::Some(nbt) => nbt,
            simdnbt::borrow::Nbt::None => return Err(SculkParseError::NoNbt),
        };
        let compound = nbt.as_compound();

        MinimalChunk::from_compound_nbt(&compound)
    }
}
