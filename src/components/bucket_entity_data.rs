use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct BucketEntityData {
    /// Turns into NoAI entity tag for all bucketable entities.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "NoAI")]
    pub no_ai: Option<bool>,

    /// Turns into Silent entity tag for all bucketable entities.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Silent")]
    pub silent: Option<bool>,

    /// Turns into NoGravity entity tag for all bucketable entities.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "NoGravity")]
    pub no_gravity: Option<bool>,

    /// Turns into Glowing entity tag for all bucketable entities.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Glowing")]
    pub glowing: Option<bool>,

    /// Turns into Invulnerable entity tag for all bucketable entities.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Invulnerable")]
    pub invulnerable: Option<bool>,

    /// Turns into Health entity tag for all bucketable entities.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Health")]
    pub health: Option<f32>,

    /// Turns into Age entity tag for axolotls.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Age")]
    pub age: Option<i32>,

    /// Turns into Variant entity tag for axolotls.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Variant")]
    pub variant: Option<i32>,

    /// Turns into the expiry time of the memory module has_hunting_cooldown for axolotls.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "HuntingCooldown")]
    pub hunting_cooldown: Option<i64>,

    /// Turns into Variant entity tag for tropical fish.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "BucketVariantTag")]
    pub bucket_variant_tag: Option<i32>,
}
