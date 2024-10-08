//! Settings used when generating the world.  

use crate::{error::SculkParseError, kv::KVPair, traits::FromCompoundNbt};

/// Settings about the world generation.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WorldGenSettings {
    /// Used to determine if the bonus chest should appear near the spawn point when a world is first entered. Available only in single player.  
    /// `bonus_chest`
    pub bonus_chest: Option<bool>,

    /// the numerical seed of the world  
    /// `seed`
    pub seed: i64,

    /// Whether structures should be generated or not  
    /// `generate_features`
    pub generate_features: bool,

    /// Contains all the dimensions.  
    /// The value here is [generator settings](https://minecraft.wiki/w/Custom_dimension).  
    /// `dimensions`
    #[cfg_attr(feature = "serde", serde(skip))]
    pub dimensions: KVPair<simdnbt::owned::NbtCompound>,
}

impl FromCompoundNbt for WorldGenSettings {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let bonus_chest = nbt.byte("bonus_chest").map(|b| b != 0);
        let seed = nbt
            .long("seed")
            .ok_or(SculkParseError::MissingField("seed".into()))?;
        let generate_features = nbt
            .byte("generate_features")
            .map(|b| b != 0)
            .unwrap_or(true);

        let dimensions = nbt
            .compound("dimensions")
            .map(|nbt| KVPair::from_compound_nbt(&nbt))
            .ok_or(SculkParseError::MissingField("dimensions".into()))??;

        Ok(Self {
            bonus_chest,
            seed,
            generate_features,
            dimensions,
        })
    }
}
