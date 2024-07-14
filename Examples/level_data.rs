use flate2::read::GzDecoder;
use sculk::level::Level;
use simdnbt::borrow;
use std::io::{Cursor, Read};

fn main() {
    // Read in the level.dat file
    let mut file = std::fs::File::open("level.dat").unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    let mut src = &contents[..];

    /// Uncompress the level.dat file since it is gzipped (probably)
    let mut src_decoder = GzDecoder::new(&mut src);
    let mut input = Vec::new();
    if src_decoder.read_to_end(&mut input).is_err() {
        input = contents;
    }
    let mut input_stream = Cursor::new(&input[..]);

    /// Convert it to an NBT compound
    let nbt = borrow::read(&mut input_stream).unwrap().unwrap();
    let nbt = nbt.as_compound();

    /// Parse the NBT compound into a Level struct
    let level = Level::from_compound_nbt(&nbt).unwrap();
}
