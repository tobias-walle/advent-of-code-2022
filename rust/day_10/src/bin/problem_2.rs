use std::str::FromStr;

use eyre::{bail, Context, ContextCompat, Result};
use utils::{read_lines_from_input_file, Lines};

fn main() -> Result<()> {
    let lines = read_lines_from_input_file().context("Cannot read input lines")?;

    let result = solve_problem(lines).context("Failed to solve problem")?;
    println!("{result}");
    Ok(())
}

fn solve_problem(lines: Lines) -> Result<String> {
    let mut cycle = 0;
    let mut x = 1;
    let mut result = String::new();

    for line in lines {
        let line = line.context("Cannot parse line")?;

        let instruction: Instruction = line.parse()?;
        match instruction {
            Instruction::AddX(n) => {
                cycle += 1;
                draw_pixel(&mut result, cycle, x);
                cycle += 1;
                draw_pixel(&mut result, cycle, x);
                x += n;
            }
            Instruction::Noop => {
                cycle += 1;
                draw_pixel(&mut result, cycle, x);
            }
        };
    }

    Ok(result.trim_end().to_string())
}

fn draw_pixel(result: &mut String, cycle: i32, x: i32) {
    let column = cycle % 40;
    if column >= x && column <= x + 2 {
        result.push('#');
    } else {
        result.push('.');
    }
    if column == 0 {
        result.push('\n');
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

        pretty_assertions::assert_eq!(
            result,
            "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......###.
#######.......#######.......#######.....
            "
            .trim()
        );
    }
}
