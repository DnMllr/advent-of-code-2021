// TODO(Dan): This file needs some code cleanup
use std::{
    collections::{BinaryHeap, HashMap},
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

use clap::Parser;

#[derive(Debug, Parser)]
/// Implements a solution to the 15th day of Advent of Code 2021.
struct Options {
    #[clap()]
    /// Path to the file that contains the input.
    input: PathBuf,

    #[clap(long)]
    /// whether to run part 2 or not
    part_2: bool,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let opts = Options::parse();

    let numbers = read_grid(&opts.input)?;
    let answer = a_star(&numbers, opts.part_2);

    println!(
        "{} {:?}",
        if opts.part_2 { "part 2" } else { "part 1" },
        answer
    );

    Ok(())
}

fn read_grid(path: &Path) -> Result<Vec<Vec<u32>>, std::io::Error> {
    BufReader::new(File::open(path)?)
        .lines()
        .map(|r| {
            r.map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).expect("characters must be valid digits"))
                    .collect()
            })
        })
        .collect()
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn distance(&self, other: &Self) -> usize {
        (self.x.max(other.x) - self.x.min(other.x)) + (self.y.max(other.y) - self.y.min(other.y))
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Node {
    position: Position,
    priority: usize,
}

impl Node {
    pub const fn start() -> Self {
        Node {
            position: Position { x: 0, y: 0 },
            priority: 0,
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.priority.cmp(&self.priority)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn a_star(grid: &[Vec<u32>], part_2: bool) -> Option<usize> {
    let end = find_end(grid, part_2);
    let mut frontier = BinaryHeap::new();
    let mut cost_so_far = HashMap::new();

    cost_so_far.insert(Node::start().position, 0);

    frontier.push(Node::start());

    while let Some(current) = frontier.pop() {
        if current.position == end {
            return cost_so_far.get(&end).copied();
        }

        let current_cost = *cost_so_far.get(&current.position).unwrap();

        for neighbor in neighbors(&current.position, grid, part_2) {
            let new_cost = current_cost + cost(&neighbor, grid, part_2);
            if !cost_so_far.contains_key(&neighbor)
                || new_cost < *cost_so_far.get(&neighbor).unwrap()
            {
                cost_so_far.insert(neighbor.clone(), new_cost);
                frontier.push(Node {
                    position: neighbor.clone(),
                    priority: new_cost + neighbor.distance(&end),
                });
            }
        }
    }

    None
}

fn cost(position: &Position, grid: &[Vec<u32>], part_2: bool) -> usize {
    if part_2 {
        let part_1_end = find_end(grid, false);
        let x = position.x % (part_1_end.x + 1);
        let y = position.y % (part_1_end.y + 1);
        let offset_x = position.x / (part_1_end.x + 1);
        let offset_y = position.y / (part_1_end.y + 1);
        let part_1_cost = cost(&Position { x, y }, grid, false);

        ((part_1_cost + offset_x + offset_y - 1) % 9) + 1
    } else {
        grid[position.y][position.x] as usize
    }
}

fn neighbors(node: &Position, grid: &[Vec<u32>], part_2: bool) -> impl Iterator<Item = Position> {
    let multiplier = if part_2 { 5 } else { 1 };
    let mut min_x = node.x.checked_sub(1).map(|x| Position { x, ..*node });
    let mut max_x = (node.x + 1 < (grid[0].len() * multiplier))
        .then(|| node.x + 1)
        .map(|x| Position { x, ..*node });
    let mut min_y = node.y.checked_sub(1).map(|y| Position { y, ..*node });
    let mut max_y = (node.y + 1 < (grid.len() * multiplier))
        .then(|| node.y + 1)
        .map(|y| Position { y, ..*node });

    std::iter::from_fn(move || {
        min_x
            .take()
            .or_else(|| max_x.take())
            .or_else(|| min_y.take())
            .or_else(|| max_y.take())
    })
}

fn find_end(grid: &[Vec<u32>], part_2: bool) -> Position {
    let multiplier = if part_2 { 5 } else { 1 };
    Position {
        y: multiplier * grid.len() - 1,
        x: multiplier * grid[0].len() - 1,
    }
}
