use std::{
    cmp::Reverse,
    collections::{BTreeSet, HashMap, HashSet},
    iter::Rev,
    rc::Rc,
};

use eyre::{bail, Context, ContextCompat, Result};
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

fn solve_problem(input: &str) -> Result<u32> {
    let valves = parsing::parse_with_nom(input, parse_valves)?;
    let best_tree = build_decision_trees(valves, 30);
    Ok(best_tree.real_pressure)
}

fn build_decision_trees(valves: Vec<Valve>, max_minutes: u32) -> Rc<DecisionTree> {
    let valves: HashMap<_, _> = valves.into_iter().map(|v| (v.id.clone(), v)).collect();
    let valves = Rc::new(valves);
    let initial: Vec<_> = valves
        .values()
        .map(|valve| {
            Rc::new(DecisionTree {
                valve_id: valve.id.clone(),
                minutes: 1,
                real_pressure: 0,
                pontential_pressure: 0,
                opened: BTreeSet::new(),
                parent: None,
            })
        })
        .collect();
    find_best_tree(initial, valves, max_minutes).unwrap()
}

fn find_best_tree(
    start: Vec<Rc<DecisionTree>>,
    valves: Rc<HashMap<ValveId, Valve>>,
    max_minutes: u32,
) -> Result<Rc<DecisionTree>> {
    let mut queue = PriorityQueue::new();
    let mut scores = HashMap::new();

    for item in start {
        queue.push(item.clone(), item.pontential_pressure);
        scores.insert(item.key(), item.real_pressure);
    }

    while !queue.is_empty() {
        let (current, _) = queue.pop().unwrap();
        if current.minutes == 30 {
            return Ok(current);
        }

        for neighbour in get_next_decision_trees(&current, &valves, max_minutes) {
            let tentative_score = neighbour.real_pressure;
            let neighbour_score = scores.get(&neighbour.key());
            if neighbour_score.is_none() || tentative_score > *neighbour_score.unwrap() {
                scores.insert(neighbour.key(), tentative_score);
                queue.push(neighbour.clone(), neighbour.real_pressure);
            }
        }
    }

    bail!("No path found!")
}

fn find_best_decision_tree(
    start: Rc<DecisionTree>,
    valves: Rc<HashMap<ValveId, Valve>>,
    max_minutes: u32,
    cache: &mut HashMap<Rc<DecisionTree>, Rc<DecisionTree>>,
) -> Rc<DecisionTree> {
    let mut queue = PriorityQueue::new();
    queue.push(start.clone(), start.minutes);
    let mut possible_decision_trees = Vec::new();
    while let Some((next, _)) = queue.pop() {
        if cache.contains_key(&next) {
            continue;
        }
        for next_decision in get_next_decision_trees(&next, &valves, max_minutes) {
            match next_decision.minutes.cmp(&max_minutes) {
                std::cmp::Ordering::Equal => {
                    possible_decision_trees.push(next_decision.clone());

                    let mut parent = &next_decision.parent;
                    while let Some(tree) = parent {
                        cache.insert(tree.clone(), next_decision.clone());
                        parent = &tree.parent;
                    }
                }
                std::cmp::Ordering::Less => {
                    queue.push(next_decision.clone(), next_decision.minutes);
                }
                std::cmp::Ordering::Greater => (),
            }
        }
    }
    possible_decision_trees
        .into_iter()
        .max_by_key(|tree| tree.real_pressure)
        .unwrap()
}

fn get_next_decision_trees(
    start: &Rc<DecisionTree>,
    valves: &HashMap<ValveId, Valve>,
    max_minutes: u32,
) -> Vec<Rc<DecisionTree>> {
    let valve = &valves[&start.valve_id];
    let mut decisions = Vec::new();
    let new_pressure: u32 = start.opened.iter().map(|v| valves[v].flow_rate).sum();
    let real_pressure = start.real_pressure + new_pressure;
    let minutes = start.minutes + 1;
    let minutes_left = max_minutes - minutes;
    if !start.opened.contains(&start.valve_id) {
        let mut opened = start.opened.clone();
        opened.insert(start.valve_id.clone());
        decisions.push(Rc::new(DecisionTree {
            valve_id: start.valve_id.clone(),
            minutes,
            real_pressure,
            pontential_pressure: real_pressure + valve.flow_rate * minutes_left,
            opened,
            parent: Some(start.clone()),
        }));
    }
    for connection_id in &valve.connections {
        decisions.push(Rc::new(DecisionTree {
            valve_id: connection_id.clone(),
            minutes,
            real_pressure,
            pontential_pressure: real_pressure,
            opened: start.opened.clone(),
            parent: Some(start.clone()),
        }));
    }
    decisions
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct DecisionTree {
    valve_id: ValveId,
    minutes: u32,
    pontential_pressure: u32,
    real_pressure: u32,
    opened: BTreeSet<ValveId>,
    parent: Option<Rc<DecisionTree>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct DecisionTreeKey {
    valve_id: ValveId,
    minutes: u32,
    opened: BTreeSet<ValveId>,
}

type Chain = Vec<ValveId>;

impl DecisionTree {
    fn chain(self: Rc<Self>) -> Vec<ValveId> {
        let mut chain = vec![];
        let mut next = Some(self);
        while let Some(tree) = next {
            chain.push(tree.valve_id.clone());
            next = tree.parent.clone();
        }
        chain
    }

    fn key(&self) -> DecisionTreeKey {
        DecisionTreeKey {
            valve_id: self.valve_id.clone(),
            minutes: self.minutes,
            opened: self.opened.clone(),
        }
    }
}

#[derive(Debug, Clone)]
struct Problem {}

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
