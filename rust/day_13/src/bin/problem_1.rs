use std::{cmp::Ordering, fmt::Debug};

use eyre::{Context, Result};
use serde::Deserialize;
use utils::read_input_file_as_string;

fn main() -> Result<()> {
    let input = read_input_file_as_string().context("Cannot read input")?;

    let result = solve_problem(&input).context("Failed to solve problem")?;
    println!("{result}");
    Ok(())
}

fn solve_problem(input: &str) -> Result<usize> {
    let pairs = parse(input)?;
    let result: usize = pairs
        .iter()
        .enumerate()
        .filter(|(_, pair)| pair.is_correct_order())
        .map(|(i, _)| i + 1)
        .sum();
    Ok(result)
}

fn parse(input: &str) -> Result<Vec<Pair>> {
    let lines: Vec<_> = input
        .trim()
        .lines()
        .filter(|line| !line.is_empty())
        .collect();
    let mut pairs = Vec::new();
    for i in (0..lines.len()).step_by(2) {
        let pair = Pair(parse_line(lines[i])?, parse_line(lines[i + 1])?);
        pairs.push(pair);
    }
    Ok(pairs)
}

fn parse_line(line: &str) -> Result<Value> {
    serde_json::from_str(line).context("Cannot parse input")
}

#[derive(Debug)]
struct Pair(Value, Value);

impl Pair {
    fn is_correct_order(&self) -> bool {
        self.0 < self.1
    }
}

#[derive(Deserialize, Clone)]
#[serde(untagged)]
enum Value {
    Number(i32),
    Array(Vec<Value>),
}

impl Value {
    fn to_array(&self) -> Self {
        match self {
            Value::Number(_) => Self::Array(vec![self.clone()]),
            Value::Array(_) => self.clone(),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match (self, other) {
            (Self::Number(v1), Self::Number(v2)) if v1 == v2 => Ordering::Equal,
            (Self::Number(v1), Self::Number(v2)) if v1 < v2 => Ordering::Less,
            (Self::Number(_), Self::Number(_)) => Ordering::Greater,
            (Self::Array(v1), Self::Array(v2)) => {
                for (i, v1_item) in v1.iter().enumerate() {
                    let v2_item = match v2.get(i) {
                        Some(v2_item) => v2_item,
                        None => return Some(Ordering::Greater),
                    };
                    match v1_item.partial_cmp(v2_item)? {
                        Ordering::Equal => continue,
                        result => return Some(result),
                    }
                }
                if v1.len() == v2.len() {
                    Ordering::Equal
                } else {
                    Ordering::Less
                }
            }
            _ => self.to_array().partial_cmp(&other.to_array())?,
        })
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{n:?}"),
            Self::Array(array) => {
                let array = array
                    .iter()
                    .map(|v| format!("{v:?}"))
                    .collect::<Vec<_>>()
                    .join(",");
                write!(f, "[{}]", array)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_example() {
        let input = read_to_string("./example.txt").unwrap();

        let result = solve_problem(&input).unwrap();
        assert_eq!(result, 13);
    }
}
