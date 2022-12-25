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
    let valves = parsing::parse_with_nom(input, parse_valves)?;
    let valves: ValvesById = valves.into_iter().map(|v| (v.id.clone(), v)).collect();
    let mut paths = Vec::new();
    for from in valves.keys() {
        for to in valves.keys() {
            let Some(path) = find_path(&valves, from, to) else { continue;};
            paths.push(path);
        }
    }
    let paths = paths.into_iter().into_group_map_by(|p| p.from.clone());
    let mut current_valve = valves.values().max_by_key(|v| v.flow_rate).unwrap();
    let mut opened = HashSet::from([current_valve.id.clone()]);
    let mut total_flow_rate = current_valve.flow_rate;
    let mut pressure = 0;
    let mut minute: u32 = 3;

    loop {
        let minutes_left = MAX_MINUTES - minute;
        let next_path = paths[&current_valve.id]
            .iter()
            .filter(|path| !opened.contains(&path.to))
            .filter(|path| path.minutes < minutes_left)
            .max_by_key(|path| calculate_path_score(path, minutes_left));
        let Some(next_path) = next_path else { break };

        let next_valve = &valves[&next_path.to];
        let minutes_move_and_open = next_path.minutes + 1;
        pressure += total_flow_rate * minutes_move_and_open;
        minute += minutes_move_and_open;
        total_flow_rate += next_valve.flow_rate;
        opened.insert(next_valve.id.clone());
        current_valve = next_valve;
    }
    let minutes_left = MAX_MINUTES - minute;
    pressure += minutes_left * total_flow_rate;
    Ok(pressure)
}

fn calculate_path_score(path: &Path, minutes_left: u32) -> Option<u32> {
    if minutes_left < path.minutes {
        return None;
    }
    let score = (minutes_left - path.minutes) * path.flow_rate;
    Some(score)
}

fn find_path(valves: &ValvesById, from: &ValveId, to: &ValveId) -> Option<Path> {
    let mut queue = vec![vec![from.clone()]];
    while let Some(path) = queue.pop() {
        let node = path.last().unwrap();
        if node == to {
            return Some(compute_path(valves, &path));
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
    None
}

fn compute_path(valves: &ValvesById, path: &[ValveId]) -> Path {
    let to = path.last().unwrap();
    Path {
        from: path[0].clone(),
        to: to.clone(),
        flow_rate: valves[to].flow_rate,
        minutes: (path.len() - 1) as u32,
        path: path.to_vec(),
    }
}

fn parse_valves(input: &str) -> IResult<&str, Vec<Valve>> {
    let (input, valves) = all_consuming(separated_list1(newline, parse_valve))(input.trim())?;
    Ok((input, valves))
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
type Paths<'a> = HashMap<(&'a ValveId, &'a ValveId), Path>;

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
}
