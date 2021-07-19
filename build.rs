use std::error::Error;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>>{

    println!("cargo:rerun-if-changed=./src/bakery/assets/textures");
    println!("cargo:rerun-if-changed=./src/bakery/baked_blocks.json");

    let textures_dir = Path::new("./src/bakery/assets/textures");
    let block_db_path = Path::new("./src/bakery/baked_blocks.json");

    bakery::bake_blocks(textures_dir, block_db_path)?;

    Ok(())

}
