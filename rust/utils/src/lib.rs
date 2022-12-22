use std::{
    env,
    fs::{read_to_string, File},
    io::{self, BufRead, BufReader},
};

use eyre::{Context, ContextCompat, Result};

pub mod parsing;

pub type Lines = io::Lines<io::BufReader<File>>;

pub fn read_lines_from_input_file() -> Result<Lines> {
    read_lines(&get_input_file_name_from_args()?)
}

pub fn read_lines(input_file_name: &str) -> Result<Lines> {
    println!("Read {input_file_name}");
    let file = File::open(input_file_name)?;
    Ok(BufReader::new(file).lines())
}

pub fn read_input_file_as_string() -> Result<String> {
    read_to_string(&get_input_file_name_from_args()?).context("Couldn't read input file as string")
}

pub fn get_input_file_name_from_args() -> Result<String> {
    let input_file_name = env::args()
        .nth(1)
        .context("Please specify the input file name as the first argument")?;
    Ok(input_file_name)
}
