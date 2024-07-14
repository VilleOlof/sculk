//! Bucket entity data component.

use crate::traits::FromCompoundNbt;
use simdnbt::borrow::NbtCompound;

/// Represents the bucket entity data component.  
/// Like axolotl in a bucket.
#[derive(Debug, Clone, PartialEq)]
pub struct BucketEntityData {
    /// Turns into NoAI entity tag for all bucketable entities.
    ///
    /// `NoAI`
    pub no_ai: Option<bool>,

    /// Turns into Silent entity tag for all bucketable entities.
    ///
    /// `Silent`
    pub silent: Option<bool>,

    /// Turns into NoGravity entity tag for all bucketable entities.
    ///
    /// `NoGravity`
    pub no_gravity: Option<bool>,

    /// Turns into Glowing entity tag for all bucketable entities.
    ///
    /// `Glowing`
    pub glowing: Option<bool>,

    /// Turns into Invulnerable entity tag for all bucketable entities.
    ///
    /// `Invulnerable`
    pub invulnerable: Option<bool>,

    /// Turns into Health entity tag for all bucketable entities.
    ///
    /// `Health`
    pub health: Option<f32>,

    /// Turns into Age entity tag for axolotls.
    ///
    /// `Age`
    pub age: Option<i32>,

    /// Turns into Variant entity tag for axolotls.
    ///
    /// `Variant`
    pub variant: Option<i32>,

    /// Turns into the expiry time of the memory module has_hunting_cooldown for axolotls.
    ///
    /// `HuntingCooldown`
    pub hunting_cooldown: Option<i64>,

    /// Turns into Variant entity tag for tropical fish.
    ///
    /// `BucketVariantTag`
    pub bucket_variant_tag: Option<i32>,
}

impl FromCompoundNbt for BucketEntityData {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        fn bool(nbt: &NbtCompound, key: &'static str) -> Option<bool> {
            nbt.int(key).map(|b| b != 0)
        }

        let no_ai = bool(&nbt, "NoAI");
        let silent = bool(&nbt, "Silent");
        let no_gravity = bool(&nbt, "NoGravity");
        let glowing = bool(&nbt, "Glowing");
        let invulnerable = bool(&nbt, "Invulnerable");
        let health = nbt.float("Health");
        let age = nbt.int("Age");
        let variant = nbt.int("Variant");
        let hunting_cooldown = nbt.long("HuntingCooldown");
        let bucket_variant_tag = nbt.int("BucketVariantTag");

        Ok(BucketEntityData {
            no_ai,
            silent,
            no_gravity,
            glowing,
            invulnerable,
            health,
            age,
            variant,
            hunting_cooldown,
            bucket_variant_tag,
        })
    }
}
