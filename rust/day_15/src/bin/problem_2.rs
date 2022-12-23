use std::{collections::HashSet, fmt::Debug};

use colored::Colorize;
use eyre::{Context, ContextCompat, Result};
use nom::{
    bytes::complete::tag,
    character::complete::newline,
    combinator::complete,
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};
use utils::{get_input_file_name_from_args, is_debugging, parsing, read_input_file_as_string};

fn main() -> Result<()> {
    let file_name = get_input_file_name_from_args()?;
    let input = read_input_file_as_string().context("Cannot read input")?;

    let search_radius = if file_name.contains("example") {
        20
    } else {
        4_000_000
    };

    let result = solve_problem(&input, search_radius).context("Failed to solve problem")?;
    println!("{result}");
    Ok(())
}

fn solve_problem(input: &str, search_radius: i32) -> Result<usize> {
    let grid = parsing::parse_with_nom(input, parse_grid)?;
    let point = find_beacon(&grid, search_radius).context("Beacon not found")?;
    println!("Found {point:?}");
    Ok(point.x as usize * 4_000_000 + point.y as usize)
}

fn find_beacon(grid: &Grid, search_radius: i32) -> Option<Point> {
    let mut highlights = HashSet::new();
    for y in 0..=search_radius {
        let mut x = 0;
        while x < search_radius {
            let point = Point { x, y };
            if is_debugging() {
                highlights.insert(point);
            }
            let Some(next_x) = find_next_x_position(grid, &point) else {
                if is_debugging() {
                    render(grid, search_radius, highlights);
                }
                return Some(point)
            };
            x = next_x;
        }
    }
    None
}

fn is_in_beacon_radius_of_at_least_one_sensor(grid: &Grid, point: &Point) -> bool {
    grid.pairs.iter().any(|pair| {
        let distance_beacon = taxi_cap_distance(&pair.sensor, &pair.beacon);
        let distance_point = taxi_cap_distance(&pair.sensor, point);
        distance_beacon >= distance_point
    })
}

fn find_next_x_position(grid: &Grid, point: &Point) -> Option<i32> {
    grid.pairs
        .iter()
        .filter_map(|pair| {
            let distance_beacon = taxi_cap_distance(&pair.sensor, &pair.beacon);
            let distance_point = taxi_cap_distance(&pair.sensor, point);
            if distance_beacon < distance_point {
                return None;
            }
            let vertical_distance = point.y.abs_diff(pair.sensor.y) as i32;
            let right_side = pair.sensor.x + distance_beacon - vertical_distance;
            Some(right_side + 1)
        })
        .max()
}

fn taxi_cap_distance(p1: &Point, p2: &Point) -> i32 {
    let result = p1.x.abs_diff(p2.x) + p1.y.abs_diff(p2.y);
    result as i32
}

fn parse_grid(input: &str) -> IResult<&str, Grid> {
    let (input, pairs) = complete(separated_list1(newline, parse_sensor_beacon_pair))(input)?;
    Ok((input, Grid { pairs }))
}

fn parse_sensor_beacon_pair(input: &str) -> IResult<&str, SensorBeaconPair> {
    let (input, (sensor, beacon)) = tuple((
        preceded(tag("Sensor at "), parse_point),
        preceded(tag(": closest beacon is at "), parse_point),
    ))(input)?;
    Ok((input, SensorBeaconPair { sensor, beacon }))
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    let (input, (x, y)) = tuple((
        preceded(tag("x="), parsing::number),
        preceded(tag(", y="), parsing::number),
    ))(input)?;
    Ok((input, Point { x, y }))
}

fn render(grid: &Grid, search_radius: i32, highlights: HashSet<Point>) {
    let sensors: HashSet<_> = grid.pairs.iter().map(|p| p.sensor).collect();
    let beacons: HashSet<_> = grid.pairs.iter().map(|p| p.beacon).collect();
    for y in 0..=search_radius {
        for x in 0..=search_radius {
            let point = Point { x, y };
            let mut pixel = if sensors.contains(&point) {
                "S".bold().truecolor(200, 50, 80)
            } else if beacons.contains(&point) {
                "B".bold().truecolor(50, 80, 200)
            } else if is_in_beacon_radius_of_at_least_one_sensor(grid, &point) {
                "#".truecolor(100, 100, 100)
            } else {
                ".".truecolor(50, 50, 50)
            };
            if highlights.contains(&point) {
                pixel = pixel.truecolor(50, 200, 80);
            }
            print!("{}", pixel);
        }
        println!();
    }
}

#[derive(Debug, Clone)]
struct Grid {
    pairs: Vec<SensorBeaconPair>,
}

#[derive(Debug, Clone)]
struct SensorBeaconPair {
    sensor: Point,
    beacon: Point,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_example() {
        let input = read_to_string("./example.txt").unwrap();

        let result = solve_problem(&input, 20).unwrap();
        assert_eq!(result, 56000011);
    }
}
