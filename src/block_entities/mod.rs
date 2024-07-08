/// BlockEntity rules.
///
/// - Fields that use the `Option<T>` type need a `#[serde(skip_serializing_if = "Option::is_none")]` attribute.  
/// - Fields that is a `bool` needs a `#[serde(deserialize_with = "crate::util::i8_to_bool")]` attribute.
/// - Fields that is a `vec` needs a `#[serde(skip_serializing_if = "Vec::is_empty")]` attribute.
///
/// ## Tests
/// Tests are always within the same file as its own block entity.
/// Tests can either be just `test`, or `basic_test`, `empty_test` or `extended_test`.
/// Tests cover the basic functionality of the block entity, and the edge cases.
/// And should always have a assert_eq!(nbt, nbt) at the end.
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

// TODO: Maybe put wall signs & banners in their own tag?
/// Represents unique data specific to a block entity.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(tag = "id")]
pub enum BlockEntityData<'a> {
    #[serde(borrow)]
    #[serde(alias = "minecraft:barrel")]
    Barrel(barrel::Barrel<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:beacon")]
    Beacon(beacon::Beacon<'a>),

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

    //
    // After this there is just a bunch of boilerplate fields for different
    // versions of skulls, banner, signs etc.
    // These could probably be proc macro'd or something.
    //
    #[serde(borrow)]
    #[serde(alias = "minecraft:skeleton_skull")]
    SkeletonSkull(skull::Skull<'a>),
    #[serde(borrow)]
    #[serde(alias = "minecraft:skeleton_wall_skull")]
    SkeletonWallSkull(skull::Skull<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:wither_skeleton_skull")]
    WitherSkeletonSkull(skull::Skull<'a>),
    #[serde(borrow)]
    #[serde(alias = "minecraft:wither_skeleton_wall_skull")]
    WitherSkeletonWallSkull(skull::Skull<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:zombie_head")]
    ZombieHead(skull::Skull<'a>),
    #[serde(borrow)]
    #[serde(alias = "minecraft:zombie_wall_head")]
    ZombieWallHead(skull::Skull<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:player_head")]
    PlayerHead(skull::Skull<'a>),
    #[serde(borrow)]
    #[serde(alias = "minecraft:player_wall_head")]
    PlayerWallHead(skull::Skull<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:creeper_head")]
    CreeperHead(skull::Skull<'a>),
    #[serde(borrow)]
    #[serde(alias = "minecraft:creeper_wall_head")]
    CreeperWallHead(skull::Skull<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:dragon_head")]
    DragonHead(skull::Skull<'a>),
    #[serde(borrow)]
    #[serde(alias = "minecraft:dragon_wall_head")]
    DragonWallHead(skull::Skull<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:piglin_head")]
    PiglinHead(skull::Skull<'a>),
    #[serde(borrow)]
    #[serde(alias = "minecraft:piglin_wall_head")]
    PiglinWallHead(skull::Skull<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:oak_hanging_sign")]
    OakHangingSign(sign::Sign<'a>),
    #[serde(borrow)]
    #[serde(alias = "minecraft:oak_wall_hanging_sign")]
    OakWallHangingSign(sign::Sign<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:spruce_hanging_sign")]
    SpruceHangingSign(sign::Sign<'a>),
    #[serde(borrow)]
    #[serde(alias = "minecraft:spruce_wall_hanging_sign")]
    SpruceWallHangingSign(sign::Sign<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:birch_hanging_sign")]
    BirchHangingSign(sign::Sign<'a>),
    #[serde(borrow)]
    #[serde(alias = "minecraft:birch_wall_hanging_sign")]
    BirchWallHangingSign(sign::Sign<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:jungle_hanging_sign")]
    JungleHangingSign(sign::Sign<'a>),
    #[serde(borrow)]
    #[serde(alias = "minecraft:jungle_wall_hanging_sign")]
    JungleWallHangingSign(sign::Sign<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:acacia_hanging_sign")]
    AcaciaHangingSign(sign::Sign<'a>),
    #[serde(borrow)]
    #[serde(alias = "minecraft:acacia_wall_hanging_sign")]
    AcaciaWallHangingSign(sign::Sign<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:dark_oak_hanging_sign")]
    DarkOakHangingSign(sign::Sign<'a>),
    #[serde(borrow)]
    #[serde(alias = "minecraft:dark_oak_wall_hanging_sign")]
    DarkOakWallHangingSign(sign::Sign<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:mangrove_hanging_sign")]
    MangroveHangingSign(sign::Sign<'a>),
    #[serde(borrow)]
    #[serde(alias = "minecraft:mangrove_wall_hanging_sign")]
    MangroveWallHangingSign(sign::Sign<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:cherry_hanging_sign")]
    CherryHangingSign(sign::Sign<'a>),
    #[serde(borrow)]
    #[serde(alias = "minecraft:cherry_wall_hanging_sign")]
    CherryWallHangingSign(sign::Sign<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:bamboo_hanging_sign")]
    BambooHangingSign(sign::Sign<'a>),
    #[serde(borrow)]
    #[serde(alias = "minecraft:bamboo_wall_hanging_sign")]
    BambooWallHangingSign(sign::Sign<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:crimson_hanging_sign")]
    CrimsonHangingSign(sign::Sign<'a>),
    #[serde(borrow)]
    #[serde(alias = "minecraft:crimson_wall_hanging_sign")]
    CrimsonWallHangingSign(sign::Sign<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:warped_hanging_sign")]
    WarpedHangingSign(sign::Sign<'a>),
    #[serde(borrow)]
    #[serde(alias = "minecraft:warped_wall_hanging_sign")]
    WarpedWallHangingSign(sign::Sign<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:oak_sign")]
    OakSign(sign::Sign<'a>),
    #[serde(borrow)]
    #[serde(alias = "minecraft:oak_wall_sign")]
    OakWallSign(sign::Sign<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:spruce_sign")]
    SpruceSign(sign::Sign<'a>),
    #[serde(borrow)]
    #[serde(alias = "minecraft:spruce_wall_sign")]
    SpruceWallSign(sign::Sign<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:birch_sign")]
    BirchSign(sign::Sign<'a>),
    #[serde(borrow)]
    #[serde(alias = "minecraft:birch_wall_sign")]
    BirchWallSign(sign::Sign<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:jungle_sign")]
    JungleSign(sign::Sign<'a>),
    #[serde(borrow)]
    #[serde(alias = "minecraft:jungle_wall_sign")]
    JungleWallSign(sign::Sign<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:acacia_sign")]
    AcaciaSign(sign::Sign<'a>),
    #[serde(borrow)]
    #[serde(alias = "minecraft:acacia_wall_sign")]
    AcaciaWallSign(sign::Sign<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:dark_oak_sign")]
    DarkOakSign(sign::Sign<'a>),
    #[serde(borrow)]
    #[serde(alias = "minecraft:dark_oak_wall_sign")]
    DarkOakWallSign(sign::Sign<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:mangrove_sign")]
    MangroveSign(sign::Sign<'a>),
    #[serde(borrow)]
    #[serde(alias = "minecraft:mangrove_wall_sign")]
    MangroveWallSign(sign::Sign<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:cherry_sign")]
    CherrySign(sign::Sign<'a>),
    #[serde(borrow)]
    #[serde(alias = "minecraft:cherry_wall_sign")]
    CherryWallSign(sign::Sign<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:bamboo_sign")]
    BambooSign(sign::Sign<'a>),
    #[serde(borrow)]
    #[serde(alias = "minecraft:bamboo_wall_sign")]
    BambooWallSign(sign::Sign<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:crimson_sign")]
    CrimsonSign(sign::Sign<'a>),
    #[serde(borrow)]
    #[serde(alias = "minecraft:crimson_wall_sign")]
    CrimsonWallSign(sign::Sign<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:warped_sign")]
    WarpedSign(sign::Sign<'a>),
    #[serde(borrow)]
    #[serde(alias = "minecraft:warped_wall_sign")]
    WarpedWallSign(sign::Sign<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:white_shulker_box")]
    WhiteShulkerBox(shulker_box::ShulkerBox<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:orange_shulker_box")]
    OrangeShulkerBox(shulker_box::ShulkerBox<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:magenta_shulker_box")]
    MagentaShulkerBox(shulker_box::ShulkerBox<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:light_blue_shulker_box")]
    LightBlueShulkerBox(shulker_box::ShulkerBox<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:yellow_shulker_box")]
    YellowShulkerBox(shulker_box::ShulkerBox<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:lime_shulker_box")]
    LimeShulkerBox(shulker_box::ShulkerBox<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:pink_shulker_box")]
    PinkShulkerBox(shulker_box::ShulkerBox<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:gray_shulker_box")]
    GrayShulkerBox(shulker_box::ShulkerBox<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:light_gray_shulker_box")]
    LightGrayShulkerBox(shulker_box::ShulkerBox<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:cyan_shulker_box")]
    CyanShulkerBox(shulker_box::ShulkerBox<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:purple_shulker_box")]
    PurpleShulkerBox(shulker_box::ShulkerBox<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:blue_shulker_box")]
    BlueShulkerBox(shulker_box::ShulkerBox<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:brown_shulker_box")]
    BrownShulkerBox(shulker_box::ShulkerBox<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:green_shulker_box")]
    GreenShulkerBox(shulker_box::ShulkerBox<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:red_shulker_box")]
    RedShulkerBox(shulker_box::ShulkerBox<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:black_shulker_box")]
    BlackShulkerBox(shulker_box::ShulkerBox<'a>),

    #[serde(alias = "minecraft:white_bed")]
    WhiteBed,

    #[serde(alias = "minecraft:orange_bed")]
    OrangeBed,

    #[serde(alias = "minecraft:magenta_bed")]
    MagentaBed,

    #[serde(alias = "minecraft:light_blue_bed")]
    LightBlueBed,

    #[serde(alias = "minecraft:yellow_bed")]
    YellowBed,

    #[serde(alias = "minecraft:lime_bed")]
    LimeBed,

    #[serde(alias = "minecraft:pink_bed")]
    PinkBed,

    #[serde(alias = "minecraft:gray_bed")]
    GrayBed,

    #[serde(alias = "minecraft:light_gray_bed")]
    LightGrayBed,

    #[serde(alias = "minecraft:cyan_bed")]
    CyanBed,

    #[serde(alias = "minecraft:purple_bed")]
    PurpleBed,

    #[serde(alias = "minecraft:blue_bed")]
    BlueBed,

    #[serde(alias = "minecraft:brown_bed")]
    BrownBed,

    #[serde(alias = "minecraft:green_bed")]
    GreenBed,

    #[serde(alias = "minecraft:red_bed")]
    RedBed,

    #[serde(alias = "minecraft:black_bed")]
    BlackBed,

    #[serde(borrow)]
    #[serde(alias = "minecraft:white_banner")]
    WhiteBanner(banners::Banner<'a>),
    #[serde(borrow)]
    #[serde(alias = "minecraft:white_wall_banner")]
    WhiteWallBanner(banners::Banner<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:orange_banner")]
    OrangeBanner(banners::Banner<'a>),
    #[serde(borrow)]
    #[serde(alias = "minecraft:orange_wall_banner")]
    OrangeWallBanner(banners::Banner<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:magenta_banner")]
    MagentaBanner(banners::Banner<'a>),
    #[serde(borrow)]
    #[serde(alias = "minecraft:magenta_wall_banner")]
    MagentaWallBanner(banners::Banner<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:light_blue_banner")]
    LightBlueBanner(banners::Banner<'a>),
    #[serde(borrow)]
    #[serde(alias = "minecraft:light_blue_wall_banner")]
    LightBlueWallBanner(banners::Banner<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:yellow_banner")]
    YellowBanner(banners::Banner<'a>),
    #[serde(borrow)]
    #[serde(alias = "minecraft:yellow_wall_banner")]
    YellowWallBanner(banners::Banner<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:lime_banner")]
    LimeBanner(banners::Banner<'a>),
    #[serde(borrow)]
    #[serde(alias = "minecraft:lime_wall_banner")]
    LimeWallBanner(banners::Banner<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:pink_banner")]
    PinkBanner(banners::Banner<'a>),
    #[serde(borrow)]
    #[serde(alias = "minecraft:pink_wall_banner")]
    PinkWallBanner(banners::Banner<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:gray_banner")]
    GrayBanner(banners::Banner<'a>),
    #[serde(borrow)]
    #[serde(alias = "minecraft:gray_wall_banner")]
    GrayWallBanner(banners::Banner<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:light_gray_banner")]
    LightGrayBanner(banners::Banner<'a>),
    #[serde(borrow)]
    #[serde(alias = "minecraft:light_gray_wall_banner")]
    LightGrayWallBanner(banners::Banner<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:cyan_banner")]
    CyanBanner(banners::Banner<'a>),
    #[serde(borrow)]
    #[serde(alias = "minecraft:cyan_wall_banner")]
    CyanWallBanner(banners::Banner<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:purple_banner")]
    PurpleBanner(banners::Banner<'a>),
    #[serde(borrow)]
    #[serde(alias = "minecraft:purple_wall_banner")]
    PurpleWallBanner(banners::Banner<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:blue_banner")]
    BlueBanner(banners::Banner<'a>),
    #[serde(borrow)]
    #[serde(alias = "minecraft:blue_wall_banner")]
    BlueWallBanner(banners::Banner<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:brown_banner")]
    BrownBanner(banners::Banner<'a>),
    #[serde(borrow)]
    #[serde(alias = "minecraft:brown_wall_banner")]
    BrownWallBanner(banners::Banner<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:green_banner")]
    GreenBanner(banners::Banner<'a>),
    #[serde(borrow)]
    #[serde(alias = "minecraft:green_wall_banner")]
    GreenWallBanner(banners::Banner<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:red_banner")]
    RedBanner(banners::Banner<'a>),
    #[serde(borrow)]
    #[serde(alias = "minecraft:red_wall_banner")]
    RedWallBanner(banners::Banner<'a>),

    #[serde(borrow)]
    #[serde(alias = "minecraft:black_banner")]
    BlackBanner(banners::Banner<'a>),
    #[serde(borrow)]
    #[serde(alias = "minecraft:black_wall_banner")]
    BlackWallBanner(banners::Banner<'a>),
}
