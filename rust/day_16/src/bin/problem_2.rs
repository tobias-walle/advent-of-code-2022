use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

use eyre::{Context, Result};
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

use utils::{parsing, read_input_file_as_string};

fn main() -> Result<()> {
    let input = read_input_file_as_string().context("Cannot read input")?;

    let result = solve_problem(&input).context("Failed to solve problem")?;
    println!("{result}");
    Ok(())
}

const MAX_MINUTES: u32 = 26;

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
    let best_plan = find_best_path(&valves, &paths);
    Ok(best_plan.unwrap().pressure_released)
}

fn find_best_path(valves: &ValvesById, paths: &PathsByFrom) -> Option<Plan> {
    let mut initial = Plan::new();
    for traveller in Traveller::values() {
        initial.add(
            &traveller,
            valves,
            OpenedValve {
                id: ValveId::new("AA"),
                minute: 0,
            },
        );
    }
    let mut queue = BinaryHeap::new();
    queue.push(initial);
    let mut count: u32 = 0;
    let mut removed: u32 = 0;
    let mut best_score: u32 = 0;

    let reduce_query_on = [10, 12, 15, 18, 20, 25];
    let mut max_score_on: HashMap<u32, u32> = HashMap::new();

    let count_travellers = Traveller::values().count();
    let mut best_plan: Option<Plan> = None;
    while let Some(plan) = queue.pop() {
        count += 1;
        if count % 100_000 == 0 {
            println!("{count}: Q={} R={removed} B={best_score}", queue.len());
        }

        let mut count_possible_paths_empty = 0;
        for traveller in Traveller::values() {
            let position = plan.position(&traveller).unwrap();
            let minutes_passed = plan.minutes_passed(&traveller);
            let minutes_left = MAX_MINUTES.saturating_sub(minutes_passed);
            let mut possible_paths = paths[&position]
                .iter()
                .filter(|path| valves[&path.to].flow_rate != 0)
                .filter(|path| !plan.opened.contains(&path.to))
                .filter(|path| minutes_left >= path.minutes)
                .peekable();
            if possible_paths.peek().is_none() {
                count_possible_paths_empty += 1;
                continue;
            }

            for path in possible_paths {
                let mut new_plan = plan.clone();
                let new_minutes = path.minutes + new_plan.minutes_passed(&traveller);
                new_plan.add(
                    &traveller,
                    valves,
                    OpenedValve {
                        id: path.to.clone(),
                        minute: new_minutes,
                    },
                );

                let mut add_to_query = true;
                for limit in reduce_query_on {
                    if minutes_passed < limit && new_minutes > limit {
                        let new_plan_pressure_released = new_plan.pressure_released;
                        let max_score = *max_score_on
                            .get(&limit)
                            .unwrap_or(&0)
                            .max(&new_plan.pressure_released);
                        max_score_on.insert(limit, max_score);
                        if new_plan_pressure_released < max_score * 90 / 100 {
                            removed += 1;
                            add_to_query = false;
                        }
                        break;
                    }
                }

                if add_to_query {
                    new_plan.update_heuristic(valves);
                    queue.push(new_plan);
                }
            }
        }

        if count_possible_paths_empty == count_travellers {
            let score = plan.pressure_released;
            if score > best_score {
                best_score = score;
                best_plan = Some(plan);
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
            paths.push(compute_path(&path));
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

fn compute_path(path: &[ValveId]) -> Path {
    let to = path.last().unwrap();
    Path {
        from: path[0].clone(),
        to: to.clone(),
        minutes: path.len() as u32,
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Traveller {
    Me,
    Elephant,
}

impl Traveller {
    fn values() -> impl Iterator<Item = Traveller> {
        [Traveller::Me, Traveller::Elephant].into_iter()
    }
}

#[derive(Debug, Clone, Default)]
struct Plan {
    order: HashMap<Traveller, Vec<OpenedValve>>,
    opened: HashSet<ValveId>,
    pressure_released: u32,
    heuristic: u32,
}

impl Ord for Plan {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.heuristic.cmp(&other.heuristic)
    }
}
impl PartialOrd for Plan {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Plan {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Plan {}

impl Plan {
    fn new() -> Self {
        let mut value = Plan::default();
        for traveller in Traveller::values() {
            value.order.insert(traveller, vec![]);
        }
        value
    }

    fn add(&mut self, traveller: &Traveller, valves: &ValvesById, valve: OpenedValve) {
        self.opened.insert(valve.id.clone());
        self.order.get_mut(traveller).unwrap().push(valve.clone());
        let minutes_left = MAX_MINUTES - valve.minute;
        self.pressure_released += minutes_left * valves[&valve.id].flow_rate;
    }

    fn update_heuristic(&mut self, valves: &ValvesById) {
        let mut heuristic_factor = 0;
        let unopened = valves.values().filter(|v| !self.opened.contains(&v.id));
        for traveller in Traveller::values() {
            let valve = self.order[&traveller].last().unwrap();
            let minutes_left = MAX_MINUTES - valve.minute;
            heuristic_factor += unopened
                .clone()
                .map(|v| v.flow_rate * minutes_left)
                .sum::<u32>();
        }
        self.heuristic = self.pressure_released + heuristic_factor / 4
    }

    fn minutes_passed(&self, traveller: &Traveller) -> u32 {
        match self.order[traveller].last() {
            Some(valve) => valve.minute,
            None => 0,
        }
    }

    pub fn position(&self, traveller: &Traveller) -> Option<ValveId> {
        self.order[traveller].last().map(|v| v.id.clone())
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
        assert_eq!(result, 1707);
    }
}
