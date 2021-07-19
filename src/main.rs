pub mod commands;
pub mod convert;
pub mod image_reader;
pub mod printer;

use std::{error::Error, path::Path};
use std::collections::HashMap;
use itertools::Itertools;
use commands::Coords;
use clap::{App, load_yaml};

fn main() -> Result<(), Box<dyn Error>> {

    let block_list_raw = include_str!("./bakery/baked_blocks.json");

    let yaml = load_yaml!("./cli.yaml");
    let matches = App::from(yaml).get_matches();

    let mut options = HashMap::new();

    for option in ["coords", "width", "height", "input"] {

        if let Some(value) = matches.value_of(option) {
            options.insert(option, value);
        } else {
            eprint!("[mcartist] Error: No argument passed to required option `{}`!", option)
        }

    }

    let block_list = json::parse(block_list_raw)?;

    let image_path = Path::new(options["input"]);

    let width: u32 = options["width"].parse()?;
    let height: u32 = options["height"].parse()?;

    let img = image_reader::read_image(image_path, width, height)?;

    let translucent = matches.is_present("translucent");

    let mc_block_matrix = convert::img_to_mc(&img, &block_list, translucent)?;

    let coords_raw: (&str, &str, &str) =
        options["coords"].split(',').collect_tuple().unwrap();

    let coords: Coords = (
        coords_raw.0.parse()?,
        coords_raw.1.parse()?,
        coords_raw.2.parse()?
    );

    let command_list = convert::blocks_to_commands(
        mc_block_matrix, coords, block_list
    );

    let in_aternos_format = matches.is_present("aternos");

    if let Some(output_file) = matches.value_of("output") {

        printer::print_to_file(
            &command_list,
            Path::new(output_file),
            in_aternos_format
        )?;

    } else {

        printer::print_to_console(
            &command_list,
            in_aternos_format
        );

    }

    Ok(())

}
