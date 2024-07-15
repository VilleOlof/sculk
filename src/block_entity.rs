use crate::{
    block_entities::{variant::BlockEntityVariant, BlockEntityKind},
    components::Components,
    error::SculkParseError,
    traits::{FromCompoundNbt, FromNbt},
    util::{get_bool, get_optional_components, get_owned_string},
};
use simdnbt::borrow::BaseNbt;
use std::io::Cursor;

/// The base fields of a block entity.
#[derive(Debug, Clone, PartialEq)]
pub struct BlockEntityBase {
    /// ID of block entity.
    pub id: String,

    /// If true, this is an invalid block entity, and this block is not immediately placed when a loaded chunk is loaded. If false, this is a normal block entity that can be immediately placed.
    ///
    /// `keepPacked`
    pub keep_packed: bool,

    /// X coordinate of the block entity.
    pub x: i32,

    /// Y coordinate of the block entity.
    pub y: i32,

    /// Z coordinate of the block entity.
    pub z: i32,

    /// Optional map of components.
    pub components: Option<Components>,
}

/// The base fields of a block entity.  
/// This is used in components that excludes x, y, z coordinates.  
///
/// Gotta love minecraft data structures.  
#[derive(Debug, Clone, PartialEq)]
pub struct NoCoordinatesBlockEntityBase {
    /// ID of block entity.
    pub id: String,

    /// If true, this is an invalid block entity, and this block is not immediately placed when a loaded chunk is loaded. If false, this is a normal block entity that can be immediately placed.
    ///
    /// `keepPacked`
    pub keep_packed: bool,

    /// Optional map of components.
    pub components: Option<Components>,
}

/// The base fields of a block entity.  
/// This is used for `lazy` block entities.  
/// So it does not contain the `components` field.  
#[derive(Debug, Clone, PartialEq)]
pub struct LazyBlockEntityBase {
    /// ID of block entity.
    pub id: String,

    /// If true, this is an invalid block entity, and this block is not immediately placed when a loaded chunk is loaded. If false, this is a normal block entity that can be immediately placed.
    ///
    /// `keepPacked`
    pub keep_packed: bool,

    /// X coordinate of the block entity.
    pub x: i32,

    /// Y coordinate of the block entity.
    pub y: i32,

    /// Z coordinate of the block entity.
    pub z: i32,
}

/// Represents a block entity.
#[derive(Debug, Clone, PartialEq)]
pub struct BlockEntity {
    /// Common fields of a block entity.
    pub base: BlockEntityBase,

    /// The specific data of the block entity.
    pub kind: BlockEntityKind,
}

/// Represents a block entity.  
/// But with no coordinates.  
#[derive(Debug, Clone, PartialEq)]
pub struct NoCoordinatesBlockEntity {
    /// Common fields of a block entity.
    pub base: NoCoordinatesBlockEntityBase,

    /// The specific data of the block entity.
    pub kind: BlockEntityKind,
}

/// Represents a `lazy` byte variant.  
/// When directly going from bytes > lazy block entity, we can borrow.  
/// But when we go through chunk data its trickier, i dont even know if its possible to borrow there.  
/// But for now we can own the bytes, sacrificing some memory.
#[derive(Debug, Clone, PartialEq)]
pub enum LazyByteVariant<'a> {
    /// Borrowed bytes.
    Borrowed(&'a [u8]),
    /// Owned bytes.
    Owned(Vec<u8>),
}

/// Represents a `lazy` block entity.  
/// This will only parse [`BlockEntityKind`] when it is accessed.  
/// You can access the [`BlockEntityKind`] data by calling `.kind()`´  
///
/// This will also not parse the `components` field.  
/// You can access the `components` field by calling `.get_components()`  
///
/// This is useful if you only need to check the id of the block entity.  
/// And do further data processing only if the id matches a specific value.
#[derive(Debug, Clone, PartialEq)]
pub struct LazyBlockEntity<'a> {
    /// Common fields of a block entity.
    pub base: LazyBlockEntityBase,

    /// The bytes that was used to parse the block entity.
    // This is a bit ugly but i found no other way with `borrow::Nbt` or `borrow::BaseNbt` to work
    nbt_bytes: LazyByteVariant<'a>,
}

impl<'a> FromCompoundNbt for LazyBlockEntity<'a> {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let base = LazyBlockEntityBase::from_compound_nbt(&nbt)?;

        let mut buf: Vec<u8> = Vec::new();
        nbt.to_owned().write(&mut buf);

        Ok(Self {
            base,
            nbt_bytes: LazyByteVariant::Owned(buf),
        })
    }
}

impl FromCompoundNbt for BlockEntityBase {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let id = get_owned_string(&nbt, "id")?;
        let keep_packed = get_bool(&nbt, "keepPacked");

        let x = nbt
            .int("x")
            .ok_or(SculkParseError::MissingField("x".into()))?;
        let y = nbt
            .int("y")
            .ok_or(SculkParseError::MissingField("y".into()))?;
        let z = nbt
            .int("z")
            .ok_or(SculkParseError::MissingField("z".into()))?;

        let components = get_optional_components(&nbt)?;

        Ok(Self {
            id,
            keep_packed,
            x,
            y,
            z,
            components,
        })
    }
}

impl FromCompoundNbt for NoCoordinatesBlockEntityBase {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let id = get_owned_string(&nbt, "id")?;
        let keep_packed = get_bool(&nbt, "keepPacked");

        let components = get_optional_components(&nbt)?;

        Ok(Self {
            id,
            keep_packed,
            components,
        })
    }
}

impl FromCompoundNbt for LazyBlockEntityBase {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let id = get_owned_string(&nbt, "id")?;
        let keep_packed = get_bool(&nbt, "keepPacked");

        let x = nbt
            .int("x")
            .ok_or(SculkParseError::MissingField("x".into()))?;
        let y = nbt
            .int("y")
            .ok_or(SculkParseError::MissingField("y".into()))?;
        let z = nbt
            .int("z")
            .ok_or(SculkParseError::MissingField("z".into()))?;

        Ok(Self {
            id,
            keep_packed,
            x,
            y,
            z,
        })
    }
}

impl FromNbt for BlockEntity {
    fn from_nbt(nbt: simdnbt::borrow::Nbt) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let base_nbt = match nbt.is_none() {
            true => return Err(SculkParseError::NoNbt),
            false => nbt.unwrap(),
        };
        let nbt = base_nbt.as_compound();

        let base = BlockEntityBase::from_compound_nbt(&nbt)?;
        let kind = BlockEntityKind::from_compound_nbt(&nbt)?;

        Ok(Self { base, kind })
    }
}

impl FromCompoundNbt for BlockEntity {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let base = BlockEntityBase::from_compound_nbt(&nbt)?;
        let kind = BlockEntityKind::from_compound_nbt(&nbt)?;

        Ok(Self { base, kind })
    }
}

impl FromCompoundNbt for NoCoordinatesBlockEntity {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let base = NoCoordinatesBlockEntityBase::from_compound_nbt(&nbt)?;
        let kind = BlockEntityKind::from_compound_nbt(&nbt)?;

        Ok(Self { base, kind })
    }
}

// It got its own silly implementation :3
impl<'a> LazyBlockEntity<'a> {
    fn from_nbt(nbt: simdnbt::borrow::Nbt, bytes: &'a [u8]) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let base_nbt: BaseNbt = match nbt.is_none() {
            true => return Err(SculkParseError::NoNbt),
            false => nbt.unwrap(),
        };

        let compound_nbt = base_nbt.as_compound();

        Ok(Self {
            base: LazyBlockEntityBase::from_compound_nbt(&compound_nbt)?,
            nbt_bytes: LazyByteVariant::Borrowed(bytes),
        })
    }
}

impl BlockEntity {
    /// Get the variant of the block entity.
    pub fn variant(&self) -> BlockEntityVariant {
        self.kind.variant()
    }

    /// Converts from bytes to a block entity.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, SculkParseError> {
        let mut cursor = Cursor::new(bytes);
        let nbt = match simdnbt::borrow::read(&mut cursor) {
            Ok(nbt) => nbt,
            Err(err) => return Err(SculkParseError::NbtError(err)),
        };

        BlockEntity::from_nbt(nbt)
    }
}

impl<'a> LazyBlockEntity<'a> {
    /// Gets the specific data for the block entity.
    pub fn kind(&self) -> Result<BlockEntityKind, SculkParseError> {
        let bytes = match &self.nbt_bytes {
            LazyByteVariant::Borrowed(bytes) => bytes,
            LazyByteVariant::Owned(bytes) => bytes.as_slice(),
        };

        let nbt = match simdnbt::borrow::read(&mut Cursor::new(bytes)) {
            Ok(nbt) => nbt,
            Err(err) => return Err(SculkParseError::NbtError(err)),
        };
        let base_nbt = match nbt.is_none() {
            true => return Err(SculkParseError::NoNbt),
            false => nbt.unwrap(),
        };
        let compound_nbt = base_nbt.as_compound();

        BlockEntityKind::from_compound_nbt(&compound_nbt)
    }

    /// Get the components of the block entity.
    pub fn get_components(&self) -> Result<Option<Components>, SculkParseError> {
        let bytes = match &self.nbt_bytes {
            LazyByteVariant::Borrowed(bytes) => bytes,
            LazyByteVariant::Owned(bytes) => bytes.as_slice(),
        };

        let nbt = match simdnbt::borrow::read(&mut Cursor::new(bytes)) {
            Ok(nbt) => nbt,
            Err(err) => return Err(SculkParseError::NbtError(err)),
        };
        let base_nbt = match nbt.is_none() {
            true => return Err(SculkParseError::NoNbt),
            false => nbt.unwrap(),
        };
        let compound_nbt = base_nbt.as_compound();

        get_optional_components(&compound_nbt)
    }

    /// Converts a [`LazyBlockEntity`] to an owned [`BlockEntity`].
    pub fn to_owned(&self) -> Result<BlockEntity, SculkParseError> {
        let bytes = match &self.nbt_bytes {
            LazyByteVariant::Borrowed(bytes) => bytes,
            LazyByteVariant::Owned(bytes) => bytes.as_slice(),
        };

        let nbt = match simdnbt::borrow::read(&mut Cursor::new(bytes)) {
            Ok(nbt) => nbt,
            Err(err) => return Err(SculkParseError::NbtError(err)),
        };

        BlockEntity::from_nbt(nbt)
    }

    /// Converts from bytes to a block entity.
    pub fn from_bytes(bytes: &'a [u8]) -> Result<Self, SculkParseError> {
        let mut cursor = Cursor::new(bytes);
        let nbt = match simdnbt::borrow::read(&mut cursor) {
            Ok(nbt) => nbt,
            Err(err) => return Err(SculkParseError::NbtError(err)),
        };

        LazyBlockEntity::from_nbt(nbt, bytes)
    }
}

#[cfg(test)]
#[test]
fn test() {
    use std::io::Read;

    let file = std::fs::File::open("test_data/chest_tool.nbt").unwrap();
    let bytes = file.bytes().map(|b| b.unwrap()).collect::<Vec<u8>>();

    let instant = std::time::Instant::now();
    let _ = BlockEntity::from_bytes(bytes.as_slice()).unwrap();
    println!("LazyBlockEntity: {:?}", instant.elapsed());
}
