use crate::{
    error::SculkParseError,
    kv::KVPair,
    traits::FromCompoundNbt,
    util::{get_owned_string, get_t_compound_vec},
};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChunkSection {
    /// The Y position of this section.  
    /// `Y`
    pub y: i8,

    pub block_states: Option<BlockStates>,

    pub biomes: Option<Biomes>,

    pub block_light: Option<Vec<u8>>,

    pub sky_light: Option<Vec<u8>>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Biomes {
    /// Set of different biomes used in this particular section.  
    pub palette: Vec<PaletteNoProps>,

    /// A packed array of 64 indices pointing to the palette, stored in an array of 64-bit integers ( Longs).
    ///
    /// If only one biome is present in the palette, this field is not required and the biome fills the whole section.
    /// All indices are the same length: the minimum amount of bits required to represent the largest index in the palette. These indices do not have a minimum size. Different chunks can have different lengths for the indices.
    pub data: Option<Vec<i64>>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BlockStates {
    /// Set of different block states used in this particular section.  
    pub palette: Vec<Palette>,

    /// A packed array of 4096 indices pointing to the palette, stored in an array of 64-bit integers ( Longs).  
    ///
    /// If only one block state is present in the palette, this field is not required and the block fills the whole section.
    /// All indices are the same length. This length is set to the minimum amount of bits required to represent the largest index in the palette, and then set to a minimum size of 4 bits. Since 1.16, the indices are not packed across multiple elements of the array, meaning that if there is no more space in a given 64-bit integer for the whole next index, it starts instead at the first (lowest) bit of the next 64-bit integer. Different sections of a chunk can have different lengths for the indices.
    pub data: Option<Vec<i64>>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Palette {
    /// Block resource location.  
    /// `Name`
    pub name: String,

    /// List of block state properties, with name being the name of the block state property.   
    /// `Properties`
    pub properties: KVPair<String>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PaletteNoProps {
    /// Block resource location.  
    /// `Name`
    pub name: String,
}

impl FromCompoundNbt for ChunkSection {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let y = nbt
            .byte("Y")
            .ok_or(SculkParseError::MissingField("Y".into()))?;

        let block_states = if let Some(nbt) = nbt.compound("block_states") {
            Some(BlockStates::from_compound_nbt(&nbt)?)
        } else {
            None
        };
        let biomes = if let Some(nbt) = nbt.compound("biomes") {
            Some(Biomes::from_compound_nbt(&nbt)?)
        } else {
            None
        };

        let block_light = nbt
            .byte_array("block_light")
            .map(|x| x.iter().map(|x| *x).collect::<Vec<u8>>());

        let sky_light = nbt
            .byte_array("sky_light")
            .map(|x| x.iter().map(|x| *x).collect::<Vec<u8>>());

        Ok(ChunkSection {
            y,
            block_states,
            biomes,
            block_light,
            sky_light,
        })
    }
}

impl FromCompoundNbt for Biomes {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let palette = if let Some(palette) = nbt.list("palette") {
            let palette = palette
                .strings()
                .ok_or(SculkParseError::InvalidField("palette".into()))?;

            palette
                .iter()
                .map(|s| PaletteNoProps {
                    name: (*s).to_string(),
                })
                .collect::<Vec<PaletteNoProps>>()
        } else {
            Vec::new()
        };
        // apparently biomes are just a list of strings and no compound
        // let palette = get_t_compound_vec(&nbt, "palette", PaletteNoProps::from_compound_nbt)?;
        let data = nbt.long_array("data");

        Ok(Biomes { palette, data })
    }
}

impl FromCompoundNbt for BlockStates {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let palette = get_t_compound_vec(&nbt, "palette", Palette::from_compound_nbt)?;
        let data = nbt.long_array("data");

        Ok(BlockStates { palette, data })
    }
}

impl FromCompoundNbt for PaletteNoProps {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let name = get_owned_string(&nbt, "name")?;

        Ok(PaletteNoProps { name })
    }
}

impl FromCompoundNbt for Palette {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let name = get_owned_string(&nbt, "Name")?;
        let properties = KVPair::<String>::from_compound_nbt(&nbt)?;

        Ok(Palette { name, properties })
    }
}
