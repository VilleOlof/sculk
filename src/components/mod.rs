use std::{borrow::Cow, collections::HashMap, ops::Deref};

use attribute_modifiers::AttributeModifier;
use banner_patterns::BannerPattern;
use base_color::BaseColor;
use bees::Bee;
use dyed_color::DyedColor;
use simdnbt::Mutf8Str;

use crate::{
    block_entity::NoCoordinatesBlockEntity, error::SculkParseError, item::ItemWithNoSlot,
    rarity::Rarity, traits::FromCompoundNbt, util::KeyValuePair,
};

pub mod attribute_modifiers;
pub mod banner_patterns;
pub mod base_color;
pub mod bees;
pub mod block_state;
pub mod bucket_entity_data;
pub mod can_break;
pub mod container;
pub mod container_loot;
pub mod custom_data;
pub mod dyed_color;

type InternalMap<'a> = HashMap<String, Component<'a>>;

#[derive(Debug, Clone, PartialEq)]
pub struct Components<'a>(InternalMap<'a>);

impl<'a> Deref for Components<'a> {
    type Target = InternalMap<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> FromCompoundNbt for Components<'a> {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let nbt_components = nbt
            .compound("components")
            .ok_or(SculkParseError::MissingField("components".into()))?;

        let mut map: InternalMap = HashMap::new();

        for (key, value) in nbt_components.iter() {
            let key = key.to_string();

            let component: Component<'a> = match key.as_str() {
                "minecraft:attribute_modifiers" => {
                    // since the root value is either list or compound, we need to pass parent nbt.
                    Component::AttributeModifiers(AttributeModifier::from_compound_nbt(&nbt)?)
                }
                "minecraft:banner_patterns" => {
                    let patterns = value
                        .list()
                        .ok_or(SculkParseError::InvalidField(
                            "minecraft:banner_patterns".into(),
                        ))?
                        .compounds()
                        .ok_or(SculkParseError::InvalidField(
                            "minecraft:banner_patterns".into(),
                        ))?
                        .into_iter()
                        .map(|nbt| BannerPattern::from_compound_nbt(&nbt))
                        .collect::<Result<Vec<BannerPattern>, SculkParseError>>()?;

                    Component::BannerPatterns(patterns)
                }
                "minecraft:base_color" => Component::BaseColor(BaseColor::from_compound_nbt(&nbt)?),
                "minecraft:bees" => {
                    let bees = value
                        .list()
                        .ok_or(SculkParseError::InvalidField("minecraft:bees".into()))?
                        .compounds()
                        .ok_or(SculkParseError::InvalidField("minecraft:bees".into()))?
                        .into_iter()
                        .map(|nbt| Bee::from_compound_nbt(&nbt))
                        .collect::<Result<Vec<Bee>, SculkParseError>>()?;

                    Component::Bees(bees)
                }
                "minecraft:block_entity_data" => {
                    let nbt = value.compound().ok_or(SculkParseError::InvalidField(
                        "minecraft:block_entity_data".into(),
                    ))?;
                    Component::BlockEntityData(NoCoordinatesBlockEntity::from_compound_nbt(&nbt)?)
                }
                "minecraft:block_state" => {
                    let nbt = value.compound().ok_or(SculkParseError::InvalidField(
                        "minecraft:block_state".into(),
                    ))?;
                    Component::BlockState(block_state::BlockState::from_compound_nbt(&nbt)?)
                }
                "minecraft:bucket_entity_data" => {
                    let nbt = value.compound().ok_or(SculkParseError::InvalidField(
                        "minecraft:bucket_entity_data".into(),
                    ))?;
                    Component::BucketEntityData(
                        bucket_entity_data::BucketEntityData::from_compound_nbt(&nbt)?,
                    )
                }
                "minecraft:bundle_conents" => {
                    let items = value
                        .list()
                        .ok_or(SculkParseError::InvalidField(
                            "minecraft:bundle_conents".into(),
                        ))?
                        .compounds()
                        .ok_or(SculkParseError::InvalidField(
                            "minecraft:bundle_conents".into(),
                        ))?
                        .into_iter()
                        .map(|nbt| ItemWithNoSlot::from_compound_nbt(&nbt))
                        .collect::<Result<Vec<ItemWithNoSlot>, SculkParseError>>()?;

                    Component::BundleContents(items)
                }
                "minecraft:can_break" => {
                    let nbt = value
                        .compound()
                        .ok_or(SculkParseError::InvalidField("minecraft:can_break".into()))?;
                    Component::CanBreak(can_break::CanBreak::from_compound_nbt(&nbt)?)
                }
                _ => Component::Unknown(value.to_owned()),
            };

            map.insert(key, component);
        }

        Ok(Components(map))
    }
}

/// Represents a component in a block entity.
#[derive(Debug, Clone, PartialEq)]
// #[serde(untagged)]
pub enum Component<'a> {
    /// `minecraft:attribute_modifiers`
    AttributeModifiers(attribute_modifiers::AttributeModifier<'a>),

    /// `minecraft:banner_patterns`
    BannerPatterns(Vec<banner_patterns::BannerPattern<'a>>),

    /// `minecraft:base_color`
    BaseColor(base_color::BaseColor<'a>),

    /// `minecraft:bees`
    Bees(Vec<bees::Bee<'a>>),

    /// `minecraft:block_entity_data`
    BlockEntityData(NoCoordinatesBlockEntity<'a>),

    /// `minecraft:block_state`
    BlockState(block_state::BlockState<'a>),

    /// `minecraft:bucket_entity_data`
    BucketEntityData(bucket_entity_data::BucketEntityData),

    /// `minecraft:bundle_contents`
    BundleContents(Vec<ItemWithNoSlot<'a>>),

    /// `minecraft:can_break`
    CanBreak(can_break::CanBreak<'a>),

    /// `minecraft:can_place_on`
    ///
    /// `CanPlaceOn` uses the same structure as `CanBreak`
    CanPlaceOn(can_break::CanBreak<'a>),

    /// `minecraft:charged_projectiles`
    ChargedProjectiles(Vec<ItemWithNoSlot<'a>>),

    /// `minecraft:container`
    Container(container::Container<'a>),

    /// `minecraft:container_loot`
    ContainerLoot(container_loot::ContainerLoot<'a>),

    /// `minecraft:custom_data`
    CustomData(custom_data::CustomData<'a>),

    /// `minecraft:custom_model_data`
    CustomModelData(i32),

    /// `custom_name`
    CustomName(Cow<'a, Mutf8Str>),

    /// `minecraft:damage`
    Damage(i32),

    /// `minecraft:debug_stick_state`
    DebugStickState(KeyValuePair<'a>),

    /// `minecraft:dyed_color`
    DyedColor(DyedColor),

    /// `minecraft:enchantment_glint_override`
    EnchantmentGlintOverride(bool),

    /// `minecraft:enchantments`
    Enchantments(u8),

    /// `minecraft:entity_data`
    EntityData(u8),

    /// `minecraft:fire_resistant`
    FireResistant(bool),

    /// `minecraft:firework_explosion`
    FireworkExplosion(u8),

    /// `minecraft:fireworks`
    Fireworks(u8),

    /// `minecraft:food`
    Food(u8),

    /// `minecraft:hide_additional_tooltip`
    HideAdditionalTooltip(bool),

    /// `minecraft:hide_tooltip`
    HideTooltip(bool),

    /// `minecraft:instrument`
    Instrument(u8),

    /// `minecraft:intangible_projectile`
    IntangibleProjectile(bool),

    /// `minecraft:item_name`
    ItemName(Cow<'a, Mutf8Str>),

    /// `minecraft:jukebox_playable`
    JukeboxPlayable(u8),

    /// `minecraft:lock`
    Lock(Cow<'a, Mutf8Str>),

    /// `minecraft:lodestone_tracker`
    LodestoneTracker(u8),

    /// `minecraft:lore`
    Lore(u8),

    /// `minecraft:map_color`
    MapColor(i32),

    /// `minecraft:map_decorations`
    MapDecorations(u8),

    /// `minecraft:map_id`
    MapId(i32),

    /// `minecraft:max_damage`
    MaxDamage(i32),

    /// `minecraft:max_stack_size`
    MaxStackSize(i32),

    /// `minecraft:note_block_sound`
    NoteBlockSound(Cow<'a, Mutf8Str>),

    /// `minecraft:ominous_bottle_amplifier`
    OminousBottleAmplifier(i32),

    /// `minecraft:pot_decorations`
    PotDecorations(u8),

    /// `minecraft:potion_contents`
    PotionContents(u8),

    /// `minecraft:profile`
    Profile(u8),

    /// `minecraft:rarity`
    Rarity(Rarity),

    /// `minecraft:recipes`
    Recipes(u8),

    /// `minecraft:repair_cost`
    RepairCost(i32),

    /// `minecraft:stored_enchantments`
    StoredEnchantments(u8),

    /// `minecraft:suspicous_stew_effects`
    SuspiciousStewEffects(u8),

    /// `minecraft:tool`
    Tool(u8),

    /// `minecraft:trim`
    Trim(u8),

    /// `minecraft:unbreakable`
    Unbreakable(u8),

    /// `minecraft:writable_book_content`
    WritableBookContent(u8),

    /// `minecraft:written_book_content`
    WrittenBookContent(u8),

    Unknown(simdnbt::owned::NbtTag),
}
