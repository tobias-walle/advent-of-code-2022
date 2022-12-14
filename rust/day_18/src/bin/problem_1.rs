use std::collections::HashSet;

use eyre::{Context, Result};
use nom::{
    bytes::complete::tag,
    character::complete::newline,
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};
use utils::{
    parsing::{self, parse_with_nom},
    read_input_file_as_string,
};

fn main() -> Result<()> {
    let input = read_input_file_as_string().context("Cannot read input")?;

    let result = solve_problem(&input).context("Failed to solve problem")?;
    println!("{result}");
    Ok(())
}

fn solve_problem(input: &str) -> Result<usize> {
    let cubes = parse_with_nom(input, parse)?;
    let cubes: HashSet<_> = cubes.into_iter().collect();
    let mut sides = 0;

    for cube in cubes.iter() {
        for neighbour in cube.neighbours() {
            if !cubes.contains(&neighbour) {
                sides += 1;
            }
        }
    }
    Ok(sides)
}

fn parse(input: &str) -> IResult<&str, Vec<Cube>> {
    all_consuming(separated_list1(
        newline,
        map(
            tuple((
                parsing::number,
                preceded(tag(","), parsing::number),
                preceded(tag(","), parsing::number),
            )),
            |(x, y, z)| Cube { x, y, z },
        ),
    ))(input.trim())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Cube {
    fn neighbours(&self) -> impl Iterator<Item = Cube> + '_ {
        let offsets = [
            Cube { x: 1, y: 0, z: 0 },
            Cube { x: -1, y: 0, z: 0 },
            Cube { x: 0, y: 1, z: 0 },
            Cube { x: 0, y: -1, z: 0 },
            Cube { x: 0, y: 0, z: 1 },
            Cube { x: 0, y: 0, z: -1 },
        ];
        offsets
            .into_iter()
            .map(|o| Cube {
                x: self.x + o.x,
                y: self.y + o.y,
                z: self.z + o.z,
            })
            .filter(move |cube| cube != self)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_example() {
        let input = read_to_string("./example.txt").unwrap();

        let result = solve_problem(&input).unwrap();
        assert_eq!(result, 64);
    }
}
