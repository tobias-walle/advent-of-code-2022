use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
};

use eyre::{ContextCompat, Result};

pub fn read_lines_from_input_file() -> Result<io::Lines<io::BufReader<File>>> {
    let input_file_name = env::args()
        .nth(1)
        .context("Please specify the input file name as the first argument")?;
    println!("Read {input_file_name}");
    let file = File::open(input_file_name)?;
    Ok(BufReader::new(file).lines())
}
