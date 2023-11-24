use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

use eyre::{Context, Result};
use utils::read_input_file_as_string;

fn main() -> Result<()> {
    let input = read_input_file_as_string().context("Cannot read input")?;

    let result = solve_problem(&input).context("Failed to solve problem")?;
    println!("{result}");
    Ok(())
}

fn solve_problem(input: &str) -> Result<usize> {
    let problem = Problem::parse(input);
    Ok(0)
}

#[derive(Debug, Clone)]
struct Problem {
    me: Me,
    fields: HashMap<Vec2, Field>,
    actions: Vec<Action>,
}

impl Problem {
    fn perform_next_action(&mut self) -> bool {
        let Some(action) = self.actions.pop() else {
            return false;
        };
        todo!("Move");
        true
    }

    fn parse(input: &str) -> Problem {
        let mut fields = HashMap::new();
        let lines: Vec<_> = input.lines().collect();
        let mut left_most = None;
        for (y, line) in lines.iter().enumerate() {
            if line.is_empty() {
                break;
            }
            for (x, char) in line.chars().enumerate() {
                let pos = Vec2 { x, y };
                if x == 0 {
                    left_most = Some(pos.clone());
                }
                let field = match char {
                    '.' => Field::Free,
                    '#' => Field::Wall,
                    ' ' => continue,
                    char => panic!("Unexpected char '{char}'"),
                };
                fields.insert(pos, field);
            }
        }
        let left_most = left_most.expect("Leftmost position not found");

        let last_line = lines[lines.len() - 1];
        let actions: Vec<_> = last_line
            .chars()
            .map(|c| match c {
                'L' => Action::Rotation(Rotation::Left),
                'R' => Action::Rotation(Rotation::Left),
                n => Action::Move(n.to_digit(10).expect("Valid digit")),
            })
            .collect();

        let me = Me {
            position: left_most,
            facing: Direction::Right,
        };

        Problem {
            me,
            fields,
            actions,
        }
    }
}

#[derive(Debug, Clone)]
struct Me {
    facing: Direction,
    position: Vec2,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Field {
    Free,
    Wall,
    Void,
}

#[derive(Debug, Clone)]
enum Action {
    Move(u32),
    Rotation(Rotation),
}

#[derive(Debug, Clone)]
struct Bound {
    row_range: RangeInclusive<usize>,
    col_range: RangeInclusive<usize>,
}

#[derive(Debug, Clone)]
enum Rotation {
    Right,
    Left,
}

#[derive(Debug, Clone)]
enum Direction {
    Top,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Vec2 {
    x: usize,
    y: usize,
}

impl Vec2 {
    fn shift(&self, direction: &Direction) -> Vec2 {
        let Vec2 { mut x, mut y } = self.clone();
        match direction {
            Direction::Top => y -= 1,
            Direction::Right => x += 1,
            Direction::Down => y += 1,
            Direction::Left => x -= 1,
        };
        Vec2 { x, y }
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
        assert_eq!(result, 0);
    }
}
