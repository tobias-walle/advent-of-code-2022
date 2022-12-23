use std::{collections::HashSet, fmt::Debug};

use colored::Colorize;
use eyre::{Context, Result};
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

    let rows = if file_name.contains("example") {
        10
    } else {
        2000000
    };

    let result = solve_problem(&input, rows).context("Failed to solve problem")?;
    println!("{result}");
    Ok(())
}

fn solve_problem(input: &str, row: i32) -> Result<usize> {
    let grid = parsing::parse_with_nom(input, parse_grid)?;
    if is_debugging() {
        render(&grid, row);
    }
    let count = count_points_that_cannot_contain_beacons_in_row(&grid, row);
    Ok(count)
}

fn count_points_that_cannot_contain_beacons_in_row(grid: &Grid, row: i32) -> usize {
    let max_distance = grid
        .pairs
        .iter()
        .map(|p| taxi_cap_distance(&p.sensor, &p.beacon))
        .max()
        .unwrap_or(0);
    let beacons: HashSet<_> = grid.pairs.iter().map(|p| p.beacon).collect();
    let mut limits = limits_of_grid(grid);
    limits.left -= max_distance;
    limits.right += max_distance;
    (limits.left..=limits.right)
        .filter(|x| {
            let point = Point { x: *x, y: row };
            is_in_beacon_radius_of_at_least_one_sensor(grid, &point) && !beacons.contains(&point)
        })
        .count()
}

fn is_in_beacon_radius_of_at_least_one_sensor(grid: &Grid, point: &Point) -> bool {
    grid.pairs.iter().any(|pair| {
        let distance_beacon = taxi_cap_distance(&pair.sensor, &pair.beacon);
        let distance_point = taxi_cap_distance(&pair.sensor, point);
        distance_beacon >= distance_point
    })
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

fn render(grid: &Grid, row_to_analyze: i32) {
    let sensors: HashSet<_> = grid.pairs.iter().map(|p| p.sensor).collect();
    let beacons: HashSet<_> = grid.pairs.iter().map(|p| p.beacon).collect();
    let mut limits = limits_of_grid(&grid);
    let offset = 10;
    limits.top -= offset;
    limits.left -= offset;
    limits.bottom += offset;
    limits.right += offset;
    for y in limits.top..=limits.bottom {
        print!(" {} ", if y == row_to_analyze { '→' } else { ' ' });
        for x in limits.left - 2..=limits.right {
            let point = Point { x, y };
            let pixel = if sensors.contains(&point) {
                "S".bold().truecolor(200, 50, 80)
            } else if beacons.contains(&point) {
                "B".bold().truecolor(50, 80, 200)
            } else if is_in_beacon_radius_of_at_least_one_sensor(grid, &point) {
                "#".truecolor(100, 100, 100)
            } else {
                ".".truecolor(50, 50, 50)
            };
            print!("{}", pixel);
        }
        println!();
    }
}

fn limits_of_grid(grid: &Grid) -> Limits {
    let points: Vec<_> = grid
        .pairs
        .iter()
        .flat_map(|p| [p.sensor, p.beacon])
        .collect();
    limits(&points)
}

fn limits(points: &[Point]) -> Limits {
    Limits {
        top: points.iter().map(|p| p.y).min().unwrap_or(0),
        right: points.iter().map(|p| p.x).max().unwrap_or(0),
        bottom: points.iter().map(|p| p.y).max().unwrap_or(0),
        left: points.iter().map(|p| p.x).min().unwrap_or(0),
    }
}

struct Limits {
    top: i32,
    right: i32,
    bottom: i32,
    left: i32,
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

        let result = solve_problem(&input, 10).unwrap();
        assert_eq!(result, 26);
    }
}
