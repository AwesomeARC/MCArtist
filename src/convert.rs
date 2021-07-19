use std::error::Error;
use json::JsonValue;
use image::{DynamicImage, GenericImageView};

use crate::commands::{Coords, SetBlock, CommandList};

struct MatchingBlock { id: u8, dev: f32 }

pub fn img_to_mc(img: &DynamicImage, block_list: &JsonValue, translucent: bool)
    -> Result< Vec<Vec<u8>>, Box<dyn Error> > {

    let (width, height) = img.dimensions();

    let width = width as usize;
    let height = height as usize;

    let mut mc_block_matrix: Vec<Vec<u8>> = vec![vec![0; width]; height];

    // Calculates deviation of rgb2 from rgb1
    let deviation = | rgb1: (u8, u8, u8), rgb2: (u8, u8, u8) |
    ((
        (rgb2.0 as f32 - rgb1.0 as f32).powf(2.0) +
        (rgb2.1 as f32 - rgb1.1 as f32).powf(2.0) +
        (rgb2.2 as f32 - rgb1.2 as f32).powf(2.0)
    )).sqrt();

    for (i, pixel) in img.to_rgba8().pixels().enumerate() {

        let px_red = pixel.0[0] as u8;
        let px_green = pixel.0[1] as u8;
        let px_blue = pixel.0[2] as u8;
        let px_alpha = pixel.0[3] as u8;

        // Considering single digit alpha
        // to be equivalent to transparency
        if px_alpha < 10 { continue; }

        let mut closest_match: Option<MatchingBlock> = None;

        for block in block_list.members() {

            let id = block["id"].as_u8().unwrap();

            // Skip testing with air
            if id == 0 { continue }

            // Keep #000000ff (pure black) as
            // fallback in case something goes wrong
            let blk_red = block["r"].as_u8().unwrap_or(0);
            let blk_green = block["g"].as_u8().unwrap_or(0);
            let blk_blue = block["b"].as_u8().unwrap_or(0);
            let blk_alpha = block["a"].as_u8().unwrap_or(255);

            if !translucent && blk_alpha < 255 { continue; }

            let new_dev = deviation(
                (px_red, px_green, px_blue),
                (blk_red, blk_green, blk_blue)
            );

            match closest_match {

                // For the very first block (initialization)
                None => {

                    closest_match = Some(MatchingBlock {
                        id,
                        dev: new_dev
                    });

                },

                // If a block with less deviation is found
                Some(MatchingBlock{ dev, .. })
                if new_dev < dev => {

                    closest_match = Some(MatchingBlock {
                        id,
                        dev: new_dev
                    });

                },

                // For all other cases
                _ => { continue; }

            }

        }

        if let Some(MatchingBlock{ id, .. }) = closest_match {
            mc_block_matrix[i / width][i % width] = id;
        }

    }

    Ok(mc_block_matrix)

}

pub fn blocks_to_commands(
    mc_block_matrix: Vec<Vec<u8>>,
    coords: Coords,
    block_list: JsonValue) -> CommandList {

    let mut command_list: CommandList = vec![];

    for (i, matrix_row) in mc_block_matrix.iter().enumerate() {

        for (j, block_id) in matrix_row.iter().enumerate() {

            let block_index = *block_id as usize;

            // Ignore air
            if block_index == 0 { continue }

            let new_coords: Coords = (
                coords.0 + j as i32,
                coords.1,
                coords.2 + i as i32
            );

            command_list.push(SetBlock::new(new_coords,
                &block_list[block_index]["name"]
                    .to_string()[..]
            ));

        }

    }

    command_list

}
