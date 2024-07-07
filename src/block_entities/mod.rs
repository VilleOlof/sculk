use std::borrow::Cow;

/// BlockEntity rules.
///
/// - Fields that use the `Option<T>` type need a `#[serde(skip_serializing_if = "Option::is_none")]` attribute.  
/// - Fields that is a bool needs a `#[serde(deserialize_with = "crate::util::i8_to_bool")]` attribute.
use serde::{Deserialize, Serialize};

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
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(tag = "id")]
pub enum BlockEntityData<'a> {
    #[serde(borrow)]
    #[serde(
        alias = "minecraft:white_banner",
        alias = "minecraft:orange_banner",
        alias = "minecraft:magenta_banner",
        alias = "minecraft:light_blue_banner",
        alias = "minecraft:yellow_banner",
        alias = "minecraft:lime_banner",
        alias = "minecraft:pink_banner",
        alias = "minecraft:gray_banner",
        alias = "minecraft:light_gray_banner",
        alias = "minecraft:cyan_banner",
        alias = "minecraft:purple_banner",
        alias = "minecraft:blue_banner",
        alias = "minecraft:brown_banner",
        alias = "minecraft:green_banner",
        alias = "minecraft:red_banner",
        alias = "minecraft:black_banner",
        alias = "minecraft:white_wall_banner",
        alias = "minecraft:orange_wall_banner",
        alias = "minecraft:magenta_wall_banner",
        alias = "minecraft:light_blue_wall_banner",
        alias = "minecraft:yellow_wall_banner",
        alias = "minecraft:lime_wall_banner",
        alias = "minecraft:pink_wall_banner",
        alias = "minecraft:gray_wall_banner",
        alias = "minecraft:light_gray_wall_banner",
        alias = "minecraft:cyan_wall_banner",
        alias = "minecraft:purle_wall_banner",
        alias = "minecraft_blue_wall_banner",
        alias = "minecraft:brown_wall_banner",
        alias = "minecraft_green_wall_banner",
        alias = "minecraft:red_wall_banner",
        alias = "minecraft:black_wall_banner"
    )]
    Banners(banners::Banners<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:barrel")]
    Barrel(barrel::Barrel<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:beacon")]
    Beacon(beacon::Beacon<'a>),

    #[serde(
        alias = "minecraft:white_bed",
        alias = "minecraft:orange_bed",
        alias = "minecraft:magenta_bed",
        alias = "minecraft:light_blue_bed",
        alias = "minecraft:yellow_bed",
        alias = "minecraft:lime_bed",
        alias = "minecraft:pink_bed",
        alias = "minecraft:gray_bed",
        alias = "minecraft:light_gray_bed",
        alias = "minecraft:cyan_bed",
        alias = "minecraft:purple_bed",
        alias = "minecraft:blue_bed",
        alias = "minecraft:brown_bed",
        alias = "minecraft:green_bed",
        alias = "minecraft:red_bed",
        alias = "minecraft:black_bed"
    )]
    Bed,

    #[serde(borrow)]
    #[serde(alias = "minecraft:beehive")]
    Beehive(beehive::Beehive<'a>),

    #[serde(alias = "minecraft:bell")]
    Bell,

    #[serde(borrow)]
    #[serde(alias = "minecraft:blast_furnace")]
    BlastFurnace(furnace::Furnace<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:brewing_stand")]
    BrewingStand(brewing_stand::BrewingStand<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:calibrated_sculk_sensor")]
    CalibratedSculkSensor(calibrated_sculk_sensor::CalibratedSculkSensor<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:campfire")]
    Campfire(campfire::Campfire<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:chiseled_bookshelf")]
    ChiseledBookshelf(chiseled_bookshelf::ChiseledBookshelf<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:chest")]
    Chest(chest::Chest<'a>),

    #[serde(alias = "minecraft:comparator")]
    Comparator(comparator::Comparator),

    #[serde(borrow)]
    #[serde(
        alias = "minecraft:command_block",
        alias = "minecraft:chain_command_block",
        alias = "minecraft:repeating_command_block"
    )]
    CommandBlock(command_block::CommandBlock<'a>),

    #[serde(alias = "minecraft:conduit")]
    Conduit(conduit::Conduit),

    #[serde(borrow)]
    #[serde(alias = "minecraft:crafter")]
    Crafter(crafter::Crafter<'a>),

    #[serde(alias = "minecraft:daylight_detector")]
    DaylightDetector,

    #[serde(borrow)]
    #[serde(alias = "minecraft:decorated_pot")]
    DecoratedPot(decorated_pot::DecoratedPot<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:dispenser")]
    Dispenser(dispenser::Dispenser<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:dropper")]
    Dropper(dropper::Dropper<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:enchanting_table")]
    EnchantingTable(enchanting_table::EnchantingTable<'a>),

    #[serde(alias = "minecraft:ender_chest")]
    EnderChest,

    #[serde(alias = "minecraft:end_gateway")]
    EndGateway(end_gateway::EndGateway),

    #[serde(alias = "minecraft:end_portal")]
    EndPortal,

    #[serde(borrow)]
    #[serde(alias = "minecraft:furnace")]
    Furnace(furnace::Furnace<'a>),

    #[serde(borrow)]
    #[serde(
        alias = "minecraft:oak_hanging_sign",
        alias = "minecraft:spruce_hanging_sign",
        alias = "minecraft:birch_hanging_sign",
        alias = "minecraft:jungle_hanging_sign",
        alias = "minecraft:acacia_hanging_sign",
        alias = "minecraft:dark_oak_hanging_sign",
        alias = "minecraft:mangrove_hanging_sign",
        alias = "minecraft:cherry_hanging_sign",
        alias = "minecraft:bamboo_hanging_sign",
        alias = "minecraft:crimson_hanging_sign",
        alias = "minecraft:warped_hanging_sign",
        alias = "minecraft:oak_wall_hanging_sign",
        alias = "minecraft:spruce_wall_hanging_sign",
        alias = "minecraft:birch_wall_hanging_sign",
        alias = "minecraft:jungle_wall_hanging_sign",
        alias = "minecraft:acacia_wall_hanging_sign",
        alias = "minecraft:dark_oak_wall_hanging_sign",
        alias = "minecraft:mangrove_wall_hanging_sign",
        alias = "minecraft:cherry_wall_hanging_sign",
        alias = "minecraft:bamboo_wall_hanging_sign",
        alias = "minecraft:crimson_wall_hanging_sign",
        alias = "minecraft:warped_wall_hanging_sign"
    )]
    HangingSing(sign::Sign<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:hopper")]
    Hopper(hopper::Hopper<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:jigsaw")]
    Jigsaw(jigsaw::Jigsaw<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:jukebox")]
    Jukebox(jukebox::Jukebox<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:lectern")]
    Lectern(lectern::Lectern<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:mob_spawner")]
    MobSpawner(mob_spawner::MobSpawner<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:piston")]
    Piston(piston::Piston<'a>),

    #[serde(alias = "minecraft:sculk_catalyst")]
    SculkCatalyst(sculk_catalyst::SculkCatalyst),

    #[serde(borrow)]
    #[serde(alias = "minecraft:sculk_sensor")]
    SculkSensor(sculk_sensor::SculkSensor<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:sculk_shrieker")]
    SculkShrieker(sculk_shrieker::SculkShrieker<'a>),

    #[serde(borrow)]
    #[serde(
        alias = "minecraft:shulker_box",
        alias = "minecraft:white_shulker_box",
        alias = "minecraft:orange_shulker_box",
        alias = "minecract:magenta_shulker_box",
        alias = "minecraft:light_blue_shulker_box",
        alias = "minecraft:yellow_shulker_box",
        alias = "minecraft:lime_shulker_box",
        alias = "minecraft:pink_shulker_box",
        alias = "minecraft:gray_shulker_box",
        alias = "minecraft:light_gray_shulker_box",
        alias = "minecraft:cyan_shulker_box",
        alias = "minecraft:purple_shulker_box",
        alias = "minecraft:blue_shulker_box",
        alias = "minecraft:brown_shulker_box",
        alias = "minecraft:green_shulker_box",
        alias = "minecraft:red_shulker_box",
        alias = "minecraft:black_shulker_box"
    )]
    ShulkerBox(shulker_box::ShulkerBox<'a>),

    #[serde(borrow)]
    #[serde(
        alias = "minecraft:oak_sign",
        alias = "minecraft:spruce_sign",
        alias = "minecraft:birch_sign",
        alias = "minecraft:jungle_sign",
        alias = "minecraft:acacia_sign",
        alias = "minecraft:dark_oak_sign",
        alias = "minecraft:mangrove_sign",
        alias = "minecraft:cherry_sign",
        alias = "minecraft:bamboo_sign",
        alias = "minecraft:crimson_sign",
        alias = "minecraft:warped_sign",
        alias = "minecraft:oak_wall_sign",
        alias = "minecraft:spruce_wall_sign",
        alias = "minecraft:birch_wall_sign",
        alias = "minecraft:jungle_wall_sign",
        alias = "minecraft:acacia_wall_sign",
        alias = "minecraft:dark_oak_wall_sign",
        alias = "minecraft:mangrove_wall_sign",
        alias = "minecraft:cherry_wall_sign",
        alias = "minecraft:bamboo_wall_sign",
        alias = "minecraft:crimson_wall_sign",
        alias = "minecraft:warped_wall_sign"
    )]
    Sign(sign::Sign<'a>),

    #[serde(borrow)]
    #[serde(
        alias = "minecraft:skeleton_skull",
        alias = "minecraft:wither_skeleton_skull",
        alias = "minecraft:zombie_head",
        alias = "minecraft:player_head",
        alias = "minecraft:creeper_head",
        alias = "minecraft:dragon_head",
        alias = "minecraft:piglin_head",
        alias = "minecraft:skeleton_wall_skull",
        alias = "minecraft:wither_skeleton_wall_skull",
        alias = "minecraft:zombie_wall_head",
        alias = "minecraft:player_wall_head",
        alias = "minecraft:creeper_wall_head",
        alias = "minecraft:dragon_wall_head",
        alias = "minecraft:piglin_wall_head"
    )]
    Skull(skull::Skull<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:structure_block")]
    StructureBlock(structure_block::StructureBlock<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:smoker")]
    Smoker(furnace::Furnace<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:soul_campfire")]
    SoulCampfire(campfire::Campfire<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:suspicious_gravel")]
    SuspiciousGravel(suspicious_block::SuspiciousBlock<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:suspicious_sand")]
    SuspiciousSand(suspicious_block::SuspiciousBlock<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:trapped_chest")]
    TrappedChest(chest::Chest<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:trail_spawner")]
    TrialSpawner(trail_spawner::TrailSpawner<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:vault")]
    Vault(vault::Vault<'a>),
}
