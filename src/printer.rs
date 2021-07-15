use std::{io::Write, path::Path};
use crate::commands::CommandList;
use std::error::Error;
use std::fs::File;

pub fn print_to_console(command_list: &CommandList) {

    for command in command_list {
        println!("{}", command.to_string());
    }

}

pub fn print_to_file(command_list: &CommandList, file_path: &Path)
    -> Result<(), Box<dyn Error>> {

    let mut file = File::create(file_path)?;

    for command in command_list {
        file.write_all(
            format!("{}\n", command.to_string()).as_bytes()
        )?;
    }

    println!(
        "Commands successfully written to {}",
        file_path.to_str().unwrap()
    ); 

    Ok(())

}
