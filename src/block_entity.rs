use serde::{Deserialize, Serialize};

use crate::{
    block_entities::{variant::BlockEntityVariant, BlockEntityData},
    ComponentMap,
};

/// Refer to `BlockEntity` for documentation.
///
/// This lacks the `id` field in a normal block entity.
/// use `.variant()` and its `kind` field to determine the type of block entity.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct BlockEntityBase<'a> {
    /// If true, this is an invalid block entity, and this block is not immediately placed when a loaded chunk is loaded. If false, this is a normal block entity that can be immediately placed.
    #[serde(rename = "keepPacked")]
    #[serde(deserialize_with = "crate::util::i8_to_bool")]
    pub keep_packed: bool,

    /// X coordinate of the block entity.
    pub x: i32,

    /// Y coordinate of the block entity.
    pub y: i32,

    /// Z coordinate of the block entity.
    pub z: i32,

    /// Optional map of components.
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    // TODO: This should be a HashMap here and a custom deserializer.
    pub components: Option<ComponentMap<'a>>,
}

/// Represents a block entity.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct BlockEntity<'a> {
    #[serde(flatten)]
    pub base: BlockEntityBase<'a>,

    #[serde(borrow)]
    #[serde(flatten)]
    pub kind: BlockEntityData<'a>,
}

impl<'a> BlockEntity<'a> {
    pub fn variant(&self) -> BlockEntityVariant {
        self.kind.variant()
    }
}

#[cfg(test)]
#[test]
fn full_block_entity_test() {
    use fastnbt::nbt;

    let nbt = nbt!({
        "id": "minecraft:chest",
        "keepPacked": 0i8,
        "x": 1,
        "y": 2,
        "z": 3,
        "CustomName": "Some name",
        "LootTable": "minecraft:chests/simple_dungeon",
        "Items": [
            {
                "Slot": 0u8,
                "id": "minecraft:dirt",
                "Count": 48
            }
        ]
    });

    let block_entity: BlockEntity = fastnbt::from_value(&nbt).unwrap();

    assert_eq!(block_entity.base.keep_packed, false);
    assert_eq!(block_entity.base.x, 1);
    assert_eq!(block_entity.base.y, 2);
    assert_eq!(block_entity.base.z, 3);

    let chest = if let BlockEntityData::Chest(chest) = &block_entity.kind {
        chest
    } else {
        panic!("Expected a chest block entity");
    };

    assert_eq!(chest.custom_name.as_ref().unwrap(), "Some name");
    assert_eq!(
        chest.loot_table.as_ref().unwrap(),
        "minecraft:chests/simple_dungeon"
    );
    assert_eq!(chest.items.len(), 1);

    let item = chest.items.get(0).unwrap();

    assert_eq!(item.id, "minecraft:dirt");
    assert_eq!(item.count, 48);
    assert_eq!(item.slot, 0);

    assert_eq!(block_entity.variant(), BlockEntityVariant::Chest);

    let serialized_nbt = fastnbt::to_value(&block_entity).unwrap();

    // assert_eq!(nbt, serialized_nbt);
}
