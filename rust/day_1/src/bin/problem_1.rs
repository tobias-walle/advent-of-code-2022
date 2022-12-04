use eyre::{ContextCompat, Result};
use priority_queue::PriorityQueue;
use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> Result<()> {
    let lines = read_lines_from_input_files()?;

    let mut top_elfs = PriorityQueue::new();

    let mut current_elf = Elf::new(1);
    for line in lines {
        let line = line?;
        if line.is_empty() {
            top_elfs.push(current_elf.clone(), current_elf.calories);
            current_elf = Elf::new(current_elf.number + 1);
        } else {
            current_elf.calories += line.parse::<u32>()?;
        }
    }

    let top_elf = top_elfs.pop().context("No top elf found")?.0;
    println!(
        "Elf {}, has the most calories: {}",
        top_elf.number, top_elf.calories
    );

    Ok(())
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Elf {
    pub number: u32,
    pub calories: u32,
}

impl Elf {
    fn new(number: u32) -> Elf {
        Elf {
            number,
            calories: 0,
        }
    }
}

fn read_lines_from_input_files() -> Result<io::Lines<io::BufReader<File>>> {
    let input_file_name = env::args()
        .nth(1)
        .context("Please specify the input file name as the first argument")?;
    println!("Read {input_file_name}");
    let file = File::open(input_file_name)?;
    Ok(BufReader::new(file).lines())
}
