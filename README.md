# Sculk

> [!IMPORTANT]  
> Supported Minecraft version: `1.21`

A Rust crate for handling data in Minecraft.  
Using the fastest NBT parser in Rust, [`simdnbt`](https://crates.io/crates/simdnbt) so you can get the best performance.

Deserialize block entities, entire chunks, item components, and more with ease.  
And get fully typed data structures for all of them.  

## TODO

- [X] Simdnbt Migration  
    [X] Add a bare bones deserialize that only does the base block entity.  
        Which can be useful to essentially lazy load components + block entity data  

- [X] Add test inside every block entity .rs file to test all their data  
- [X] Own Block Entity for every banner type, shulker, sign, hanging sign, skull, wall   versions etc. so proper variant > id conversion can be done.  
- [X] Fix broken components  
- [ ] Add all components  
- [ ] Add Chunk module for chunk data
- [ ] Once done, rethink/look back on pub api  
- [ ] Criterion benchmarks  
- [ ] Fork mca-parser and make it barebones to uncompressed data only (based-mca)  