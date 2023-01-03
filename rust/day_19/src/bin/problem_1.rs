use std::{collections::HashMap, rc::Rc};

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

const TIME_IN_MINUTES: u32 = 24;

fn solve_problem(input: &str) -> Result<i32> {
    let blueprints = parse_with_nom(input, parse)?;
    let best = find_optimal_solution_for_blueprint(&blueprints[0]);
    Ok(best.score())
}

fn find_optimal_solution_for_blueprint(blueprint: &Blueprint) -> State {
    let initial_state = State {
        minute: 0,
        robots: vec![blueprint.get_robot(&Resource::Ore)],
        resources: HashMap::new(),
        purchase_history: vec![],
    };
    let mut query = vec![initial_state];
    let mut completed = vec![];
    while let Some(mut state) = query.pop() {
        if completed.len() % 100_000 == 0 {
            println!("{}: {} / {}", state.minute, query.len(), completed.len());
        }
        if state.minute == TIME_IN_MINUTES {
            completed.push(state);
            continue;
        }

        state.minute += 1;

        // Check what can be purchased
        let purchasable: Vec<_> = blueprint
            .robots
            .values()
            .cloned()
            .filter(|robot| robot.can_affort(&state.resources))
            .collect();

        // Collect resources from robots
        for robot in &state.robots {
            let amount = *state.resources.get(&robot.resource).unwrap_or(&0) + 1;
            state.resources.insert(robot.resource.clone(), amount);
        }

        // Add all possible desicions to query
        let purchase_decisions: Vec<_> = purchasable
            .iter()
            .map(|robot| {
                let mut state = state.clone();
                robot.subtract_costs(&mut state.resources);
                state.robots.push(robot.clone());
                state.purchase_history.push(Purchase {
                    minute: state.minute,
                    robot: robot.clone(),
                });
                state
            })
            .collect();
        let non_purchase_decision = state.clone();

        query.push(non_purchase_decision);
        query.extend(purchase_decisions);
    }
    dbg!(&completed);

    completed.into_iter().max_by_key(State::score).unwrap()
}

fn parse(input: &str) -> IResult<&str, Vec<Blueprint>> {
    all_consuming(many1(map(
        tuple((
            preceded(tag("Blueprint "), parsing::number),
            tuple((tag(":"), multispace1)),
            parse_robots,
        )),
        |(id, _, robots)| Blueprint {
            id,
            robots: robots
                .into_iter()
                .map(|r| (r.resource.clone(), Rc::new(r)))
                .collect(),
        },
    )))(input.trim())
}

fn parse_robots(input: &str) -> IResult<&str, Vec<Robot>> {
    many1(map(
        tuple((
            preceded(tag("Each "), parse_resource),
            preceded(tag(" robot costs "), parse_costs),
            tuple((tag("."), multispace0)),
        )),
        |(resource, costs, _)| Robot { resource, costs },
    ))(input)
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
struct State {
    minute: u32,
    resources: Resources,
    robots: Vec<Rc<Robot>>,
    purchase_history: Vec<Purchase>,
}

type Resources = HashMap<Resource, i32>;

impl State {
    fn score(&self) -> i32 {
        self.resources[&Resource::Geode]
    }
}

#[derive(Debug, Clone)]
struct Purchase {
    minute: u32,
    robot: Rc<Robot>,
}

#[derive(Debug, Clone)]
struct Blueprint {
    id: u8,
    robots: HashMap<Resource, Rc<Robot>>,
}

impl Blueprint {
    fn get_robot(&self, resource: &Resource) -> Rc<Robot> {
        self.robots[resource].clone()
    }
}

#[derive(Debug, Clone)]
struct Robot {
    resource: Resource,
    costs: HashMap<Resource, i32>,
}

impl Robot {
    fn can_affort(&self, resources: &Resources) -> bool {
        self.costs
            .iter()
            .all(|(resource, costs)| resources.get(resource).unwrap_or(&0) >= costs)
    }

    fn subtract_costs(&self, resources: &mut Resources) {
        if !self.can_affort(resources) {
            return;
        }

        for (resource, costs) in &self.costs {
            let mut amount = *resources.get(resource).unwrap_or(&0);
            amount -= costs;
            resources.insert(resource.clone(), amount);
        }
    }
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
