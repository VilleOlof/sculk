use based_mca::Region;
use sculk::chunk::Chunk;
use std::fs::File;

fn main() {
    // Read in the region file
    let mut file = File::open("r.0.0.mca").unwrap();
    // Parse the region file
    let region = Region::from_reader(&mut file).unwrap();

    // Iterate over every chunk in the region
    for x in 0..32 {
        for z in 0..32 {
            // Get the chunk at the specified chunk coordinates
            let raw_chunk = region.get_chunk(x, z).unwrap().unwrap();

            // Get the raw chunk data
            let data = match raw_chunk.uncompress() {
                Ok(data) => data,
                Err(e) => {
                    println!("Error: {:?}", e);
                    continue;
                }
            };

            // parse the raw chunk data into a Chunk struct
            let chunk = Chunk::from_bytes(&data).unwrap();
        }
    }
}
