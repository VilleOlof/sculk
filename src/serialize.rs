pub mod block_entity_serialize {
    use crate::BlockEntity;
    use fastnbt::Value;
    use std::borrow::Cow;

    /// Deserialize a block entity from a `fastnbt::Value`.
    pub fn deserialize_from_value<'de, 'a>(
        value: &'a Value,
    ) -> Result<BlockEntity<'a>, fastnbt::error::Error>
    where
        'de: 'a,
    {
        let mut block_entity = fastnbt::from_value::<BlockEntity>(value)?;

        block_entity.base.id = Cow::Borrowed(block_entity.kind.to_id());

        Ok(block_entity)
    }

    /// Deserialize a block entity from a byte slice.
    pub fn deserialize_from_bytes<'de, 'a>(
        bytes: &'a [u8],
    ) -> Result<BlockEntity<'a>, fastnbt::error::Error>
    where
        'de: 'a,
    {
        let mut block_entity = fastnbt::from_bytes::<BlockEntity>(bytes)?;

        block_entity.base.id = Cow::Borrowed(block_entity.kind.to_id());

        Ok(block_entity)
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
}
