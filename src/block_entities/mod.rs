use std::collections::HashMap;

use fastnbt::Value;
use serde::{Deserialize, Serialize};

pub mod banners;
pub mod barrel;
pub mod beacon;
pub mod beehive;
pub mod blast_furnace;
pub mod brewing_stand;
pub mod end_gateway;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum BlockEntityData<'a> {
    #[serde(borrow)]
    Banners(banners::Banners<'a>),

    #[serde(borrow)]
    Barrel(barrel::Barrel<'a>),

    #[serde(borrow)]
    Beacon(beacon::Beacon<'a>),

    Bed,

    Beehive(beehive::Beehive),

    Bell,

    #[serde(borrow)]
    BlastFurnace(blast_furnace::BlastFurnace<'a>),

    #[serde(borrow)]
    BrewingStand(brewing_stand::BrewingStand<'a>),

    EndGateway(end_gateway::EndGateway),
    Other(Option<Value>),
}

/// Converts a `BlockEntityData` enum to a `HashMap<String, Value>`.
macro_rules! to_value_map {
    ($value:expr) => {
        match fastnbt::to_value($value) {
            Ok(Value::Compound(map)) => Some(map),
            _ => None,
        }
    };
}

impl<'a> BlockEntityData<'a> {
    pub fn new(id: &str, data: Option<HashMap<String, Value>>) -> Self {
        match data {
            Some(data) => match id {
                "minecraft:end_gateway" => {
                    let map = Value::Compound(data);

                    let end_gateway: end_gateway::EndGateway = match fastnbt::from_value(&map) {
                        Ok(end_gateway) => end_gateway,
                        Err(_) => return BlockEntityData::Other(Some(map)),
                    };

                    BlockEntityData::EndGateway(end_gateway)
                }
                // TODO: Implement the rest of the block entities.
                _ => BlockEntityData::Other(Some(Value::Compound(data))),
            },
            _ => match id {
                "minecraft:bed" => BlockEntityData::Bed,
                _ => BlockEntityData::Other(None),
            },
        }
    }

    pub fn as_hashmap(&'a self) -> Option<HashMap<String, Value>> {
        match self {
            BlockEntityData::Banners(banners) => to_value_map!(banners),
            BlockEntityData::Barrel(barrel) => to_value_map!(barrel),
            BlockEntityData::Beacon(beacon) => to_value_map!(beacon),
            BlockEntityData::EndGateway(end_gateway) => to_value_map!(end_gateway),
            BlockEntityData::Bed => None,
            BlockEntityData::Beehive(beehive) => to_value_map!(beehive),
            BlockEntityData::Bell => None,
            BlockEntityData::BlastFurnace(blast_furnace) => to_value_map!(blast_furnace),
            BlockEntityData::BrewingStand(brewing_stand) => to_value_map!(brewing_stand),

            BlockEntityData::Other(value) => match value {
                Some(value) => match value {
                    Value::Compound(map) => Some(map.clone()),
                    _ => None,
                },
                _ => None,
            },
        }
    }
}
