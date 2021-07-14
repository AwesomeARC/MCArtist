use std::error::Error;
use json::JsonValue;
use image::{DynamicImage, GenericImageView};

struct MatchingBlock { id: u8, dev: f32 }

pub fn img_to_mc(img: &DynamicImage, block_list: &JsonValue)
    -> Result< Vec<Vec<u8>>, Box<dyn Error> > {

    let (width, height) = img.dimensions();

    let width = width as usize;
    let height = height as usize;

    let mut mc_block_matrix: Vec<Vec<u8>> = vec![vec![0; width]; height];

    let deviation = | rgb1: (u8, u8, u8), rgb2: (u8, u8, u8) |
    ((
        (rgb2.0 - rgb1.0).pow(2) +
        (rgb2.1 - rgb1.1).pow(2) +
        (rgb2.2 - rgb1.2).pow(2)
    ) as f32 ).sqrt();

    let mut closest_match: Option<MatchingBlock> = None;

    for (i, pixel) in img.to_rgb8().pixels().enumerate() {

        let px_red = pixel.0[0] as u8;
        let px_green = pixel.0[1] as u8;
        let px_blue = pixel.0[2] as u8;

        for block in block_list.members() {

            let id = block["id"].as_u8().unwrap();

            let blk_red = block["r"].as_u8().unwrap();
            let blk_green = block["g"].as_u8().unwrap();
            let blk_blue = block["b"].as_u8().unwrap();

            let new_dev = deviation(
                (px_red, px_green, px_blue),
                (blk_red, blk_green, blk_blue)
            );


            match closest_match {

                None => {
                    closest_match = Some(MatchingBlock {
                        id,
                        dev: new_dev
                    })
                },

                Some(MatchingBlock{ dev, .. })
                if new_dev < dev => {
                    closest_match = Some(MatchingBlock {
                        id,
                        dev: new_dev
                    })
                },

                _ => { }

            }

        }

        if let Some(MatchingBlock{ id, .. }) = closest_match {
            mc_block_matrix[i / width][i % width] = id;
        }

    }

    Ok(mc_block_matrix)

}
