use mca::RegionReader;
use sculk::chunk::Chunk;
use std::fs::File;
use std::io::Read;

fn main() {
    // Read in the region file
    let mut data = Vec::new();
    let mut file = File::open("r.0.0.mca").unwrap();
    file.read_to_end(&mut data).unwrap();

    // Initialize the region
    // This mostly just validates the header
    let region = RegionReader::new(&data).unwrap();

    // Iterate over every chunk in the region
    for x in 0..32 {
        for z in 0..32 {
            // Get the chunk at the specified chunk coordinates
            let chunk = region.get_chunk(x, z).unwrap().unwrap();

            // Decompress the chunk data
            // This will most commonly be either ZLib or LZ4 compressed
            let decompressed = chunk.decompress().unwrap();

            // parse the raw chunk data into a Chunk struct
            let chunk = Chunk::from_bytes(&decompressed).unwrap();
        }
    }
}
