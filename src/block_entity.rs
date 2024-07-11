use simdnbt::{borrow::BaseNbt, Mutf8Str};

use crate::{
    block_entities::{variant::BlockEntityVariant, BlockEntityKind},
    error::SculkParseError,
    traits::{FromCompoundNbt, FromNbt},
    util::{get_bool, get_optional_components, get_owned_mutf8str},
    Components,
};
use std::{borrow::Cow, io::Cursor};

/// The base fields of a block entity.
#[derive(Debug, Clone, PartialEq)]
pub struct BlockEntityBase<'a> {
    /// ID of block entity.
    pub id: Cow<'a, Mutf8Str>,

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
    pub components: Option<Components<'a>>,
}

/// The base fields of a block entity.  
/// This is used for `lazy` block entities.  
/// So it does not contain the `components` field.  
#[derive(Debug, Clone, PartialEq)]
pub struct LazyBlockEntityBase<'a> {
    /// ID of block entity.
    pub id: Cow<'a, Mutf8Str>,

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
pub struct BlockEntity<'a> {
    /// Common fields of a block entity.
    pub base: BlockEntityBase<'a>,

    /// The specific data of the block entity.
    pub kind: BlockEntityKind<'a>,
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
#[derive(Debug, PartialEq)]
pub struct LazyBlockEntity<'a> {
    /// Common fields of a block entity.
    pub base: BlockEntityBase<'a>,

    /// The bytes that was used to parse the block entity.
    // This is a bit ugly but i found no other way with `borrow::Nbt` or `borrow::BaseNbt` to work
    nbt_bytes: &'a [u8],
}

impl<'a> FromCompoundNbt for BlockEntityBase<'a> {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let id = get_owned_mutf8str(&nbt, "id")?;
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

impl<'a> FromCompoundNbt for LazyBlockEntityBase<'a> {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let id = get_owned_mutf8str(&nbt, "id")?;
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

impl<'a> FromNbt for BlockEntity<'a> {
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

impl<'a> LazyBlockEntity<'a> {
    fn from_nbt(nbt: simdnbt::borrow::Nbt<'a>, bytes: &'a [u8]) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let base_nbt: BaseNbt<'a> = match nbt.is_none() {
            true => return Err(SculkParseError::NoNbt),
            false => nbt.unwrap(),
        };

        let compound_nbt = base_nbt.as_compound();

        Ok(Self {
            base: BlockEntityBase::from_compound_nbt(&compound_nbt)?,
            nbt_bytes: bytes,
        })
    }
}

impl<'a> BlockEntity<'a> {
    pub fn variant(&self) -> BlockEntityVariant {
        self.kind.variant()
    }

    pub fn from_bytes(bytes: &'a [u8]) -> Result<Self, SculkParseError> {
        let mut cursor = Cursor::new(bytes);
        let nbt = match simdnbt::borrow::read(&mut cursor) {
            Ok(nbt) => nbt,
            Err(err) => return Err(SculkParseError::NbtError(err)),
        };

        BlockEntity::from_nbt(nbt)
    }
}

impl<'a> LazyBlockEntity<'a> {
    pub fn kind(&self) -> Result<BlockEntityKind, SculkParseError> {
        let nbt = match simdnbt::borrow::read(&mut Cursor::new(self.nbt_bytes)) {
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

    pub fn get_components(&self) -> Result<Option<Components>, SculkParseError> {
        let nbt = match simdnbt::borrow::read(&mut Cursor::new(self.nbt_bytes)) {
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

    let file = std::fs::File::open("test_data/chest_banner.nbt").unwrap();
    let bytes = file.bytes().map(|b| b.unwrap()).collect::<Vec<u8>>();

    let instant = std::time::Instant::now();
    let _ = LazyBlockEntity::from_bytes(bytes.as_slice()).unwrap();
    println!("LazyBlockEntity: {:?}", instant.elapsed());
}
