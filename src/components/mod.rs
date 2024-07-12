use std::{borrow::Cow, collections::HashMap, ops::Deref};

use attribute_modifiers::AttributeModifier;
use banner_patterns::BannerPattern;
use base_color::BaseColor;
use bees::Bee;
use simdnbt::Mutf8Str;

use crate::{
    block_entities::skull, block_entity::NoCoordinatesBlockEntity, color::RGB, entity::Entity,
    error::SculkParseError, item::ItemWithNoSlot, kv::KVPair, rarity::Rarity,
    traits::FromCompoundNbt, util::get_t_list,
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
pub mod enchantments;
pub mod firework_explosion;
pub mod fireworks;
pub mod food;
pub mod instrument;
pub mod jukebox_playable;
pub mod lodestone_tracker;
pub mod map_decorations;
pub mod potion_contents;
pub mod suspicious_stew_effects;
pub mod tool;
pub mod trim;
pub mod unbreakable;
pub mod writable_book_content;
pub mod written_book_content;

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
                    let list = value.list().ok_or(SculkParseError::InvalidField(
                        "minecraft:banner_patterns".into(),
                    ))?;
                    let patterns = get_t_list(
                        &list,
                        "minecraft:banner_patterns",
                        BannerPattern::from_compound_nbt,
                    )?;

                    Component::BannerPatterns(patterns)
                }
                "minecraft:base_color" => Component::BaseColor(BaseColor::from_compound_nbt(&nbt)?),
                "minecraft:bees" => {
                    let list = value
                        .list()
                        .ok_or(SculkParseError::InvalidField("minecraft:bees".into()))?;
                    let bees = get_t_list::<Bee>(&list, "minecraft:bees", Bee::from_compound_nbt)?;

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
                    let list = value.list().ok_or(SculkParseError::InvalidField(
                        "minecraft:bundle_conents".into(),
                    ))?;
                    let items = get_t_list(
                        &list,
                        "minecraft_bundle_conents",
                        ItemWithNoSlot::from_compound_nbt,
                    )?;

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
    /// Can be defined as a compound or a list. If defined as a list, corresponds to modifiers.  
    /// `minecraft:attribute_modifiers`
    AttributeModifiers(attribute_modifiers::AttributeModifier<'a>),

    /// List of all patterns applied to the banner or the shield.  
    /// `minecraft:banner_patterns`
    BannerPatterns(Vec<banner_patterns::BannerPattern<'a>>),

    /// The base dye color of the banner applied on a shield.  
    /// `minecraft:base_color`
    BaseColor(base_color::BaseColor<'a>),

    /// The entities currently in the beehive or bee nest.  
    /// `minecraft:bees`
    Bees(Vec<bees::Bee<'a>>),

    /// [Block entity](https://minecraft.wiki/w/Block_entity) NBT applied when this block is placed.
    /// `minecraft:block_entity_data`
    BlockEntityData(NoCoordinatesBlockEntity<'a>),

    /// The block state properties to apply when placing this block.  
    /// `minecraft:block_state`
    BlockState(block_state::BlockState<'a>),

    /// NBT applied to an [entity](https://minecraft.wiki/w/Entity) when placed from this bucket. Only tags below are applied.
    /// `minecraft:bucket_entity_data`
    BucketEntityData(bucket_entity_data::BucketEntityData),

    /// The items stored inside this [bundle](https://minecraft.wiki/w/Bundle).  
    /// `minecraft:bundle_contents`
    BundleContents(Vec<ItemWithNoSlot<'a>>),

    /// The only blocks this item may break when used by a player in [adventure](https://minecraft.wiki/w/Adventure) mode.
    /// `minecraft:can_break`
    CanBreak(can_break::CanBreak<'a>),

    /// Determines which blocks that blocks with this component can be placed against in [adventure](https://minecraft.wiki/w/Adventure) mode.  
    /// `minecraft:can_place_on`
    ///
    /// `CanPlaceOn` uses the same structure as `CanBreak`
    CanPlaceOn(can_break::CanBreak<'a>),

    /// The items loaded as projectiles into this crossbow. If not present, this crossbow is not charged.  
    /// `minecraft:charged_projectiles`
    ChargedProjectiles(Vec<ItemWithNoSlot<'a>>),

    /// The items contained in this [container](https://minecraft.wiki/w/Container).  
    /// `minecraft:container`
    Container(container::Container<'a>),

    /// The unresolved loot table and seed of this container item.  
    /// `minecraft:container_loot`
    ContainerLoot(container_loot::ContainerLoot<'a>),

    /// Contains key-value pairs of any custom data not used by the game, either as an object or a [SNBT](https://minecraft.wiki/w/SNBT) string.  
    /// `minecraft:custom_data`
    CustomData(custom_data::CustomData<'a>),

    /// A value used in the "custom_model_data" [item tag](https://minecraft.wiki/w/Model#Item_models) in the overrides of item models.  
    /// `minecraft:custom_model_data`
    CustomModelData(i32),

    /// The JSON text component to use as this item's name. See [Raw JSON text](https://minecraft.wiki/w/Raw_JSON_text_format) format.  
    /// `custom_name`
    CustomName(Cow<'a, Mutf8Str>),

    /// The number of uses consumed (not remaining) of the item's durability. Must be a non-negative integer, defaults to 0.  
    /// `minecraft:damage`
    Damage(i32),

    /// The selected block state properties used by this debug stick.  
    /// `minecraft:debug_stick_state`
    DebugStickState(KVPair<'a, Cow<'a, Mutf8Str>>),

    /// Can be defined as a compound or an integer. If defined as an integer, corresponds to  rgb.  
    /// `minecraft:dyed_color`
    DyedColor(dyed_color::DyedColor),

    ///  Overrides the enchantment glint effect on this item. When true, this item will display a glint, even without enchantments. When false, this item will not display a glint, even with enchantments.  
    /// `minecraft:enchantment_glint_override`
    EnchantmentGlintOverride(bool),

    /// Can contain either the following fields, or key-value pairs of levels of [enchantments](https://minecraft.wiki/w/Enchantment)  
    /// `minecraft:enchantments`
    Enchantments(enchantments::Enchantments<'a>),

    /// NBT applied to an [entity](https://minecraft.wiki/w/Entity) when created from an item.   
    /// `minecraft:entity_data`
    EntityData(Entity<'a>),

    /// If set, this item will not burn in fire or lava.  
    /// `minecraft:fire_resistant`
    FireResistant(bool),

    /// The explosion effect stored by this [firework star](https://minecraft.wiki/w/Firework_star).  
    /// `minecraft:firework_explosion`
    FireworkExplosion(firework_explosion::FireworkExplosion),

    /// `minecraft:fireworks`
    Fireworks(fireworks::Fireworks),

    /// If set, this item is considered as a food, and can be eaten.  
    /// `minecraft:food`
    Food(food::Food<'a>),

    /// If set, it will hide additional info on this item's tooltip.  
    /// `minecraft:hide_additional_tooltip`
    HideAdditionalTooltip(bool),

    /// If set, it will completely hide this item's tooltip, including its name.  
    /// `minecraft:hide_tooltip`
    HideTooltip(bool),

    /// [instrument](https://minecraft.wiki/w/Goat_horn) (referenced by ID or inlined)  
    /// `minecraft:instrument`
    Instrument(instrument::Instrument<'a>),

    /// If set, this projectile item can't be picked up by a player when fired, except in creative mode. Can only be used within [charged_projectiles](https://minecraft.wiki/w/Data_component_format#charged_projectiles) components.  
    /// `minecraft:intangible_projectile`
    IntangibleProjectile(bool),

    /// The default name of this item, as a JSON text component. See [Raw JSON text format](https://minecraft.wiki/w/Raw_JSON_text_format). Unlike the [custom_name](https://minecraft.wiki/w/Data_component_format#custom_name) component, this name cannot be changed through an anvil, and does not show in some labels, such as banner markers and item frames.  
    /// `minecraft:item_name`
    ItemName(Cow<'a, Mutf8Str>),

    ///  If present, this item can be inserted into a [jukebox](https://minecraft.wiki/w/Jukebox) and plays the specified song.  
    /// `minecraft:jukebox_playable`
    JukeboxPlayable(jukebox_playable::JukeboxPlayable<'a>),

    ///  The string value representing the "key" to open this container item. The key must be an item with the same value as its custom name.  
    /// `minecraft:lock`
    Lock(Cow<'a, Mutf8Str>),

    /// If specified, stores information about the lodestone this compass should point towards.  
    /// `minecraft:lodestone_tracker`
    LodestoneTracker(lodestone_tracker::LodestoneTracker<'a>),

    /// List of additional lines to display in this item's tooltip. Has a maximum of 256 lines.   
    /// `minecraft:lore`
    Lore(Vec<Cow<'a, Mutf8Str>>),

    /// The color of the markings on this [filled map](https://minecraft.wiki/w/Filled_map) item texture.  
    /// `minecraft:map_color`
    MapColor(RGB),

    ///  Contains key-value pairs of the icons to display on this [filled map](https://minecraft.wiki/w/Filled_map).  
    /// `minecraft:map_decorations`
    MapDecorations(map_decorations::MapDecorations<'a>),

    /// The number of this filled map, representing the shared state holding map contents and markers.  
    /// `minecraft:map_id`
    MapId(i32),

    /// The maximum amount of damage that this item can take. If not set, this item cannot take damage. Must be a positive integer. Note that this component cannot be combined with [max_stack_size](https://minecraft.wiki/w/Data_component_format#max_stack_size).  
    /// `minecraft:max_damage`
    MaxDamage(i32),

    /// The maximum number of items that can fit in a stack. Must be a positive integer between 1 and 99 (inclusive). Note that this component cannot be combined with [max_damage](https://minecraft.wiki/w/Data_component_format#max_damage).  
    /// `minecraft:max_stack_size`
    MaxStackSize(i32),

    /// The ID of the sound event played by a note block when this [player head](https://minecraft.wiki/w/Player_head) is placed above.  
    /// `minecraft:note_block_sound`
    NoteBlockSound(Cow<'a, Mutf8Str>),

    /// The amplifier amount of the [Bad Omen](https://minecraft.wiki/w/Bad_Omen) effect on this [ominous bottle](https://minecraft.wiki/w/Ominous_bottle). Must be a positive integer between 0 and 4 (inclusive).  
    /// `minecraft:ominous_bottle_amplifier`
    OminousBottleAmplifier(i32),

    /// A list of the sherds applied on each face of this [decorated pot](https://minecraft.wiki/w/Decorated_pot). If the list has less than 4 entries, the remaining ones default to `minecraft:brick`.   
    ///
    /// Each entry: The ID of an item. Can be either brick or a sherd.  
    ///
    /// `minecraft:pot_decorations`
    PotDecorations(Vec<Cow<'a, Mutf8Str>>),

    /// The potion and custom effects contained in this [potion](https://minecraft.wiki/w/Potion), [splash potion](https://minecraft.wiki/w/Splash_potion), [lingering potion](https://minecraft.wiki/w/Lingering_potion), or [tipped arrow](https://minecraft.wiki/w/Tipped_arrow). If defined as a string, corresponds to  potion.   
    /// `minecraft:potion_contents`
    PotionContents(potion_contents::PotionContents<'a>),

    /// Information about the owner of this player head. If defined as a string, corresponds to name.  
    /// `minecraft:profile`
    Profile(skull::SkullProfile<'a>),

    /// Sets the rarity of this item, which affects the default color of its name.  
    /// `minecraft:rarity`
    Rarity(Rarity),

    /// The recipes that a player unlocks when this knowledge book is used.  
    /// `minecraft:recipes`
    Recipes(Vec<Cow<'a, Mutf8Str>>),

    /// The number of experience levels to add to the base level cost when repairing, combining, or renaming this item with an anvil. Must be a non-negative integer, defaults to 0.  
    /// `minecraft:repair_cost`
    RepairCost(i32),

    ///  Can contain either the following fields, or key-value pairs of levels of enchantments. For the latter, corresponds to  levels.   
    /// `minecraft:stored_enchantments`
    StoredEnchantments(enchantments::Enchantments<'a>),

    /// The effects applied when consuming this suspicious stew.  
    /// `minecraft:suspicous_stew_effects`
    SuspiciousStewEffects(Vec<suspicious_stew_effects::SuspiciousStewEffects<'a>>),

    /// If set, this item is considered as a [tool](https://minecraft.wiki/w/Tool).  
    /// `minecraft:tool`
    Tool(tool::Tool<'a>),

    /// Contains the trim applied to this [armor](https://minecraft.wiki/w/Armor) piece.  
    /// `minecraft:trim`
    Trim(trim::Trim<'a>),

    /// If set, this item doesn't lose durability when used.  
    /// `minecraft:unbreakable`
    Unbreakable(unbreakable::Unbreakable),

    /// The contents of this [book and quill](https://minecraft.wiki/w/Book_and_quill).  
    /// `minecraft:writable_book_content`
    WritableBookContent(writable_book_content::WritableBookContent<'a>),

    /// The contents and metadata of this [written book](https://minecraft.wiki/w/Written_book).  
    /// `minecraft:written_book_content`
    WrittenBookContent(written_book_content::WrittenBookContent<'a>),

    /// Unknown component.
    Unknown(simdnbt::owned::NbtTag),
}
