use std::ops::Deref;

use simdnbt::borrow::NbtCompound;

use crate::error::SculkParseError;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Uuid(pub [i32; 4]);

impl Uuid {
    #[allow(unused_variables)]
    pub fn from_i128(i: i128) -> Self {
        unimplemented!()
    }

    pub fn to_i128(&self) -> i128 {
        unimplemented!()
    }

    pub fn to_hex_string(&self) -> String {
        unimplemented!()
    }
}

impl From<Vec<i32>> for Uuid {
    fn from(v: Vec<i32>) -> Self {
        let mut arr = [0; 4];
        arr.copy_from_slice(&v);
        Self(arr)
    }
}

impl From<[i32; 4]> for Uuid {
    fn from(arr: [i32; 4]) -> Self {
        Self(arr)
    }
}

impl Uuid {
    pub fn from_nbt_to_vec(nbt: &NbtCompound, key: &'static str) -> Vec<Self> {
        nbt.list(key)
            .map_or_else(
                || Ok::<Vec<Uuid>, SculkParseError>(Vec::<Uuid>::new()),
                |nbt| {
                    let mut entries = vec![];

                    for arr in nbt
                        .int_arrays()
                        .ok_or(SculkParseError::InvalidField(key.into()))?
                    {
                        let uuid = Uuid::from(arr.to_vec());
                        entries.push(uuid);
                    }

                    Ok(entries)
                },
            )
            .unwrap_or(Vec::new())
    }
}

impl Deref for Uuid {
    type Target = [i32; 4];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
