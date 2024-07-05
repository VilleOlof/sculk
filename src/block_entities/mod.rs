use std::collections::HashMap;

use fastnbt::Value;
use serde::{Deserialize, Serialize};

pub mod banners;
pub mod barrel;
pub mod beacon;
pub mod end_gateway;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum BlockEntityData<'a> {
    #[serde(borrow)]
    Banners(banners::Banners<'a>),

    #[serde(borrow)]
    Barrel(barrel::Barrel<'a>),

    #[serde(borrow)]
    Beacon(beacon::Beacon<'a>),

    EndGateway(end_gateway::EndGateway),
    Other(Value),
}

/// Converts a `BlockEntityData` enum to a `HashMap<String, Value>`.
macro_rules! to_value_map {
    ($value:expr) => {
        match fastnbt::to_value($value) {
            Ok(Value::Compound(map)) => map,
            _ => HashMap::new(),
        }
    };
}

impl<'a> BlockEntityData<'a> {
    pub fn new(id: &str, data: HashMap<String, Value>) -> Self {
        match id {
            "minecraft:end_gateway" => {
                let map = Value::Compound(data);

                let end_gateway: end_gateway::EndGateway = match fastnbt::from_value(&map) {
                    Ok(end_gateway) => end_gateway,
                    Err(_) => return BlockEntityData::Other(map),
                };

                BlockEntityData::EndGateway(end_gateway)
            }
            _ => BlockEntityData::Other(Value::Compound(data)),
        }
    }

    pub fn as_hashmap(&'a self) -> HashMap<String, Value> {
        match self {
            BlockEntityData::Banners(banners) => to_value_map!(banners),
            BlockEntityData::Barrel(barrel) => to_value_map!(barrel),
            BlockEntityData::Beacon(beacon) => to_value_map!(beacon),
            BlockEntityData::EndGateway(end_gateway) => to_value_map!(end_gateway),
            BlockEntityData::Other(value) => match value {
                Value::Compound(map) => map.clone(),
                _ => HashMap::new(),
            },
        }
    }
}
