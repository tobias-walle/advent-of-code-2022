use std::collections::HashSet;

use derive_more::*;
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
    let sides = get_outside_sides(&cubes);
    Ok(sides.len())
}

fn get_outside_sides(cubes: &HashSet<Cube>) -> HashSet<Side> {
    let all_sides = get_sides(cubes);
    let mut query: Vec<_> = all_sides
        .iter()
        .filter(|s| !is_other_cube_infront(cubes, s))
        .cloned()
        .collect();
    let mut result = HashSet::new();
    while let Some(next) = query.pop() {
        let neighbours = next.neighbours(cubes, &all_sides);
        for neighbour in neighbours {
            if !result.contains(&neighbour) {
                query.push(neighbour.clone());
            }
        }
        result.insert(next);
    }
    result
}

fn get_sides(cubes: &HashSet<Cube>) -> HashSet<Side> {
    let mut sides = HashSet::new();
    for cube in cubes.iter() {
        for neighbour in cube.neighbours() {
            if !cubes.contains(&neighbour) {
                let direction = neighbour.coords() - cube.coords();
                let direction = direction.direction().unwrap();
                let side = Side {
                    origin: cube.clone(),
                    direction,
                };
                sides.insert(side);
            }
        }
    }
    sides
}

fn is_other_cube_infront(cubes: &HashSet<Cube>, side: &Side) -> bool {
    let origin = &side.origin;
    let direction = &side.direction;
    let other_axis: Vec<_> = Axis::all_but(&direction.axis).collect();
    let cubes_on_axis: Vec<_> = cubes
        .iter()
        .filter(|other| *other != origin)
        .filter(|other| other_axis.iter().all(|s| other.get(s) == origin.get(s)))
        .collect();
    let is_other_cube_in_front = cubes_on_axis.iter().any(|other| {
        if direction.inverted {
            other.get(&direction.axis) < origin.get(&direction.axis)
        } else {
            other.get(&direction.axis) > origin.get(&direction.axis)
        }
    });
    is_other_cube_in_front
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
            |(x, y, z)| Cube::new(x, y, z),
        ),
    ))(input.trim())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Side {
    origin: Cube,
    direction: Direction,
}

impl Side {
    fn neighbours<'a>(
        &self,
        cubes: &HashSet<Cube>,
        all_sides: &'a HashSet<Side>,
    ) -> impl Iterator<Item = Self> + 'a {
        let mut possible_neighbours = HashSet::new();
        for other in Axis::all_but(&self.direction.axis) {
            for inverted in [true, false] {
                let shifted_to_other = self.origin.offset(&other, if inverted { -1 } else { 1 });
                let shifted_diagonally = shifted_to_other.offset(
                    &self.direction.axis,
                    if self.direction.inverted { -1 } else { 1 },
                );
                if cubes.contains(&shifted_diagonally) {
                    possible_neighbours.insert(Side {
                        origin: shifted_diagonally,
                        direction: Direction::new(other.clone(), !inverted),
                    });
                } else if cubes.contains(&shifted_to_other) {
                    possible_neighbours.insert(Side {
                        origin: shifted_to_other,
                        direction: self.direction.clone(),
                    });
                } else {
                    possible_neighbours.insert(Side {
                        origin: self.origin.clone(),
                        direction: Direction::new(other.clone(), inverted),
                    });
                }
            }
        }
        possible_neighbours
            .into_iter()
            .filter(move |n| all_sides.contains(n))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Cube(Vec3);

impl Cube {
    fn new(x: AxisUnit, y: AxisUnit, z: AxisUnit) -> Self {
        Self(Vec3 { x, y, z })
    }

    fn get(&self, side: &Axis) -> AxisUnit {
        self.0.get(side)
    }

    fn offset(&self, side: &Axis, amount: AxisUnit) -> Cube {
        Cube(self.0.offset(side, amount))
    }

    fn coords(&self) -> Vec3 {
        self.0.clone()
    }

    fn neighbours(&self) -> impl Iterator<Item = Cube> + '_ {
        Axis::all()
            .into_iter()
            .flat_map(|side| [self.offset(&side, -1), self.offset(&side, 1)])
            .filter(move |cube| cube != self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Add, Sub)]
pub struct Vec3 {
    x: AxisUnit,
    y: AxisUnit,
    z: AxisUnit,
}

impl Vec3 {
    pub fn get(&self, axis: &Axis) -> AxisUnit {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
            Axis::Z => self.z,
        }
    }

    pub fn offset(&self, axis: &Axis, amount: AxisUnit) -> Self {
        let mut vec3 = self.clone();
        match axis {
            Axis::X => vec3.x += amount,
            Axis::Y => vec3.y += amount,
            Axis::Z => vec3.z += amount,
        };
        vec3
    }

    /// Only works on normalized vector
    pub fn direction(&self) -> Option<Direction> {
        let direction = match (self.x, self.y, self.z) {
            (1, 0, 0) => Direction::new(Axis::X, false),
            (0, 1, 0) => Direction::new(Axis::Y, false),
            (0, 0, 1) => Direction::new(Axis::Z, false),
            (-1, 0, 0) => Direction::new(Axis::X, true),
            (0, -1, 0) => Direction::new(Axis::Y, true),
            (0, 0, -1) => Direction::new(Axis::Z, true),
            _ => return None,
        };
        Some(direction)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Direction {
    axis: Axis,
    inverted: bool,
}

impl Direction {
    fn new(axis: Axis, inverted: bool) -> Self {
        Self { axis, inverted }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Add)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    fn all() -> impl Iterator<Item = Axis> {
        [Axis::X, Axis::Y, Axis::Z].into_iter()
    }

    fn all_but(other: &Axis) -> impl Iterator<Item = Axis> + '_ {
        Self::all().filter(move |axis| axis != other)
    }
}

type AxisUnit = i32;

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
