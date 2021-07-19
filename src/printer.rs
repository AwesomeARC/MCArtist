use std::{io::Write, path::Path};
use crate::commands::CommandList;
use std::error::Error;
use std::fs::File;

pub fn print_to_console(command_list: &CommandList, in_aternos_format: bool) {

    for command in command_list {
        println!("{}",
            if in_aternos_format { command.to_aternos_string() }
            else { command.to_string() }
        );
    }

}

pub fn print_to_file(command_list: &CommandList, file_path: &Path, in_aternos_format: bool)
    -> Result<(), Box<dyn Error>> {

    let mut file = File::create(file_path)?;

    for command in command_list {
        file.write_all(
            format!("{}\n",
                if in_aternos_format { command.to_aternos_string() }
                else { command.to_string() }
            ).as_bytes()
        )?;
    }

    println!(
        "Commands successfully written to {}",
        file_path.to_str().unwrap()
    ); 

    Ok(())

}
