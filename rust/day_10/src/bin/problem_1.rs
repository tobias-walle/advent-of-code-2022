use std::str::FromStr;

use eyre::{bail, Context, ContextCompat, Result};
use utils::{read_lines_from_input_file, Lines};

fn main() -> Result<()> {
    let lines = read_lines_from_input_file().context("Cannot read input lines")?;

    let result = solve_problem(lines).context("Failed to solve problem")?;
    println!("{result}");
    Ok(())
}

fn solve_problem(lines: Lines) -> Result<i32> {
    // Parse
    let mut cycle = 0;
    let mut x = 1;
    let mut result = 0;

    for line in lines {
        let line = line.context("Cannot parse line")?;

        // Parse instruction of line
        let instruction: Instruction = line.parse()?;
        match instruction {
            Instruction::AddX(n) => {
                next_cycle(&mut cycle, x, &mut result);
                next_cycle(&mut cycle, x, &mut result);
                x += n;
            }
            Instruction::Noop => {
                next_cycle(&mut cycle, x, &mut result);
            }
        };
    }

    Ok(result)
}

fn next_cycle(cycle: &mut i32, x: i32, result: &mut i32) {
    *cycle += 1;
    if *cycle == 20 || (*cycle + 20) % 40 == 0 {
        *result += *cycle * x;
    }
}

#[derive(Debug)]
enum Instruction {
    Noop,
    AddX(i32),
}

impl FromStr for Instruction {
    type Err = eyre::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = string.split(' ').collect();
        let result = match parts.first() {
            Some(&"noop") => Self::Noop,
            Some(&"addx") => Self::AddX(
                parts
                    .get(1)
                    .context("Expect second argument of addx")?
                    .parse()
                    .context("Expect second argument of addx to be number")?,
            ),
            _ => bail!("Cannot parse intruction: {string}"),
        };
        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use utils::read_lines;

    use super::*;

    #[test]
    fn test_example_problem() {
        let lines = read_lines("example.txt").unwrap();

        let result = solve_problem(lines).unwrap();

        assert_eq!(result, 13140)
    }
}
