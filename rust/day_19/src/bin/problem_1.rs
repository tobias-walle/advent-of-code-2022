use std::collections::HashMap;

use eyre::{Context, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace0, multispace1, newline, space0},
    combinator::{all_consuming, map},
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, terminated, tuple},
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
    let blueprints = parse_with_nom(input, parse)?;
    dbg!(&blueprints);
    Ok(0)
}

fn parse(input: &str) -> IResult<&str, Vec<Blueprint>> {
    all_consuming(many1(map(
        tuple((
            delimited(
                tag("Blueprint "),
                parsing::number,
                tuple((tag(":"), multispace1)),
            ),
            parse_robots,
        )),
        |(id, robots)| Blueprint { id, robots },
    )))(input.trim())
}

fn parse_robots(input: &str) -> IResult<&str, Vec<Robot>> {
    terminated(
        separated_list1(
            terminated(tag("."), multispace1),
            map(
                tuple((
                    preceded(tag("Each "), parse_resource),
                    preceded(tag(" robot costs "), parse_costs),
                )),
                |(resource, costs)| Robot { resource, costs },
            ),
        ),
        tuple((tag("."), multispace0)),
    )(input)
}

fn parse_costs(input: &str) -> IResult<&str, HashMap<Resource, i32>> {
    map(
        separated_list1(
            delimited(space0, tag("and"), space0),
            map(
                tuple((terminated(parsing::number, space0), parse_resource)),
                |(costs, resource)| (resource, costs),
            ),
        ),
        |costs| costs.into_iter().collect(),
    )(input)
}

fn parse_resource(input: &str) -> IResult<&str, Resource> {
    alt((
        map(tag("ore"), |_| Resource::Ore),
        map(tag("clay"), |_| Resource::Clay),
        map(tag("obsidian"), |_| Resource::Obsidian),
        map(tag("geode"), |_| Resource::Geode),
    ))(input)
}

#[derive(Debug, Clone)]
struct Blueprint {
    id: u8,
    robots: Vec<Robot>,
}

#[derive(Debug, Clone)]
struct Robot {
    resource: Resource,
    costs: HashMap<Resource, i32>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_example() {
        let input = read_to_string("./example.txt").unwrap();

        let result = solve_problem(&input).unwrap();
        assert_eq!(result, 0);
    }
}
