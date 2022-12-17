use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
    fmt::Debug,
};

use colored::Colorize;
use eyre::{bail, Context, ContextCompat, Result};
use utils::read_input_file_as_string;

use priority_queue::PriorityQueue;

fn main() -> Result<()> {
    let input = read_input_file_as_string().context("Cannot read input")?;

    let result = solve_problem(&input).context("Failed to solve problem")?;
    println!("{result}");
    Ok(())
}

fn solve_problem(input: &str) -> Result<i32> {
    let terrain = parse(input)?;
    let mut best_path_terrain = None;
    let mut best_path = None;
    let mut best_path_length = i32::MAX;
    for point in terrain.points.iter().flatten() {
        if point.height == MIN_HEIGHT {
            let terrain = Terrain {
                start_position: point.position,
                ..terrain.clone()
            };
            let Ok(path) = find_best_path(&terrain) else {
                continue;
            };
            let length = path.len() as i32 - 1;
            if length < best_path_length {
                best_path_length = length;
                best_path_terrain = Some(terrain);
                best_path = Some(path);
            }
        }
    }
    let best_path = best_path.context("No best path found")?;
    println!("{}", render(&best_path_terrain.unwrap(), &best_path));
    Ok(best_path_length)
}

fn find_best_path(terrain: &Terrain) -> Result<Path> {
    let mut queue = PriorityQueue::new();
    let mut came_from: HashMap<Coordinate, Point> = HashMap::new();

    let start = terrain.get(terrain.start_position).unwrap();
    let target = terrain.get(terrain.target_position).unwrap();
    queue.push(start.clone(), Reverse(0));

    let mut costs_so_far = HashMap::new();
    costs_so_far.insert(start.position, 0);

    while !queue.is_empty() {
        let (mut current, _) = queue.pop().unwrap();
        if current.position == target.position {
            let mut path = Vec::from([current.position]);
            while let Some(previous) = came_from.get(&current.position) {
                current = previous.clone();
                path.push(current.position);
            }
            return Ok(path);
        }

        for neighbour in terrain.reachable_neighbours(current.position) {
            let costs = neighbour.position.distance(&current.position) as i32;
            let tentative_score = costs_so_far[&current.position] + costs;
            let neighbour_score = *costs_so_far.get(&neighbour.position).unwrap_or(&i32::MAX);
            if tentative_score < neighbour_score {
                came_from.insert(neighbour.position, current.clone());
                costs_so_far.insert(neighbour.position, tentative_score);
                let priority = tentative_score + cost_heuristic(&neighbour, target);
                queue.push(neighbour.clone(), Reverse(priority));
            }
        }
    }

    bail!("No path found!")
}

fn cost_heuristic(p_from: &Point, p_to: &Point) -> i32 {
    let distance =
        p_to.position.x.abs_diff(p_from.position.x) + p_to.position.y.abs_diff(p_from.position.y);
    distance as i32
}

fn parse(input: &str) -> Result<Terrain> {
    let input = input.trim();
    let mut points = Vec::new();

    let mut start_position: Option<Coordinate> = None;
    let mut target_position: Option<Coordinate> = None;
    for (y, line) in input.lines().enumerate() {
        let mut items = Vec::new();
        for (x, char) in line.chars().enumerate() {
            let height = match char {
                'S' => {
                    start_position = Some((x, y).into());
                    MIN_HEIGHT
                }
                'E' => {
                    target_position = Some((x, y).into());
                    MAX_HEIGHT
                }
                _ => char as u8 - b'a',
            };
            items.push(Point {
                position: (x, y).into(),
                height,
            });
        }
        points.push(items);
    }

    let start_position = start_position.context("Start position not found")?;
    Ok(Terrain {
        start_position,
        target_position: target_position.context("Target position not found")?,
        width: points[0].len(),
        height: points.len(),
        points,
    })
}

fn render(terrain: &Terrain, path: &Path) -> String {
    let points_in_path: HashSet<_> = path.clone().into_iter().collect();
    let mut output = String::new();
    for y in 0..terrain.height {
        for x in 0..terrain.width {
            let point = terrain.get((x, y)).unwrap();
            let brightness = 55 + ((point.height as f32 / MAX_HEIGHT as f32) * 200_f32) as u8;
            let pixel = "█";
            let pixel = if point.position == terrain.start_position {
                pixel.truecolor(0, brightness * 2, brightness)
            } else if point.position == terrain.target_position {
                pixel.truecolor(brightness / 2, 0, brightness)
            } else if points_in_path.contains(&point.position) {
                pixel.truecolor(0, brightness / 2, brightness)
            } else {
                pixel.truecolor(brightness, brightness, brightness)
            };
            output += &format!("{}", pixel);
        }
        output += "\n";
    }
    output
}

const MIN_HEIGHT: u8 = 0;
const MAX_HEIGHT: u8 = b'z' - b'a';

#[derive(Debug, Clone, PartialEq, Eq)]
struct Terrain {
    width: usize,
    height: usize,
    points: Vec<Vec<Point>>,
    start_position: Coordinate,
    target_position: Coordinate,
}

impl Terrain {
    pub fn get(&self, coordinate: impl Into<Coordinate>) -> Option<&Point> {
        let coordinate = coordinate.into();
        self.points.get(coordinate.y)?.get(coordinate.x)
    }

    pub fn get_option(&self, coordinate: Option<impl Into<Coordinate>>) -> Option<&Point> {
        let coordinate = coordinate?.into();
        self.get(coordinate)
    }

    pub fn reachable_neighbours(&self, coordinate: impl Into<Coordinate>) -> Vec<Point> {
        let coordinate = coordinate.into();
        let current = self.get(coordinate).unwrap();
        self.neighbours(coordinate)
            .into_iter()
            .filter(|neighbour| neighbour.height <= current.height + 1)
            .collect()
    }

    pub fn neighbours(&self, coordinate: impl Into<Coordinate>) -> Vec<Point> {
        let coordinate = coordinate.into();
        Vec::from([
            self.get_option(coordinate.down()),
            self.get_option(coordinate.right()),
            self.get_option(coordinate.left()),
            self.get_option(coordinate.up()),
        ])
        .into_iter()
        .flatten()
        .cloned()
        .collect()
    }
}

type Path = Vec<Coordinate>;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn distance(&self, other: &Coordinate) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl Debug for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Coordinate {
    fn up(&self) -> Option<Self> {
        if self.y == 0 {
            return None;
        }
        Some(Self {
            y: self.y - 1,
            ..*self
        })
    }

    fn down(&self) -> Option<Self> {
        if self.y == usize::MAX {
            return None;
        }
        Some(Self {
            y: self.y + 1,
            ..*self
        })
    }

    fn left(&self) -> Option<Self> {
        if self.x == 0 {
            return None;
        }
        Some(Self {
            x: self.x - 1,
            ..*self
        })
    }

    fn right(&self) -> Option<Self> {
        if self.x == usize::MAX {
            return None;
        }
        Some(Self {
            x: self.x + 1,
            ..*self
        })
    }
}

impl From<(usize, usize)> for Coordinate {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    position: Coordinate,
    height: u8,
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_example() {
        let input = read_to_string("./example.txt").unwrap();

        let result = solve_problem(&input).unwrap();
        assert_eq!(result, 31);
    }
}
