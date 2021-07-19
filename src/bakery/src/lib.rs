use std::fs::{self, read_dir};
use std::error::Error;
use std::path::Path;
use image::{self, GenericImageView};
use json::{self, JsonValue};

pub fn bake_blocks(textures_dir: &Path, block_db_path: &Path)
    -> Result<(), Box<dyn Error>> {

    let mut texture_entries: Vec<_> = read_dir(textures_dir)?
        .map(|res| res.unwrap().path())
        .collect();

    texture_entries.sort();


    let mut block_list = json::parse(r#" [

        {

            "id": 0,
            "texture": null,
            "name": "air",
            "r": 0,
            "g": 0,
            "b": 0

        }

    ] "#)?;

    for (i, texture_entry) in texture_entries.iter().enumerate() {

        let id = i + 1;

        let texture = String::from(
            texture_entry
            .file_name().unwrap()
            .to_str().unwrap()
        );

        let name = texture.replace(".png", "");

        let img = image::open( texture_entry )?;

        let mut red_sum: u32 = 0;
        let mut green_sum: u32 = 0;
        let mut blue_sum: u32 = 0;
        let mut alpha_sum: u32 = 0;

        for pixel in img.to_rgba8().pixels() {

            red_sum += pixel.0[0] as u32;
            green_sum += pixel.0[1] as u32;
            blue_sum += pixel.0[2] as u32;
            alpha_sum += pixel.0[3] as u32;

        }

        let img_area = img.dimensions().0 * img.dimensions().1;

        let red = red_sum as u32 / img_area;
        let green = green_sum as u32 / img_area;
        let blue = blue_sum as u32 / img_area;
        let alpha = alpha_sum as u32 / img_area;

        let mut block = JsonValue::new_object();

        block.insert("id", JsonValue::from(id))?;
        block.insert("texture", texture)?;
        block.insert("name", name)?;
        block.insert("r", red)?;
        block.insert("g", green)?;
        block.insert("b", blue)?;
        block.insert("a", alpha)?;

        block_list.push(block)?;

    }

    fs::write(block_db_path, json::stringify(block_list))?;

    println!("Successfully written.");

    Ok(())

}

