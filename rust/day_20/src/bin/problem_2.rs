use colored::Colorize;
use eyre::{Context, Result};

use utils::read_input_file_as_string;

fn main() -> Result<()> {
    let input = read_input_file_as_string().context("Cannot read input")?;

    let result = solve_problem(&input).context("Failed to solve problem")?;
    println!("{result}");
    Ok(())
}

type Number = i64;

fn solve_problem(input: &str) -> Result<Number> {
    let input = parse(input);
    Ok(decrypt_and_get_coordinates(&input))
}

fn decrypt_and_get_coordinates(input: &[Number]) -> Number {
    let result = perform_moves(input);
    get_coordinates(&result)
}

const DECRYPT_KEY: Number = 811589153;

fn perform_moves(input: &[Number]) -> Vec<Number> {
    let input: Vec<Number> = input.iter().map(|v| v * DECRYPT_KEY).collect();
    let mut result: Vec<_> = input.iter().enumerate().collect();
    let len = input.len() as Number;
    let max = len - 1;
    for _ in 0..10 {
        for (input_index, n) in input.iter().enumerate() {
            let index = result
                .iter()
                .position(|(i, _)| input_index == *i)
                .expect("n is always in result");

            let mut new_index = index as Number + n;
            if new_index > max {
                new_index %= max;
            }
            if new_index < 0 {
                new_index = -(-new_index % max);
                new_index += max;
            }
            let new_index = usize::try_from(new_index).expect("new_index to fit in usize");

            result.remove(index);
            result.insert(new_index, (input_index, n));
            if input.len() <= 10 {
                println!(
                    "{n:>2}: {index} -> {new_index} | {} | {max}",
                    format_vec(
                        &result.iter().map(|v| *v.1).collect::<Vec<_>>(),
                        index,
                        new_index
                    )
                );
            }
        }
    }
    let result: Vec<Number> = result.iter().map(|v| *v.1).collect();
    normalize(&result)
}

fn normalize(result: &[Number]) -> Vec<Number> {
    let zero_position = result
        .iter()
        .position(|v| *v == 0)
        .expect("0 to be in result");
    let mut normalized = result[zero_position..].to_vec();
    normalized.extend(&result[..zero_position]);
    normalized
}

fn get_coordinates(result: &[Number]) -> Number {
    let a = get_with_wrapping(result, 1000);
    let b = get_with_wrapping(result, 2000);
    let c = get_with_wrapping(result, 3000);
    dbg!(a, b, c);
    a + b + c
}

fn get_with_wrapping(list: &[Number], i: usize) -> Number {
    list[i % list.len()]
}

fn parse(input: &str) -> Vec<Number> {
    input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().expect("String to be number"))
        .collect()
}

fn format_vec(list: &[Number], from_index: usize, to_index: usize) -> String {
    let content = list
        .iter()
        .enumerate()
        .map(|(i, n)| {
            if i == from_index && i == to_index {
                n.to_string().magenta().to_string()
            } else if i == from_index {
                n.to_string().bright_red().to_string()
            } else if i == to_index {
                n.to_string().cyan().to_string()
            } else {
                n.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(", ");
    format!("[{content}]")
}

#[cfg(test)]
mod tests {
    // use pretty_assertions::assert_eq;
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_example() {
        let input = read_to_string("./example.txt").unwrap();

        let result = solve_problem(&input).unwrap();
        assert_eq!(result, 1623178306);
    }
}
