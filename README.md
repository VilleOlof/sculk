# Sculk

> [!IMPORTANT]  
> Supported Minecraft version: `1.21`

A Rust crate for handling data in Minecraft.  
Using the fastest NBT parser in Rust, [`simdnbt`](https://crates.io/crates/simdnbt) so you can get the best performance.

Deserialize block entities, entire chunks, item components, and more with ease.  
And get fully typed data structures for all of them.  

## Performance

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

## TODO

- [X] Simdnbt Migration  
    [X] Add a bare bones deserialize that only does the base block entity.  
        Which can be useful to essentially lazy load components + block entity data  

- [X] Add test inside every block entity .rs file to test all their data  
- [X] Own Block Entity for every banner type, shulker, sign, hanging sign, skull, wall   versions etc. so proper variant > id conversion can be done.  
- [X] Fix broken components  
- [X] Add all components  
- [X] Add Chunk module for chunk data  
- [X] Add Player inventory / ender chest  
- [X] Add level.dat  
- [ ] Player statistics?    
- [ ] Player advancements?    
- [ ] Command storage, idcounts, map data, raids?    
- [ ] Once done, rethink/look back on pub api  
- [/] Criterion benchmarks  
- [/] Fork mca-parser and make it barebones to uncompressed data only (based-mca)  