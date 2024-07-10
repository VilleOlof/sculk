# Sculk

> [!IMPORTANT]  
> Supported Minecraft version: `1.21`

A Rust crate for handling data in Minecraft.  
This crate is mainly focused on *reading* data  
and deserialize it into Rust data structures.  

But it can also serialize back into NBT format but there may be issues and or bloated NBT data.  

## TODO

- [X] Add test inside every block entity .rs file to test all their data
- [X] Own Block Entity for every banner type, shulker, sign, hanging sign, skull, wall versions etc. so proper variant > id conversion can be done.
- [ ] Fix broken components
- [ ] Add all block entity & component default values if specified on wiki.
- [ ] Add all components
- [ ] Add tests for all components
- [ ] Once done, rethink/look back on pub api
- [ ] Criterion benchmarks