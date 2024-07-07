use fastnbt::Value;

use crate::BlockEntity;

/// Deserialize a block entity from a `fastnbt::Value`.
pub fn deserialize_from_value<'de, 'a>(
    value: &'a Value,
) -> Result<BlockEntity<'a>, fastnbt::error::Error>
where
    'de: 'a,
{
    Ok(fastnbt::from_value::<BlockEntity>(value)?)
}

/// Deserialize a block entity from a byte slice.
pub fn deserialize_from_bytes<'de, 'a>(
    bytes: &'a [u8],
) -> Result<BlockEntity<'a>, fastnbt::error::Error>
where
    'de: 'a,
{
    Ok(fastnbt::from_bytes::<BlockEntity>(bytes)?)
}

/// Serialize a block entity to a `fastnbt::Value`.
pub fn serialize_to_value<'de, 'a>(
    block_entity: &'a BlockEntity,
) -> Result<Value, fastnbt::error::Error>
where
    'de: 'a,
{
    fastnbt::to_value(&block_entity)
}

/// Serialize a block entity to a byte vector.
pub fn serialize_to_bytes<'de, 'a>(
    block_entity: &'a BlockEntity,
) -> Result<Vec<u8>, fastnbt::error::Error>
where
    'de: 'a,
{
    fastnbt::to_bytes(&block_entity)
}
