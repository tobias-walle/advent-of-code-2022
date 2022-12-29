use std::{
    any::Any,
    collections::{HashMap, HashSet},
    thread,
    time::Duration,
};

use colored::{ColoredString, Colorize};
use eyre::{bail, Context, ContextCompat, Result};
use utils::{is_debugging, read_input_file_as_string};

fn main() -> Result<()> {
    let input = read_input_file_as_string().context("Cannot read input")?;

    let result = solve_problem(&input, 1_000_000_000_000).context("Failed to solve problem")?;
    println!("{result}");
    Ok(())
}

fn solve_problem(input: &str, settled_pieces_until_end: usize) -> Result<i64> {
    let mut game = parse(input, settled_pieces_until_end)?;
    game.render();
    while game.tick() {
        game.render();
    }
    game.render();
    Ok(game.height)
}

fn parse(input: &str, settled_pieces_until_end: usize) -> Result<Game> {
    let mut directions = vec![];
    for c in input.trim().chars() {
        let direction = match c {
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => bail!("Invalid direction: {c}"),
        };
        directions.push(direction);
    }
    #[rustfmt::skip]
    let piece_types = vec![
        Piece::new(PieceTypeId::A, [
          [1, 1, 1, 1],
        ]),
        Piece::new(PieceTypeId::B, [
          [0, 1, 0],
          [1, 1, 1],
          [0, 1, 0],
        ]),
        Piece::new(PieceTypeId::C, [
          [0, 0, 1],
          [0, 0, 1],
          [1, 1, 1],
        ]),
        Piece::new(PieceTypeId::D, [
          [1],
          [1],
          [1],
          [1],
        ]),
        Piece::new(PieceTypeId::E, [
          [1, 1],
          [1, 1],
        ]),
    ];
    let mut game = Game {
        settled_pieces_until_end,
        width: 7,
        directions,
        next_direction_index: 0,
        last_direction: None,
        falling: piece_types[0].clone(),
        piece_types,
        next_piece_type_index: 1,
        settled: HashMap::new(),
        settled_pieces: Vec::new(),
        height: 0,
        settled_pieces_count: 0,
    };
    game.falling = game.place_new_falling_piece(&game.falling);
    Ok(game)
}

#[derive(Debug, Clone)]
struct Game {
    pub settled_pieces_until_end: usize,
    pub width: i64,
    pub height: i64,
    pub piece_types: Vec<Piece>,
    pub next_piece_type_index: usize,
    pub directions: Vec<Direction>,
    pub last_direction: Option<Direction>,
    pub next_direction_index: usize,
    pub settled: HashMap<Point, PieceTypeId>,
    pub settled_pieces: Vec<SettledPiece>,
    pub settled_pieces_count: usize,
    pub falling: Piece,
}

impl Game {
    fn tick(&mut self) -> bool {
        if self.settled_pieces_count == 10_000 {
            let pattern = self.find_repeating_pattern().expect("No pattern found");
            println!(
                "Apply repeating pattern with {} pieces.",
                pattern.settled_pieces.len()
            );
            self.apply_pattern_to_speedup_simulation(&pattern);
            println!(
                "After: count={} height={}",
                self.settled_pieces_count, self.height
            );
        }

        let is_settled = self.try_moves().is_err();

        if is_settled {
            self.settled_pieces_count += 1;
            let mut new_height = self.height;
            for point in &self.falling.points {
                new_height = new_height.max(point.y + 1);
                self.settled.insert(*point, self.falling.type_id);
            }
            let settled_piece = SettledPiece {
                type_id: self.falling.type_id,
                piece_type_index: self.current_piece_type_index(),
                direction_index: self.current_direction_index(),
                position_x: self.falling.position_x(),
                height_diff: new_height - self.height,
            };
            self.settled_pieces.push(settled_piece);
            self.height = new_height;

            let new_piece_type = self.piece_types[self.next_piece_type_index].clone();
            self.next_piece_type_index = (self.next_piece_type_index + 1) % self.piece_types.len();

            self.falling = self.place_new_falling_piece(&new_piece_type);
        }

        self.settled_pieces_count < self.settled_pieces_until_end
    }

    fn current_direction_index(&self) -> usize {
        self.next_direction_index
            .checked_sub(1)
            .unwrap_or(self.directions.len())
    }

    fn current_piece_type_index(&self) -> usize {
        self.next_piece_type_index
            .checked_sub(1)
            .unwrap_or(self.piece_types.len())
    }

    fn try_moves(&mut self) -> Result<()> {
        let direction = &self.directions[self.next_direction_index];
        self.next_direction_index = (self.next_direction_index + 1) % self.directions.len();

        self.last_direction = None;
        self.falling = self
            .falling
            .shift(self, direction)
            .context("Failed to move {direction:?}")?;
        self.last_direction = Some(direction.clone());

        self.falling = self
            .falling
            .shift(self, &Direction::Down)
            .context("Failed to move down")?;

        Ok(())
    }

    fn place_new_falling_piece(&self, piece_type: &Piece) -> Piece {
        let mut piece = piece_type.clone();
        piece.points = piece
            .points
            .iter()
            .map(|p| Point {
                x: p.x + 2,
                y: p.y + self.height + 3,
            })
            .collect();
        piece
    }

    fn find_repeating_pattern(&self) -> Option<Pattern> {
        let mut pattern = Vec::new();
        let last_piece = self.settled_pieces.last()?;
        let last_piece_index = self.settled_pieces.len() - 1;
        let mut expected_piece_index = last_piece_index;
        for piece in self.settled_pieces.iter().skip(1).rev() {
            if piece == last_piece && !pattern.is_empty() {
                break;
            }
            let Some(expected_piece) = self.settled_pieces.get(expected_piece_index) else {
                return None;
            };
            if piece == expected_piece {
                pattern.push(expected_piece);
                expected_piece_index -= 1;
            } else {
                pattern.clear();
                expected_piece_index = last_piece_index;
            }
        }
        if pattern.is_empty() {
            None
        } else {
            Some(Pattern {
                height_diff: pattern.iter().map(|piece| piece.height_diff).sum(),
                settled_pieces: pattern.into_iter().rev().cloned().collect(),
            })
        }
    }

    fn apply_pattern_to_speedup_simulation(&mut self, pattern: &Pattern) {
        let pieces_left = self.settled_pieces_until_end - self.settled_pieces_count;
        let times_to_apply_pattern = pieces_left / pattern.settled_pieces.len();
        let height_diff = times_to_apply_pattern as i64 * pattern.height_diff;

        self.settled_pieces_count += times_to_apply_pattern * pattern.settled_pieces.len();
        self.height += height_diff;
        self.falling.points = self
            .falling
            .points
            .iter()
            .map(|point| Point {
                y: point.y + height_diff,
                ..*point
            })
            .collect();
        self.settled = self
            .settled
            .iter()
            .map(|(point, piece)| {
                (
                    Point {
                        y: point.y + height_diff,
                        ..*point
                    },
                    *piece,
                )
            })
            .collect();
    }

    fn render(&self) {
        if !is_debugging() {
            return;
        }
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!(
            "C={} H={} D={:?}",
            self.settled_pieces_count, self.height, self.last_direction
        );
        let render_window = 40;
        let render_end = (self.height + 10).max(render_window);
        let render_start = render_end - render_window;
        for y in (render_start..=render_end).rev() {
            for x in 0..self.width {
                let p = Point { x, y };
                let pixel = if self.falling.points.contains(&p) {
                    self.falling.type_id.pixel()
                } else if let Some(type_id) = self.settled.get(&p) {
                    type_id.pixel()
                } else {
                    "__".truecolor(50, 50, 50)
                };
                print!("{}", pixel);
            }
            println!();
        }
        thread::sleep(Duration::from_millis(1000 / 24));
    }
}

struct Pattern {
    settled_pieces: Vec<SettledPiece>,
    height_diff: i64,
}

#[derive(Debug, Clone)]
enum Direction {
    Right,
    Left,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PieceTypeId {
    A,
    B,
    C,
    D,
    E,
}

impl PieceTypeId {
    fn pixel(&self) -> ColoredString {
        match *self {
            PieceTypeId::A => "██".red(),
            PieceTypeId::B => "██".blue(),
            PieceTypeId::C => "██".green(),
            PieceTypeId::D => "██".yellow(),
            PieceTypeId::E => "██".cyan(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SettledPiece {
    type_id: PieceTypeId,
    direction_index: usize,
    position_x: i64,
    height_diff: i64,
    piece_type_index: usize,
}

#[derive(Debug, Clone)]
struct Piece {
    type_id: PieceTypeId,
    points: HashSet<Point>,
}

impl Piece {
    fn new<const W: usize, const H: usize>(type_id: PieceTypeId, shape: [[u8; W]; H]) -> Self {
        let mut points = HashSet::new();
        for (y, line) in shape.iter().enumerate() {
            for (x, cell) in line.iter().enumerate() {
                assert!(matches!(*cell, 0 | 1));
                if *cell == 1 {
                    points.insert(Point::new(x as i64, (H - 1 - y) as i64));
                }
            }
        }
        Self { type_id, points }
    }

    pub fn shift(&self, game: &Game, direction: &Direction) -> Option<Piece> {
        let points: Option<HashSet<Point>> = self
            .points
            .iter()
            .map(|p| p.shift(game, direction))
            .collect();

        let piece = match points {
            Some(points) => Piece {
                type_id: self.type_id,
                points,
            },
            None => self.clone(),
        };

        if piece.is_possible(game) {
            None
        } else {
            Some(piece)
        }
    }

    fn is_possible(&self, game: &Game) -> bool {
        for point in &self.points {
            if point.y < 0 || game.settled.get(point).is_some() {
                return true;
            };
        }
        false
    }

    fn position_x(&self) -> i64 {
        self.points.iter().map(|point| point.x).min().unwrap_or(0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn shift(&self, game: &Game, direction: &Direction) -> Option<Point> {
        let point = match direction {
            Direction::Right => {
                if self.x == game.width - 1 {
                    return None;
                } else {
                    Point {
                        x: self.x + 1,
                        ..*self
                    }
                }
            }
            Direction::Left => {
                if self.x == 0 {
                    return None;
                } else {
                    Point {
                        x: self.x - 1,
                        ..*self
                    }
                }
            }
            Direction::Down => Point {
                y: self.y - 1,
                ..*self
            },
        };
        if matches!(direction, Direction::Left | Direction::Right)
            && game.settled.contains_key(&point)
        {
            None
        } else {
            Some(point)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_example_with_3() {
        let input = read_to_string("./example.txt").unwrap();

        let result = solve_problem(&input, 3).unwrap();
        assert_eq!(result, 6);
    }

    #[test]
    fn test_example_with_11_000() {
        let input = read_to_string("./example.txt").unwrap();

        let result = solve_problem(&input, 11_000).unwrap();
        assert_eq!(result, 16661);
    }

    #[test]
    fn test_example_with_1_trillion() {
        let input = read_to_string("./example.txt").unwrap();

        let result = solve_problem(&input, 1_000_000_000_000).unwrap();
        assert_eq!(result, 1514285714288);
    }
}
