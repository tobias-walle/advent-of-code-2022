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
            if !cubes.contains(&neighbour) && is_outside(&cubes, &neighbour) {
                sides += 1;
            }
        }
    }
    Ok(sides)
}

fn is_outside(cubes: &HashSet<Cube>, cube: &Cube) -> bool {
    Side::all().any(|side| {
        let other_sides: Vec<_> = Side::all().filter(|s| s != &side).collect();
        let cubes_on_axis: Vec<_> = cubes
            .iter()
            .filter(|other| other_sides.iter().all(|s| other.get(s) == cube.get(s)))
            .collect();
        let something_left_or_top = cubes_on_axis
            .iter()
            .any(|other| other.get(&side) < cube.get(&side));
        let something_right_or_bottom = cubes_on_axis
            .iter()
            .any(|other| other.get(&side) > cube.get(&side));
        !something_left_or_top || !something_right_or_bottom
    })
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
enum Side {
    X,
    Y,
    Z,
}

impl Side {
    fn all() -> impl Iterator<Item = Side> {
        [Side::X, Side::Y, Side::Z].into_iter()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Cube {
    fn get(&self, side: &Side) -> i32 {
        match side {
            Side::X => self.x,
            Side::Y => self.y,
            Side::Z => self.z,
        }
    }

    fn offset(&self, side: &Side, amount: i32) -> Cube {
        let mut cube = self.clone();
        match side {
            Side::X => cube.x += amount,
            Side::Y => cube.y += amount,
            Side::Z => cube.z += amount,
        };
        cube
    }

    fn neighbours(&self) -> impl Iterator<Item = Cube> + '_ {
        Side::all()
            .into_iter()
            .flat_map(|side| [self.offset(&side, -1), self.offset(&side, 1)])
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
        assert_eq!(result, 58);
    }
}
