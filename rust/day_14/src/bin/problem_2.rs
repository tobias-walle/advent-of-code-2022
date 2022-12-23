use colored::Colorize;
use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

use eyre::{Context, Result};
use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, multispace0, space0},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{delimited, terminated, tuple},
    IResult,
};
use utils::{parsing, read_input_file_as_string};

fn main() -> Result<()> {
    let input = read_input_file_as_string().context("Cannot read input")?;

    let result = solve_problem(&input).context("Failed to solve problem")?;
    println!("{result}");
    Ok(())
}

fn solve_problem(input: &str) -> Result<usize> {
    let mut simulation = parsing::parse_with_nom(input, parse_simulation)?;
    while tick(&mut simulation) {}
    render(&simulation);
    let count_sand = simulation
        .get_coordinates(|field| matches!(field, Field::Sand))
        .count();
    Ok(count_sand)
}

fn tick(simulation: &mut Simulation) -> bool {
    let mut new_sand_positions = Vec::new();
    let emitters = simulation.get_coordinates(|field| matches!(field, Field::Emitter));
    let mut continue_simulation = true;
    for emitter in emitters {
        let mut position = *emitter;
        loop {
            let next_position = simulation
                .try_sand_position(position.down())
                .or_else(|| simulation.try_sand_position(position.down().left()))
                .or_else(|| simulation.try_sand_position(position.down().right()));

            match next_position {
                // Next position found, set the position
                Some(next_position) => position = next_position,
                // Next position not found, sand has settled
                None => break,
            }
        }
        new_sand_positions.push(position);
        if position == *emitter {
            // Sand reached the top
            continue_simulation = false;
        }
    }
    for position in new_sand_positions {
        simulation.fields.insert(position, Field::Sand);
    }
    continue_simulation
}

fn render(simulation: &Simulation) {
    let bounds = simulation.bounds();
    let mut result = String::new();
    for y in bounds.top..=bounds.bottom + 2 {
        for x in bounds.left - 3..=bounds.right + 3 {
            let coord = Coordinate::new(x, y);
            let pixel = "█";
            let pixel = match simulation.get(&coord) {
                Field::Air => pixel.truecolor(0, 0, 0),
                Field::Rock => pixel.truecolor(120, 120, 120),
                Field::Sand => pixel.truecolor(61, 61, 40),
                Field::Emitter => pixel.white(),
            };
            result += &format!("{pixel}");
        }
        result += "\n";
    }
    println!("{}", result);
}

fn parse_simulation(input: &str) -> IResult<&str, Simulation> {
    let (input, rock_paths) = all_consuming(terminated(
        separated_list1(line_ending, parse_path),
        multispace0,
    ))(input)?;
    let mut fields: HashMap<_, _> = rock_paths
        .into_iter()
        .flat_map(|path| path.coordinates())
        .map(|coord| (coord, Field::Rock))
        .collect();
    fields.insert(Coordinate::new(500, 0), Field::Emitter);
    let simulation = Simulation::new(fields);
    Ok((input, simulation))
}

fn parse_path(input: &str) -> IResult<&str, Path> {
    let (input, coordinates) = separated_list1(tag("->"), parse_coordinate)(input)?;
    Ok((input, Path(coordinates)))
}

fn parse_coordinate(input: &str) -> IResult<&str, Coordinate> {
    let (input, (x, _, y)) = delimited(
        space0,
        tuple((parsing::number, tag(","), parsing::number)),
        space0,
    )(input)?;
    Ok((input, Coordinate::new(x, y)))
}

#[derive(Debug, Clone)]
struct Path(Vec<Coordinate>);

impl Path {
    fn coordinates(&self) -> HashSet<Coordinate> {
        let mut coordinates = HashSet::new();
        let mut path = self.0.iter();
        let (Some(mut from), Some(mut to)) = (path.next(), path.next()) else {
            return coordinates;
        };
        loop {
            for x in revert_if_necessary(from.x..=to.x) {
                for y in revert_if_necessary(from.y..=to.y) {
                    coordinates.insert(Coordinate::new(x, y));
                }
            }
            let Some(next) = path.next() else {
                return coordinates;
            };
            (from, to) = (to, next);
        }
    }
}

fn revert_if_necessary(range: RangeInclusive<usize>) -> RangeInclusive<usize> {
    if range.start() > range.end() {
        *range.end()..=*range.start()
    } else {
        range
    }
}

#[derive(Debug, Clone)]
struct Simulation {
    fields: HashMap<Coordinate, Field>,
    floor_y: usize,
}

impl Simulation {
    fn new(fields: HashMap<Coordinate, Field>) -> Self {
        let bottom = fields.keys().map(|c| c.y).max().unwrap();
        Self {
            fields,
            floor_y: bottom + 2,
        }
    }

    fn get(&self, coord: &Coordinate) -> &Field {
        if coord.y >= self.floor_y {
            &Field::Rock
        } else {
            self.fields.get(coord).unwrap_or(&Field::Air)
        }
    }

    fn get_coordinates<F>(&self, filter: F) -> impl Iterator<Item = &Coordinate>
    where
        F: Fn(&Field) -> bool,
    {
        self.fields
            .iter()
            .filter(move |(_, field)| filter(field))
            .map(|(coordinate, _)| coordinate)
    }

    fn try_sand_position(&self, position: Coordinate) -> Option<Coordinate> {
        if self.is_blocked(&position) {
            None
        } else {
            Some(position)
        }
    }

    fn is_blocked(&self, position: &Coordinate) -> bool {
        !matches!(self.get(position), Field::Air)
    }

    fn bounds(&self) -> Bounds {
        let coordinates: HashSet<_> = self.fields.keys().collect();
        let right = coordinates.iter().map(|c| c.x).max().unwrap();
        let left = coordinates.iter().map(|c| c.x).min().unwrap();
        let bottom = coordinates.iter().map(|c| c.y).max().unwrap();
        let top = coordinates.iter().map(|c| c.y).min().unwrap();
        Bounds {
            top,
            bottom,
            left,
            right,
        }
    }
}

#[derive(Debug, Clone)]
struct Bounds {
    top: usize,
    bottom: usize,
    left: usize,
    right: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn down(&self) -> Coordinate {
        Coordinate {
            y: self.y + 1,
            ..*self
        }
    }

    pub fn left(&self) -> Coordinate {
        Coordinate {
            x: self.x - 1,
            ..*self
        }
    }

    pub fn right(&self) -> Coordinate {
        Coordinate {
            x: self.x + 1,
            ..*self
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Field {
    Emitter,
    Air,
    Rock,
    Sand,
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_example() {
        let input = read_to_string("./example.txt").unwrap();

        let result = solve_problem(&input).unwrap();
        assert_eq!(result, 93);
    }
}
