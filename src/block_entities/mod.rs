use crate::{error::SculkParseError, traits::FromCompoundNbt, util::get_owned_string};
use jukebox::Jukebox;

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
pub enum BlockEntityKind {
    /// `minecraft:<color>_banner` or `minecraft:<color>_wall_banner`
    Banners(banners::Banner),

    /// `minecraft:barrel`
    Barrel(barrel::Barrel),

    /// `minecraft:beacon`
    Beacon(beacon::Beacon),

    /// `minecraft:<color>_bed`
    Bed,

    /// minecraft:beehive
    Beehive(beehive::Beehive),

    /// `minecraft:bell`
    Bell,

    /// `minecraft:blast_furnace`
    BlastFurnace(furnace::Furnace),

    /// `minecraft:brewing_stand`
    BrewingStand(brewing_stand::BrewingStand),

    /// `minecraft:calibrated_sculk_sensor`
    CalibratedSculkSensor(calibrated_sculk_sensor::CalibratedSculkSensor),

    /// `minecraft:campfire`
    Campfire(campfire::Campfire),

    /// `minecraft:chiseled_bookshelf`
    ChiseledBookshelf(chiseled_bookshelf::ChiseledBookshelf),

    /// `minecraft:chest`
    Chest(chest::Chest),

    /// `minecraft:comparator`
    Comparator(comparator::Comparator),

    /// `minecraft:command_block`, `minecraft:chain_command_block`, `minecraft:repeating_command_block`
    CommandBlock(command_block::CommandBlock),

    /// `minecraft:conduit`
    Conduit(conduit::Conduit),

    /// `minecraft:crafter`
    Crafter(crafter::Crafter),

    /// `minecraft:daylight_detector`
    DaylightDetector,

    /// `minecraft:decorated_pot`
    DecoratedPot(decorated_pot::DecoratedPot),

    /// `minecraft:dispenser`
    Dispenser(dispenser::Dispenser),

    /// `minecraft:dropper`
    Dropper(dropper::Dropper),

    /// `minecraft:enchanting_table`
    EnchantingTable(enchanting_table::EnchantingTable),

    /// `minecraft:ender_chest`
    EnderChest,

    /// `minecraft:end_gateway`
    EndGateway(end_gateway::EndGateway),

    /// `minecraft:end_portal`
    EndPortal,

    /// `minecraft:furnace`
    Furnace(furnace::Furnace),

    /// `minecraft:<wood>_hanging_sign` or `minecraft:<wood>_wall_hanging_sign`
    HangingSign(sign::Sign),

    /// `minecraft:hopper`
    Hopper(hopper::Hopper),

    /// `minecraft:jigsaw`
    Jigsaw(jigsaw::Jigsaw),

    /// `minecraft:jukebox`
    Jukebox(jukebox::Jukebox),

    /// `minecraft:lectern`
    Lectern(lectern::Lectern),

    /// `minecraft:mob_spawner`
    MobSpawner(mob_spawner::MobSpawner),

    /// `minecraft:piston`
    Piston(piston::Piston),

    /// `minecraft:sculk_catalyst`
    SculkCatalyst(sculk_catalyst::SculkCatalyst),

    /// `minecraft:sculk_sensor`
    SculkSensor(sculk_sensor::SculkSensor),

    /// `minecraft:sculk_shrieker`
    SculkShrieker(sculk_shrieker::SculkShrieker),

    /// `minecraft:<color>_shulker_box`
    ShulkerBox(shulker_box::ShulkerBox),

    /// `minecraft:<wood>_sign` or `minecraft:<wood>_wall_sign`
    Sign(sign::Sign),

    /// `minecraft:skull`, `minecraft:skeleton_skull`, `minecraft:wither_skeleton_skull`, `minecraft:zombie_head`, `minecraft:player_head`, `minecraft:creeper_head`, `minecraft:dragon_head`, `minecraft:piglin_head`  
    /// And their wall variants.
    Skull(skull::Skull),

    /// `minecraft:structure_block`
    StructureBlock(structure_block::StructureBlock),

    /// `minecraft:smoker`
    Smoker(furnace::Furnace),

    /// `minecraft:soul_campfire`
    SoulCampfire(campfire::Campfire),

    /// `minecraft:suspicious_gravel`
    SuspiciousGravel(suspicious_block::SuspiciousBlock),

    /// `minecraft:suspicious_sand`
    SuspiciousSand(suspicious_block::SuspiciousBlock),

    /// `minecraft:trapped_chest`
    TrappedChest(chest::Chest),

    /// `minecraft:trail_spawner`
    TrialSpawner(trail_spawner::TrailSpawner),

    /// `minecraft:vault`
    Vault(vault::Vault),
}

impl FromCompoundNbt for BlockEntityKind {
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

        let id = get_owned_string(&nbt, "id").map_err(|_| {
            SculkParseError::MissingField(
                "BlockEntityKind requires a parent block entity tag, no / invalid id found".into(),
            )
        })?;

        let kind = match id.as_str() {
            "minecraft:banner"
            | "minecraft:white_banner"
            | "minecraft:orange_banner"
            | "minecraft:magenta_banner"
            | "minecraft:light_blue_banner"
            | "minecraft:yellow_banner"
            | "minecraft:lime_banner"
            | "minecraft:pink_banner"
            | "minecraft:gray_banner"
            | "minecraft:light_gray_banner"
            | "minecraft:cyan_banner"
            | "minecraft:purple_banner"
            | "minecraft:blue_banner"
            | "minecraft:brown_banner"
            | "minecraft:green_banner"
            | "minecraft:red_banner"
            | "minecraft:black_banner"
            | "minecraft:white_wall_banner"
            | "minecraft:orange_wall_banner"
            | "minecraft:magenta_wall_banner"
            | "minecraft:light_blue_wall_banner"
            | "minecraft:yellow_wall_banner"
            | "minecraft:lime_wall_banner"
            | "minecraft:pink_wall_banner"
            | "minecraft:gray_wall_banner"
            | "minecraft:light_gray_wall_banner"
            | "minecraft:cyan_wall_banner"
            | "minecraft:purple_wall_banner"
            | "minecraft:blue_wall_banner"
            | "minecraft:brown_wall_banner"
            | "minecraft:green_wall_banner"
            | "minecraft:red_wall_banner"
            | "minecraft:black_wall_banner" => {
                BlockEntityKind::Banners(Banner::from_compound_nbt(&nbt)?)
            }
            "minecraft:barrel" => BlockEntityKind::Barrel(Barrel::from_compound_nbt(&nbt)?),
            "minecraft:beacon" => BlockEntityKind::Beacon(Beacon::from_compound_nbt(&nbt)?),
            "minecraft:bed"
            | "minecraft:white_bed"
            | "minecraft:orange_bed"
            | "minecraft:magenta_bed"
            | "minecraft:light_blue_bed"
            | "minecraft:yellow_bed"
            | "minecraft:lime_bed"
            | "minecraft:pink_bed"
            | "minecraft:gray_bed"
            | "minecraft:light_gray_bed"
            | "minecraft:cyan_bed"
            | "minecraft:purple_bed"
            | "minecraft:blue_bed"
            | "minecraft:brown_bed"
            | "minecraft:green_bed"
            | "minecraft:red_bed"
            | "minecraft:black_bed" => BlockEntityKind::Bed,
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
            "minecraft:oak_hanging_sign"
            | "minecraft:spruce_hanging_sign"
            | "minecraft:birch_hanging_sign"
            | "minecraft:jungle_hanging_sign"
            | "minecraft:acacia_hanging_sign"
            | "minecraft:dark_oak_hanging_sign"
            | "minecraft:mangrove_hanging_sign"
            | "minecraft:cherry_hanging_sign"
            | "minecraft:bamboo_hanging_sign"
            | "minecraft:crimson_hanging_sign"
            | "minecraft:warped_hanging_sign"
            | "minecraft:oak_wall_hanging_sign"
            | "minecraft:spruce_wall_hanging_sign"
            | "minecraft:birch_wall_hanging_sign"
            | "minecraft:jungle_wall_hanging_sign"
            | "minecraft:acacia_wall_hanging_sign"
            | "minecraft:dark_oak_wall_hanging_sign"
            | "minecraft:mangrove_wall_hanging_sign"
            | "minecraft:cherry_wall_hanging_sign"
            | "minecraft:bamboo_wall_hanging_sign"
            | "minecraft:crimson_wall_hanging_sign"
            | "minecraft:warped_wall_hanging_sign" => {
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
            "minecraft:shulker_box"
            | "minecraft:white_shulker_box"
            | "minecraft:orange_shulker_box"
            | "minecraft:magenta_shulker_box"
            | "minecraft:light_blue_shulker_box"
            | "minecraft:yellow_shulker_box"
            | "minecraft:lime_shulker_box"
            | "minecraft:pink_shulker_box"
            | "minecraft:gray_shulker_box"
            | "minecraft:light_gray_shulker_box"
            | "minecraft:cyan_shulker_box"
            | "minecraft:purple_shulker_box"
            | "minecraft:blue_shulker_box"
            | "minecraft:brown_shulker_box"
            | "minecraft:green_shulker_box"
            | "minecraft:red_shulker_box"
            | "minecraft:black_shulker_box" => {
                BlockEntityKind::ShulkerBox(ShulkerBox::from_compound_nbt(&nbt)?)
            }
            "minecraft:sign"
            | "minecraft:oak_sign"
            | "minecraft:spruce_sign"
            | "minecraft:birch_sign"
            | "minecraft:jungle_sign"
            | "minecraft:acacia_sign"
            | "minecraft:dark_oak_sign"
            | "minecraft:mangrove_sign"
            | "minecraft:cherry_sign"
            | "minecraft:bamboo_sign"
            | "minecraft:crimson_sign"
            | "minecraft:warped_sign"
            | "minecraft:oak_wall_sign"
            | "minecraft:spruce_wall_sign"
            | "minecraft:birch_wall_sign"
            | "minecraft:jungle_wall_sign"
            | "minecraft:acacia_wall_sign"
            | "minecraft:dark_oak_wall_sign"
            | "minecraft:mangrove_wall_sign"
            | "minecraft:cherry_wall_sign"
            | "minecraft:bamboo_wall_sign"
            | "minecraft:crimson_wall_sign"
            | "minecraft:warped_wall_sign" => BlockEntityKind::Sign(Sign::from_compound_nbt(&nbt)?),
            // SKULL
            "minecraft:skull"
            | "minecraft:skeleton_skull"
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
