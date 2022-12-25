use std::{
    cmp::{min, Reverse},
    collections::{BTreeSet, HashMap, HashSet},
    iter::Rev,
    rc::Rc,
};

use eyre::{bail, Context, ContextCompat, Result};
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    combinator::all_consuming,
    multi::{separated_list0, separated_list1},
    sequence::{preceded, tuple},
    IResult,
};
use priority_queue::PriorityQueue;
use utils::{parsing, read_input_file_as_string};

fn main() -> Result<()> {
    let input = read_input_file_as_string().context("Cannot read input")?;

    let result = solve_problem(&input).context("Failed to solve problem")?;
    println!("{result}");
    Ok(())
}

const MAX_MINUTES: u32 = 30;

fn solve_problem(input: &str) -> Result<u32> {
    let valves = parse(input)?;
    let mut paths = Vec::new();
    for from in valves.keys() {
        for to in valves.keys() {
            let Some(path) = find_path(&valves, from, to) else { continue;};
            paths.push(path);
        }
    }
    println!("Computed {} paths", paths.len());
    let paths: PathsByFrom = paths.into_iter().into_group_map_by(|p| p.from.clone());
    let best_plan = find_best_open_order(&valves, &paths);
    Ok(best_plan.unwrap().pressure_released(&valves))
}

fn find_best_open_order(valves: &ValvesById, paths: &PathsByFrom) -> Option<Plan> {
    let initial = Plan::from([OpenedValve {
        id: ValveId::new("AA"),
        minute: 0,
    }]);
    let mut queue = vec![initial];
    let mut count: u32 = 0;
    let mut removed: u32 = 0;
    let mut best_score: u32 = 0;

    let reduce_query_on = [5, 10, 15, 18, 20, 25];
    let mut max_score_on: HashMap<u32, u32> = HashMap::new();

    let mut best_plan: Option<Plan> = None;
    while let Some(plan) = queue.pop() {
        count += 1;
        if count % 100_000 == 0 {
            println!("{count}: Q={} R={removed} B={best_score}", queue.len());
        }
        let position = plan.position().unwrap();
        let minutes_passed = plan.minutes_passed();
        let minutes_left = MAX_MINUTES.saturating_sub(minutes_passed);
        let mut possible_paths = paths[&position]
            .iter()
            .filter(|path| !plan.opened.contains(&path.to))
            .filter(|path| minutes_left >= path.minutes)
            .peekable();
        if possible_paths.peek().is_none() {
            let score = plan.pressure_released(valves);
            if score > best_score {
                best_score = score;
                best_plan = Some(plan);
            }
            continue;
        }

        for path in possible_paths {
            let mut new_plan = plan.clone();
            let new_minutes = path.minutes + new_plan.minutes_passed();
            new_plan.add(OpenedValve {
                id: path.to.clone(),
                minute: new_minutes,
            });

            let mut add_to_query = true;
            for limit in reduce_query_on {
                if minutes_passed < limit && new_minutes > limit {
                    let new_plan_pressure_released = new_plan.pressure_released(valves);
                    let max_score = *max_score_on
                        .get(&limit)
                        .unwrap_or(&0)
                        .max(&new_plan_pressure_released);
                    max_score_on.insert(limit, max_score);
                    if new_plan_pressure_released < max_score * 95 / 100 {
                        removed += 1;
                        add_to_query = false;
                    }
                    break;
                }
            }

            if add_to_query {
                queue.push(new_plan);
            }
        }
    }

    println!("Found result in {count} iterations.");
    best_plan
}

fn find_path(valves: &ValvesById, from: &ValveId, to: &ValveId) -> Option<Path> {
    let mut queue = vec![vec![from.clone()]];
    let mut paths = vec![];
    while let Some(path) = queue.pop() {
        let node = path.last().unwrap();
        if node == to {
            paths.push(compute_path(valves, &path));
            continue;
        }
        for connection in &valves[node].connections {
            if path.contains(connection) {
                continue;
            }
            let mut new_path = path.clone();
            new_path.push(connection.clone());
            queue.push(new_path);
        }
    }
    paths.into_iter().min_by_key(|path| path.minutes)
}

fn compute_path(valves: &ValvesById, path: &[ValveId]) -> Path {
    let to = path.last().unwrap();
    Path {
        from: path[0].clone(),
        to: to.clone(),
        flow_rate: valves[to].flow_rate,
        minutes: path.len() as u32,
        path: path.to_vec(),
    }
}

fn parse_valves(input: &str) -> IResult<&str, Vec<Valve>> {
    let (input, valves) = all_consuming(separated_list1(newline, parse_valve))(input.trim())?;
    Ok((input, valves))
}

fn parse(input: &str) -> Result<ValvesById> {
    let valves = parsing::parse_with_nom(input, parse_valves)?;
    Ok(valves.into_iter().map(|v| (v.id.clone(), v)).collect())
}

fn parse_valve(input: &str) -> IResult<&str, Valve> {
    let (input, (id, flow_rate, connections)) = tuple((
        preceded(tag("Valve "), alpha1),
        preceded(tag(" has flow rate="), parsing::number),
        preceded(
            alt((
                tag("; tunnels lead to valves "),
                tag("; tunnel leads to valve "),
            )),
            separated_list1(tag(", "), alpha1),
        ),
    ))(input)?;
    Ok((
        input,
        Valve {
            id: ValveId(id.into()),
            flow_rate,
            connections: connections
                .into_iter()
                .map(|id| ValveId(id.into()))
                .collect(),
        },
    ))
}

type ValvesById = HashMap<ValveId, Valve>;
type PathsByFrom = HashMap<ValveId, Vec<Path>>;

#[derive(Debug, Clone, Default)]
struct Plan {
    order: Vec<OpenedValve>,
    opened: HashSet<ValveId>,
}

impl Plan {
    fn new() -> Self {
        Default::default()
    }

    fn add(&mut self, valve: OpenedValve) {
        self.opened.insert(valve.id.clone());
        self.order.push(valve);
    }

    fn pressure_released(&self, valves: &ValvesById) -> u32 {
        let mut total_pressure = 0;
        for order in &self.order {
            let minutes_left = MAX_MINUTES - order.minute;
            total_pressure += minutes_left * valves[&order.id].flow_rate;
        }
        total_pressure
    }

    fn minutes_passed(&self) -> u32 {
        match self.order.last() {
            Some(valve) => valve.minute,
            None => 0,
        }
    }

    pub fn position(&self) -> Option<ValveId> {
        self.order.last().map(|v| v.id.clone())
    }
}

impl<const N: usize> From<[OpenedValve; N]> for Plan {
    fn from(valves: [OpenedValve; N]) -> Self {
        let mut plan = Plan::new();
        for valve in valves {
            plan.add(valve);
        }
        plan
    }
}

#[derive(Debug, Clone)]
struct OpenedValve {
    id: ValveId,
    minute: u32,
}

#[derive(Debug, Clone)]
struct Path {
    from: ValveId,
    to: ValveId,
    path: Vec<ValveId>,
    flow_rate: u32,
    minutes: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct ValveId(String);

impl ValveId {
    fn new(id: &str) -> Self {
        Self(id.into())
    }
}

#[derive(Debug, Clone)]
struct Valve {
    id: ValveId,
    flow_rate: u32,
    connections: Vec<ValveId>,
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_example() {
        let input = read_to_string("./example.txt").unwrap();

        let result = solve_problem(&input).unwrap();
        assert_eq!(result, 1651);
    }

    #[test]
    fn test_plan() {
        let input = read_to_string("./example.txt").unwrap();
        let valves = parse(&input).unwrap();
        let mut path = Plan::new();
        path.add(OpenedValve {
            id: ValveId("DD".into()),
            minute: 2,
        });
        path.add(OpenedValve {
            id: ValveId("BB".into()),
            minute: 5,
        });
        path.add(OpenedValve {
            id: ValveId("JJ".into()),
            minute: 9,
        });
        path.add(OpenedValve {
            id: ValveId("HH".into()),
            minute: 17,
        });
        path.add(OpenedValve {
            id: ValveId("EE".into()),
            minute: 21,
        });
        path.add(OpenedValve {
            id: ValveId("CC".into()),
            minute: 24,
        });

        assert_eq!(path.pressure_released(&valves), 1651);
    }
}
