# Sculk

> [!IMPORTANT]  
> Supported Minecraft version: `1.21`


**Not stable to use in production, many things are misspelled, wrong data types and or doesn't work as for  now, i'm writing this as a part of another actual production project and will continue to patch  
and fix this project for that, it does work for 90%+ of cases and easy manual work but unstable outside  of that**

A Rust crate for handling data in Minecraft.  
Using the fastest NBT parser in Rust, [`simdnbt`](https://crates.io/crates/simdnbt) so you can get the best performance.

Deserialize block entities, entire chunks, item components, and more with ease.  
And get fully typed data structures for all of them.  

## Cargo Features

- `stats` Enables the `Statistics` data structure and its deserialization.  
    This enables `serde` as a dependency. Thus why its a feature.  

## Performance rant

Theres currently one big bottleneck and thats my skill issue with lifetimes.  
All strings are `Cow<'a, Mutf8str` which are owned strings, that is first converted from borrowed strings.  
This is mostly a temporary solution until I can figure out how to properly handle lifetimes with those strings.  
And currently when deserializing a chunk, it copies memory for every block entity due to again, my lifetime issues.  
And since so many fields are strings and theres a lot of block entities in chunks,  
it slows it down by 30-100ms on my machine per 32x32 chunk (one region).  

Currently chunks use `BlockEntity` instead of its `LazyBlockEntity`, because getting a lazy block entity  
from a nbt compound tag currently requires copying it has a `Vec<u8>` which is so slow and inefficient.  
But if you were to deserialize a LazyBlockEntity from bytes directly, its about 83% faster with relatively basic data.  
So i want to make it deserialize with LazyBlockEntity, specially since it has a `to_owned` method.  
But again, lifetime issues.  
