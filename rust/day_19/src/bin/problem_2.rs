use std::{collections::HashMap, sync::Arc, thread};

use eyre::{Context, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace0, multispace1, space0},
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

const TIME_IN_MINUTES: u32 = 32;

fn solve_problem(input: &str) -> Result<i32> {
    let mut blueprints = parse_with_nom(input, parse)?;
    if blueprints.len() > 3 {
        blueprints = blueprints[..3].to_vec();
    }

    let mut scores = vec![];
    for blueprint in blueprints {
        scores.push(thread::spawn(move || {
            let best = find_optimal_solution_for_blueprint(&blueprint);
            (blueprint, best.score())
        }));
    }

    let result = scores
        .into_iter()
        .map(|handle| {
            let (blueprint, score) = handle.join().unwrap();
            let blueprint_id = blueprint.id;
            println!("Blueprint {blueprint_id} - Score: {score}");
            score
        })
        .product();
    Ok(result)
}

fn find_optimal_solution_for_blueprint(blueprint: &Blueprint) -> State {
    let robots_by_resource: HashMap<_, _> = blueprint
        .robots
        .iter()
        .map(|robot| (robot.resource.clone(), robot.clone()))
        .collect();
    let mut query: Vec<_> = blueprint
        .robots
        .iter()
        .map(|robot| State {
            minute: 0,
            robots: vec![robots_by_resource[&Resource::Ore].clone()],
            resources: HashMap::new(),
            next_to_purchase: robot.clone(),
            purchase_history: vec![],
        })
        .collect();

    let mut completed = vec![];

    let mut max_robot_cost_by_resource = HashMap::new();
    for robot in &blueprint.robots {
        for (resource, costs) in &robot.costs {
            let max_costs = *max_robot_cost_by_resource.get(resource).unwrap_or(&0);
            if *costs > max_costs {
                max_robot_cost_by_resource.insert(resource.clone(), *costs);
            }
        }
    }

    while let Some(mut state) = query.pop() {
        if state.minute == TIME_IN_MINUTES {
            completed.push(state);
            continue;
        }

        state.minute += 1;

        // Check if next robot can be afforted
        let to_purchase = state.next_to_purchase.clone();
        let can_be_afforted = to_purchase.can_affort(&state.resources);

        // Collect resources from robots
        for robot in &state.robots {
            let amount = *state.resources.get(&robot.resource).unwrap_or(&0) + 1;
            state.resources.insert(robot.resource.clone(), amount);
        }

        // Cancel if next purchase decision cannot be afforted
        if !can_be_afforted {
            query.push(state);
            continue;
        }

        // Purchase
        to_purchase.subtract_costs(&mut state.resources);
        state.robots.push(to_purchase.clone());
        state.purchase_history.push(Purchase {
            minute: state.minute,
            robot: to_purchase.clone(),
        });

        // Next possible decisions
        let next_possible_decisions = blueprint
            .robots
            .iter()
            .filter(|robot| {
                // Do not buy robots if there are already enough resources available
                let amount = state.resources.get(&robot.resource).unwrap_or(&0);
                let max_robot_costs = &max_robot_cost_by_resource
                    .get(&robot.resource)
                    .unwrap_or(&i32::MAX);
                *amount <= max_robot_costs.saturating_add(1)
            })
            .map(|next_to_purchase| State {
                next_to_purchase: next_to_purchase.clone(),
                ..state.clone()
            });
        query.extend(next_possible_decisions);
    }

    completed
        .into_iter()
        .max_by_key(|state| state.score())
        .unwrap()
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
            robots: robots.into_iter().map(Arc::new).collect(),
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    minute: u32,
    resources: Resources,
    robots: Vec<Arc<Robot>>,
    next_to_purchase: Arc<Robot>,
    purchase_history: Vec<Purchase>,
}

type Resources = HashMap<Resource, i32>;

impl State {
    fn score(&self) -> i32 {
        *self.resources.get(&Resource::Geode).unwrap_or(&0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Purchase {
    minute: u32,
    robot: Arc<Robot>,
}

#[derive(Debug, Clone)]
struct Blueprint {
    id: u8,
    robots: Vec<Arc<Robot>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
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
    fn test_find_optimal_solution_for_blueprint() {
        let input = read_to_string("./example.txt").unwrap();

        let blueprints = parse_with_nom(&input, parse).unwrap();
        let best = find_optimal_solution_for_blueprint(&blueprints[0]);

        assert_eq!(best.score(), 56);
    }
}
