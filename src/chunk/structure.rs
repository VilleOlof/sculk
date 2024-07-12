//! Honestly i dont know what the fuck this shit is but uh i want to fully support everything in chunks soooo?

use crate::{
    error::SculkParseError,
    kv::KVPair,
    traits::FromCompoundNbt,
    util::{get_owned_mutf8str, get_owned_optional_mutf8str},
};
use simdnbt::{borrow::NbtCompound, Mutf8Str};
use std::{borrow::Cow, collections::HashMap};

/// Structure data in this chunk.
#[derive(Debug, Clone, PartialEq)]
pub struct Structures<'a> {
    /// Coordinates of chunks that contain Starts.   
    /// Each 64-bit number of this array represents a chunk coordinate (i.e. block coordinate / 16), with its X packed into the low (least significant) 32 bits and Z packed into the high (most significant) 32 bits.
    pub references: KVPair<'a, Vec<i64>>,

    /// Structures that are yet to be generated, stored by general type. Some parts of the structures may have already been generated. Completely generated structures are removed by setting their id to "INVALID" and removing all other tags  
    /// Only the structures that can spawn in this dimension are stored, for example, EndCity is stored only in the end chunks.
    pub starts: KVPair<'a, Structure<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Structure<'a> {
    /// Bounding box of the entire structure (remaining Children). Value is 6 ints: the minimum X, Y, and Z coordinates followed by the maximum X, Y, and Z coordinates. Absent if id is `INVALID`.  
    /// `BB`
    pub bb: Option<[i32; 6]>,

    /// The biome id this structure is in. Absent if id is `INVALID`.
    pub biome: Option<Cow<'a, Mutf8Str>>,

    ///  List of structure pieces making up this structure, that were not generated yet. Absent if id is `INVALID`.  
    /// `Children`
    // pub children: Vec<StructureChild<'a>>,

    /// Chunk X coordinate of the start of the structure. Absent if id is `INVALID`.  
    /// `ChunkX`
    pub chunk_x: Option<i32>,

    /// Chunk Z coordinate of the start of the structure. Absent if id is `INVALID`.  
    /// `ChunkZ`
    pub chunk_z: Option<i32>,

    /// If there's no structure of this kind in this chunk, this value is `INVALID`, else it's the structure name.
    pub id: Cow<'a, Mutf8Str>,

    /// (Monument only) List of chunks that have had their piece of the structure created. Absent if id is `INVALID`.  
    /// `Processed`
    pub processed: Option<Vec<ProcessedChunk>>,

    /// (Village only) Whether the village generated at least 3 non-roads. Absent if id is `INVALID`.  
    /// `Valid`
    pub valid: Option<bool>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProcessedChunk {
    /// The chunk's X position (chunk coordinates, not block coordinates).  
    /// `X`
    pub x: i32,
    /// The chunk's Z position.  
    /// `Z`
    pub z: i32,
}

impl<'a> FromCompoundNbt for Structures<'a> {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let references = nbt
            .compound("References")
            .map(|nbt| KVPair::<'a, Vec<i64>>::from_compound_nbt(&nbt))
            .ok_or(SculkParseError::MissingField("References".into()))??;

        let starts = nbt
            .compound("starts")
            .map(|nbt| KVPair::<'a, Structure<'a>>::from_compound_nbt(&nbt))
            .ok_or(SculkParseError::MissingField("starts".into()))??;

        Ok(Structures { references, starts })
    }
}

impl<'a> FromCompoundNbt for KVPair<'a, Vec<i64>> {
    fn from_compound_nbt(nbt: &NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let mut map = HashMap::new();

        for (key, value) in nbt.iter() {
            let key = key.to_string();
            let value = value
                .long_array()
                .ok_or(SculkParseError::InvalidField(key.clone()))?;

            map.insert(Cow::Owned(key), value);
        }

        Ok(KVPair::new(map))
    }
}

impl<'a> FromCompoundNbt for KVPair<'a, Structure<'a>> {
    fn from_compound_nbt(nbt: &NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let mut map = HashMap::new();

        for (key, value) in nbt.iter() {
            let key = key.to_string();
            let value = value
                .compound()
                .ok_or(SculkParseError::InvalidField(key.clone()))?;
            let value = Structure::from_compound_nbt(&value)?;

            map.insert(Cow::Owned(key), value);
        }

        Ok(KVPair::new(map))
    }
}

impl<'a> FromCompoundNbt for Structure<'a> {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let bb = nbt.int_array("BB").map(|arr| {
            let mut bb = [0; 6];
            for (i, val) in arr.iter().enumerate() {
                bb[i] = *val;
            }
            bb
        });

        let biome = get_owned_optional_mutf8str(&nbt, "biome");
        let chunk_x = nbt.int("ChunkX");
        let chunk_z = nbt.int("ChunkZ");
        let id = get_owned_mutf8str(&nbt, "id")?;

        let processed = if let Some(list) = nbt.list("processed") {
            let mut processed = vec![];
            for nbt in list
                .compounds()
                .ok_or(SculkParseError::InvalidField("processed".into()))?
            {
                processed.push(ProcessedChunk::from_compound_nbt(&nbt)?);
            }
            Some(processed)
        } else {
            None
        };

        let valid = nbt.byte("Valid").map(|b| b != 0);

        Ok(Structure {
            bb,
            biome,
            chunk_x,
            chunk_z,
            id,
            processed,
            valid,
        })
    }
}

impl FromCompoundNbt for ProcessedChunk {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let x = nbt
            .int("X")
            .ok_or(SculkParseError::MissingField("X".into()))?;
        let z = nbt
            .int("Z")
            .ok_or(SculkParseError::MissingField("Z".into()))?;

        Ok(ProcessedChunk { x, z })
    }
}
