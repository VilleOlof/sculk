//! ## BlockEntity rules.
//!
//! - Fields that use the `Option<T>` type need a `#[serde(skip_serializing_if = "Option::is_none")]` attribute.  
//! - Fields that is a `bool` needs a `#[serde(deserialize_with = "crate::util::i8_to_bool")]` attribute.
//! - Fields that is a `vec` needs a `#[serde(skip_serializing_if = "Vec::is_empty")]` attribute.
//!
//! ## Tests
//! Tests are always within the same file as its own block entity.
//! Tests can either be just `test`, or `basic_test`, `empty_test` or `extended_test`.
//! Tests cover the basic functionality of the block entity, and the edge cases.
//! And should always have a `assert_eq!(nbt, serialized_nbt);` at the end.

use jukebox::Jukebox;

use crate::{error::SculkParseError, traits::FromCompoundNbt, util::get_owned_mutf8str};

pub mod variant;

pub mod banners;
pub mod barrel;
pub mod beacon;
pub mod beehive;
pub mod brewing_stand;
pub mod calibrated_sculk_sensor;
pub mod campfire;
pub mod chest;
pub mod chiseled_bookshelf;
pub mod command_block;
pub mod comparator;
pub mod conduit;
pub mod crafter;
pub mod decorated_pot;
pub mod dispenser;
pub mod dropper;
pub mod enchanting_table;
pub mod end_gateway;
pub mod furnace;
pub mod hopper;
pub mod jigsaw;
pub mod jukebox;
pub mod lectern;
pub mod mob_spawner;
pub mod piston;
pub mod sculk_catalyst;
pub mod sculk_sensor;
pub mod sculk_shrieker;
pub mod shulker_box;
pub mod sign;
pub mod skull;
pub mod structure_block;
pub mod suspicious_block;
pub mod trail_spawner;
pub mod vault;

/// Represents unique data specific to a block entity.
#[derive(Debug, Clone, PartialEq)]
pub enum BlockEntityKind<'a> {
    /// `minecraft:<color>_banner` or `minecraft:<color>_wall_banner`
    Banners(banners::Banner<'a>),

    /// `minecraft:barrel`
    Barrel(barrel::Barrel<'a>),

    /// `minecraft:beacon`
    Beacon(beacon::Beacon<'a>),

    /// `minecraft:<color>_bed`
    Bed,

    /// minecraft:beehive
    Beehive(beehive::Beehive<'a>),

    /// `minecraft:bell`
    Bell,

    /// `minecraft:blast_furnace`
    BlastFurnace(furnace::Furnace<'a>),

    /// `minecraft:brewing_stand`
    BrewingStand(brewing_stand::BrewingStand<'a>),

    /// `minecraft:calibrated_sculk_sensor`
    CalibratedSculkSensor(calibrated_sculk_sensor::CalibratedSculkSensor<'a>),

    /// `minecraft:campfire`
    Campfire(campfire::Campfire<'a>),

    /// `minecraft:chiseled_bookshelf`
    ChiseledBookshelf(chiseled_bookshelf::ChiseledBookshelf<'a>),

    /// `minecraft:chest`
    Chest(chest::Chest<'a>),

    /// `minecraft:comparator`
    Comparator(comparator::Comparator),

    /// `minecraft:command_block`, `minecraft:chain_command_block`, `minecraft:repeating_command_block`
    CommandBlock(command_block::CommandBlock<'a>),

    /// `minecraft:conduit`
    Conduit(conduit::Conduit),

    /// `minecraft:crafter`
    Crafter(crafter::Crafter<'a>),

    /// `minecraft:daylight_detector`
    DaylightDetector,

    /// `minecraft:decorated_pot`
    DecoratedPot(decorated_pot::DecoratedPot<'a>),

    /// `minecraft:dispenser`
    Dispenser(dispenser::Dispenser<'a>),

    /// `minecraft:dropper`
    Dropper(dropper::Dropper<'a>),

    /// `minecraft:enchanting_table`
    EnchantingTable(enchanting_table::EnchantingTable<'a>),

    /// `minecraft:ender_chest`
    EnderChest,

    /// `minecraft:end_gateway`
    EndGateway(end_gateway::EndGateway),

    /// `minecraft:end_portal`
    EndPortal,

    /// `minecraft:furnace`
    Furnace(furnace::Furnace<'a>),

    /// `minecraft:<wood>_hanging_sign` or `minecraft:<wood>_wall_hanging_sign`
    HangingSign(sign::Sign<'a>),

    /// `minecraft:hopper`
    Hopper(hopper::Hopper<'a>),

    /// `minecraft:jigsaw`
    Jigsaw(jigsaw::Jigsaw<'a>),

    /// `minecraft:jukebox`
    Jukebox(jukebox::Jukebox<'a>),

    /// `minecraft:lectern`
    Lectern(lectern::Lectern<'a>),

    /// `minecraft:mob_spawner`
    MobSpawner(mob_spawner::MobSpawner<'a>),

    /// `minecraft:piston`
    Piston(piston::Piston<'a>),

    /// `minecraft:sculk_catalyst`
    SculkCatalyst(sculk_catalyst::SculkCatalyst),

    /// `minecraft:sculk_sensor`
    SculkSensor(sculk_sensor::SculkSensor<'a>),

    /// `minecraft:sculk_shrieker`
    SculkShrieker(sculk_shrieker::SculkShrieker<'a>),

    /// `minecraft:<color>_shulker_box`
    ShulkerBox(shulker_box::ShulkerBox<'a>),

    /// `minecraft:<wood>_sign` or `minecraft:<wood>_wall_sign`
    Sign(sign::Sign<'a>),

    /// `minecraft:skull`, `minecraft:skeleton_skull`, `minecraft:wither_skeleton_skull`, `minecraft:zombie_head`, `minecraft:player_head`, `minecraft:creeper_head`, `minecraft:dragon_head`, `minecraft:piglin_head`  
    /// And their wall variants.
    Skull(skull::Skull<'a>),

    /// `minecraft:structure_block`
    StructureBlock(structure_block::StructureBlock<'a>),

    /// `minecraft:smoker`
    Smoker(furnace::Furnace<'a>),

    /// `minecraft:soul_campfire`
    SoulCampfire(campfire::Campfire<'a>),

    /// `minecraft:suspicious_gravel`
    SuspiciousGravel(suspicious_block::SuspiciousBlock<'a>),

    /// `minecraft:suspicious_sand`
    SuspiciousSand(suspicious_block::SuspiciousBlock<'a>),

    /// `minecraft:trapped_chest`
    TrappedChest(chest::Chest<'a>),

    /// `minecraft:trail_spawner`
    TrialSpawner(trail_spawner::TrailSpawner<'a>),

    /// `minecraft:vault`
    Vault(vault::Vault<'a>),
}

impl<'a> FromCompoundNbt for BlockEntityKind<'a> {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        use banners::Banner;
        use barrel::Barrel;
        use beacon::Beacon;
        use beehive::Beehive;
        use brewing_stand::BrewingStand;
        use calibrated_sculk_sensor::CalibratedSculkSensor;
        use campfire::Campfire;
        use chest::Chest;
        use chiseled_bookshelf::ChiseledBookshelf;
        use command_block::CommandBlock;
        use comparator::Comparator;
        use conduit::Conduit;
        use crafter::Crafter;
        use decorated_pot::DecoratedPot;
        use dispenser::Dispenser;
        use dropper::Dropper;
        use enchanting_table::EnchantingTable;
        use end_gateway::EndGateway;
        use furnace::Furnace;
        use hopper::Hopper;
        use jigsaw::Jigsaw;
        use lectern::Lectern;
        use mob_spawner::MobSpawner;
        use piston::Piston;
        use sculk_catalyst::SculkCatalyst;
        use sculk_sensor::SculkSensor;
        use sculk_shrieker::SculkShrieker;
        use shulker_box::ShulkerBox;
        use sign::Sign;
        use skull::Skull;
        use structure_block::StructureBlock;
        use suspicious_block::SuspiciousBlock;
        use trail_spawner::TrailSpawner;
        use vault::Vault;

        let id = get_owned_mutf8str(&nbt, "id").map_err(|_| {
            SculkParseError::MissingField(
                "BlockEntityKind requires a parent block entity tag, no / invalid id found".into(),
            )
        })?;

        let kind = match id.to_str().as_ref() {
            "white_banner"
            | "orange_banner"
            | "magenta_banner"
            | "light_blue_banner"
            | "yellow_banner"
            | "lime_banner"
            | "pink_banner"
            | "gray_banner"
            | "light_gray_banner"
            | "cyan_banner"
            | "purple_banner"
            | "blue_banner"
            | "brown_banner"
            | "green_banner"
            | "red_banner"
            | "black_banner"
            | "white_wall_banner"
            | "orange_wall_banner"
            | "magenta_wall_banner"
            | "light_blue_wall_banner"
            | "yellow_wall_banner"
            | "lime_wall_banner"
            | "pink_wall_banner"
            | "gray_wall_banner"
            | "light_gray_wall_banner"
            | "cyan_wall_banner"
            | "purple_wall_banner"
            | "blue_wall_banner"
            | "brown_wall_banner"
            | "green_wall_banner"
            | "red_wall_banner"
            | "black_wall_banner" => BlockEntityKind::Banners(Banner::from_compound_nbt(&nbt)?),
            "minecraft:barrel" => BlockEntityKind::Barrel(Barrel::from_compound_nbt(&nbt)?),
            "minecraft:beacon" => BlockEntityKind::Beacon(Beacon::from_compound_nbt(&nbt)?),
            "white_bed" | "orange_bed" | "magenta_bed" | "light_blue_bed" | "yellow_bed"
            | "lime_bed" | "pink_bed" | "gray_bed" | "light_gray_bed" | "cyan_bed"
            | "purple_bed" | "blue_bed" | "brown_bed" | "green_bed" | "red_bed" | "black_bed" => {
                BlockEntityKind::Bed
            }
            "minecraft:beehive" => BlockEntityKind::Beehive(Beehive::from_compound_nbt(&nbt)?),
            "minecraft:bell" => BlockEntityKind::Bell,
            "minecraft:blast_furnace" => {
                BlockEntityKind::BlastFurnace(Furnace::from_compound_nbt(&nbt)?)
            }
            "minecraft:brewing_stand" => {
                BlockEntityKind::BrewingStand(BrewingStand::from_compound_nbt(&nbt)?)
            }
            "minecraft:calibrated_sculk_sensor" => BlockEntityKind::CalibratedSculkSensor(
                CalibratedSculkSensor::from_compound_nbt(&nbt)?,
            ),
            "minecraft:campfire" => BlockEntityKind::Campfire(Campfire::from_compound_nbt(&nbt)?),
            "minecraft:chiseled_bookshelf" => {
                BlockEntityKind::ChiseledBookshelf(ChiseledBookshelf::from_compound_nbt(&nbt)?)
            }
            "minecraft:chest" => BlockEntityKind::Chest(Chest::from_compound_nbt(&nbt)?),
            "minecraft:comparator" => {
                BlockEntityKind::Comparator(Comparator::from_compound_nbt(&nbt)?)
            }
            "minecraft:command_block"
            | "minecraft:chain_command_block"
            | "minecraft:repeating_command_block" => {
                BlockEntityKind::CommandBlock(CommandBlock::from_compound_nbt(&nbt)?)
            }
            "minecraft:conduit" => BlockEntityKind::Conduit(Conduit::from_compound_nbt(&nbt)?),
            "minecraft:crafter" => BlockEntityKind::Crafter(Crafter::from_compound_nbt(&nbt)?),
            "minecraft:daylight_detector" => BlockEntityKind::DaylightDetector,
            "minecraft:decorated_pot" => {
                BlockEntityKind::DecoratedPot(DecoratedPot::from_compound_nbt(&nbt)?)
            }
            "minecraft:dispenser" => {
                BlockEntityKind::Dispenser(Dispenser::from_compound_nbt(&nbt)?)
            }
            "minecraft:dropper" => BlockEntityKind::Dropper(Dropper::from_compound_nbt(&nbt)?),
            "minecraft:enchanting_table" => {
                BlockEntityKind::EnchantingTable(EnchantingTable::from_compound_nbt(&nbt)?)
            }
            "minecraft:ender_chest" => BlockEntityKind::EnderChest,
            "minecraft:end_gateway" => {
                BlockEntityKind::EndGateway(EndGateway::from_compound_nbt(&nbt)?)
            }
            "minecraft:end_portal" => BlockEntityKind::EndPortal,
            "minecraft:furnace" => BlockEntityKind::Furnace(Furnace::from_compound_nbt(&nbt)?),
            "oak_hanging_sign"
            | "spruce_hanging_sign"
            | "birch_hanging_sign"
            | "jungle_hanging_sign"
            | "acacia_hanging_sign"
            | "dark_oak_hanging_sign"
            | "mangrove_hanging_sign"
            | "cherry_hanging_sign"
            | "bamboo_hanging_sign"
            | "crimson_hanging_sign"
            | "warped_hanging_sign"
            | "oak_wall_hanging_sign"
            | "spruce_wall_hanging_sign"
            | "birch_wall_hanging_sign"
            | "jungle_wall_hanging_sign"
            | "acacia_wall_hanging_sign"
            | "dark_oak_wall_hanging_sign"
            | "mangrove_wall_hanging_sign"
            | "cherry_wall_hanging_sign"
            | "bamboo_wall_hanging_sign"
            | "crimson_wall_hanging_sign"
            | "warped_wall_hanging_sign" => {
                BlockEntityKind::HangingSign(Sign::from_compound_nbt(&nbt)?)
            }
            "minecraft:hopper" => BlockEntityKind::Hopper(Hopper::from_compound_nbt(&nbt)?),
            "minecraft:jigsaw" => BlockEntityKind::Jigsaw(Jigsaw::from_compound_nbt(&nbt)?),
            "minecraft:jukebox" => BlockEntityKind::Jukebox(Jukebox::from_compound_nbt(&nbt)?),
            "minecraft:lectern" => BlockEntityKind::Lectern(Lectern::from_compound_nbt(&nbt)?),
            "minecraft:mob_spawner" => {
                BlockEntityKind::MobSpawner(MobSpawner::from_compound_nbt(&nbt)?)
            }
            "minecraft:piston" => BlockEntityKind::Piston(Piston::from_compound_nbt(&nbt)?),
            "minecraft:sculk_catalyst" => {
                BlockEntityKind::SculkCatalyst(SculkCatalyst::from_compound_nbt(&nbt)?)
            }
            "minecraft:sculk_sensor" => {
                BlockEntityKind::SculkSensor(SculkSensor::from_compound_nbt(&nbt)?)
            }
            "minecraft:sculk_shrieker" => {
                BlockEntityKind::SculkShrieker(SculkShrieker::from_compound_nbt(&nbt)?)
            }
            "shulker_box"
            | "white_shulker_box"
            | "orange_shulker_box"
            | "magenta_shulker_box"
            | "light_blue_shulker_box"
            | "yellow_shulker_box"
            | "lime_shulker_box"
            | "pink_shulker_box"
            | "gray_shulker_box"
            | "light_gray_shulker_box"
            | "cyan_shulker_box"
            | "purple_shulker_box"
            | "blue_shulker_box"
            | "brown_shulker_box"
            | "green_shulker_box"
            | "red_shulker_box"
            | "black_shulker_box" => {
                BlockEntityKind::ShulkerBox(ShulkerBox::from_compound_nbt(&nbt)?)
            }
            "oak_sign" | "spruce_sign" | "birch_sign" | "jungle_sign" | "acacia_sign"
            | "dark_oak_sign" | "mangrove_sign" | "cherry_sign" | "bamboo_sign"
            | "crimson_sign" | "warped_sign" | "oak_wall_sign" | "spruce_wall_sign"
            | "birch_wall_sign" | "jungle_wall_sign" | "acacia_wall_sign"
            | "dark_oak_wall_sign" | "mangrove_wall_sign" | "cherry_wall_sign"
            | "bamboo_wall_sign" | "crimson_wall_sign" | "warped_wall_sign" => {
                BlockEntityKind::HangingSign(Sign::from_compound_nbt(&nbt)?)
            }
            // SKULL
            "minecraft:skeleton_skull"
            | "minecraft:skeleton_wall_skull"
            | "minecraft:wither_skeleton_skull"
            | "minecraft:wither_skeleton_wall_skull"
            | "minecraft:zombie_head"
            | "minecraft:zombie_wall_head"
            | "minecraft:player_head"
            | "minecraft:player_wall_head"
            | "minecraft:creeper_head"
            | "minecraft:creeper_wall_head"
            | "minecraft:piglin_head"
            | "minecraft:piglin_wall_head"
            | "minecraft:dragon_head"
            | "minecraft:dragon_wall_head" => {
                BlockEntityKind::Skull(Skull::from_compound_nbt(&nbt)?)
            }
            "minecraft:structure_block" => {
                BlockEntityKind::StructureBlock(StructureBlock::from_compound_nbt(&nbt)?)
            }
            "minecraft:smoker" => BlockEntityKind::Smoker(Furnace::from_compound_nbt(&nbt)?),
            "minecraft:soul_campfire" => {
                BlockEntityKind::SoulCampfire(Campfire::from_compound_nbt(&nbt)?)
            }
            "minecraft:suspicious_gravel" => {
                BlockEntityKind::SuspiciousGravel(SuspiciousBlock::from_compound_nbt(&nbt)?)
            }
            "minecraft:suspicious_sand" => {
                BlockEntityKind::SuspiciousSand(SuspiciousBlock::from_compound_nbt(&nbt)?)
            }
            "minecraft:trapped_chest" => {
                BlockEntityKind::TrappedChest(Chest::from_compound_nbt(&nbt)?)
            }
            "minecraft:trail_spawner" => {
                BlockEntityKind::TrialSpawner(TrailSpawner::from_compound_nbt(&nbt)?)
            }
            "minecraft:vault" => BlockEntityKind::Vault(Vault::from_compound_nbt(&nbt)?),
            _ => return Err(SculkParseError::UnsupportedBlockEntity(id.to_string())),
        };

        Ok(kind)
    }
}
