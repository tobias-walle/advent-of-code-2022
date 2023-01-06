use colored::Colorize;
use eyre::{Context, Result};

use utils::read_input_file_as_string;

fn main() -> Result<()> {
    let input = read_input_file_as_string().context("Cannot read input")?;

    let result = solve_problem(&input).context("Failed to solve problem")?;
    println!("{result}");
    Ok(())
}

fn solve_problem(input: &str) -> Result<i32> {
    let input = parse(input);
    Ok(decrypt_and_get_coordinates(&input))
}

fn decrypt_and_get_coordinates(input: &[i32]) -> i32 {
    let result = perform_moves(input);
    get_coordinates(&result)
}

fn perform_moves(input: &[i32]) -> Vec<i32> {
    let mut result: Vec<_> = input.iter().enumerate().collect();
    let len = input.len() as i32;
    let max = len - 1;
    for (input_index, n) in input.iter().enumerate() {
        let index = result
            .iter()
            .position(|(i, _)| input_index == *i)
            .expect("n is always in result");

        let mut new_index = index as i32 + n;
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
    let result: Vec<_> = result.iter().map(|v| *v.1).collect();
    normalize(&result)
}

fn normalize(result: &[i32]) -> Vec<i32> {
    let zero_position = result
        .iter()
        .position(|v| *v == 0)
        .expect("0 to be in result");
    let mut normalized = result[zero_position..].to_vec();
    normalized.extend(&result[..zero_position]);
    normalized
}

fn get_coordinates(result: &[i32]) -> i32 {
    let a = get_with_wrapping(result, 1000);
    let b = get_with_wrapping(result, 2000);
    let c = get_with_wrapping(result, 3000);
    dbg!(a, b, c);
    a + b + c
}

fn get_with_wrapping(list: &[i32], i: usize) -> i32 {
    list[i % list.len()]
}

fn parse(input: &str) -> Vec<i32> {
    input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().expect("String to be number"))
        .collect()
}

fn format_vec(list: &[i32], from_index: usize, to_index: usize) -> String {
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
        assert_eq!(result, 3);
    }

    #[test]
    fn test_decrypt_and_get_coordinates() {
        let result = decrypt_and_get_coordinates(&[3, 1, 0]);

        assert_eq!(result, 4);
    }

    #[test]
    fn test_input() {
        let input = read_to_string("./input.txt").unwrap();

        let result = solve_problem(&input).unwrap();

        dbg!(result);
        assert!(result < 2659);
    }

    #[test]
    fn test_normalize() {
        assert_eq!(normalize(&[0, -6, 5, 1]), vec![0, -6, 5, 1]);
        assert_eq!(normalize(&[-6, 5, 1, 0]), vec![0, -6, 5, 1]);
        assert_eq!(normalize(&[5, 1, 0, -6]), vec![0, -6, 5, 1]);
        assert_eq!(normalize(&[1, 0, -6, 5]), vec![0, -6, 5, 1]);
    }

    #[test]
    fn test_perform_moves_example() {
        let input = read_to_string("./example.txt").unwrap();
        let input = parse(&input);

        let result = perform_moves(&input);

        assert_eq!(result, vec![0, 3, -2, 1, 2, -3, 4]);
    }

    #[test]
    fn test_perform_moves_2() {
        let result = perform_moves(&[0, 1, -6, 5]);

        assert_eq!(result, vec![0, -6, 5, 1]);
    }

    #[test]
    fn test_perform_moves_3() {
        let result = perform_moves(&[7, 3, -10, 0]);

        assert_eq!(result, vec![0, 3, -10, 7]);
    }

    #[test]
    fn test_perform_moves_4() {
        let result = perform_moves(&[3, 1, 0]);

        assert_eq!(result, vec![0, 3, 1]);
    }

    #[test]
    fn test_perform_moves_5() {
        let result = perform_moves(&[0, -1, -1, 1]);

        assert_eq!(result, vec![0, -1, 1, -1]);
    }
}
