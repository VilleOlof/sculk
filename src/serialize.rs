use fastnbt::Value;

use crate::{block_entity::LoseBlockEntity, BlockEntity};

/// Deserialize a block entity from a `fastnbt::Value`.
pub fn deserialize_from_value<'de, 'a>(
    value: &'a Value,
) -> Result<BlockEntity<'a>, fastnbt::error::Error>
where
    'de: 'a,
{
    let block_entity = fastnbt::from_value::<LoseBlockEntity>(value)?;

    Ok(BlockEntity::from(block_entity))
}

/// Deserialize a block entity from a byte slice.
pub fn deserialize_from_bytes<'de, 'a>(
    bytes: &'a [u8],
) -> Result<BlockEntity<'a>, fastnbt::error::Error>
where
    'de: 'a,
{
    let block_entity = fastnbt::from_bytes::<LoseBlockEntity>(bytes)?;

    Ok(BlockEntity::from(block_entity))
}

/// Serialize a block entity to a `fastnbt::Value`.
pub fn serialize_to_value<'de, 'a>(
    block_entity: &'a BlockEntity,
) -> Result<Value, fastnbt::error::Error>
where
    'de: 'a,
{
    let lose_block_entity = LoseBlockEntity::from(block_entity);

    fastnbt::to_value(&lose_block_entity)
}

/// Serialize a block entity to a byte vector.
pub fn serialize_to_bytes<'de, 'a>(
    block_entity: &'a BlockEntity,
) -> Result<Vec<u8>, fastnbt::error::Error>
where
    'de: 'a,
{
    let lose_block_entity = LoseBlockEntity::from(block_entity);

    fastnbt::to_bytes(&lose_block_entity)
}
