use eyre::{ContextCompat, Result};
use priority_queue::PriorityQueue;
use utils::read_lines_from_input_file;

fn main() -> Result<()> {
    let lines = read_lines_from_input_file()?;

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

    let top_elf = top_elfs.pop().context("No first elf found")?.0;
    let second_elf = top_elfs.pop().context("No second elf found")?.0;
    let third_elf = top_elfs.pop().context("No third elf found")?.0;
    println!(
        "Elf {}, has the most calories: {}",
        top_elf.number, top_elf.calories
    );
    println!(
        "Elf {}, has the second most calories: {}",
        second_elf.number, second_elf.calories
    );
    println!(
        "Elf {}, has the third most calories: {}",
        third_elf.number, third_elf.calories
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
