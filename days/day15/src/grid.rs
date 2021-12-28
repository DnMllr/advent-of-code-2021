use std::{
    fs::File,
    io::{BufRead, BufReader},
    marker::PhantomData,
    path::Path,
};

use crate::astar::GridStrategy;

pub type Grid = Vec<Vec<usize>>;
pub type GridRef<'a> = &'a [Vec<usize>];

pub fn read_grid(path: &Path) -> Result<Grid, std::io::Error> {
    BufReader::new(File::open(path)?)
        .lines()
        .map(|r| {
            r.map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).expect("characters must be valid digits") as usize)
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
    pub const ZERO: Self = Self { x: 0, y: 0 };

    pub fn bounds(grid: GridRef) -> Self {
        Self {
            y: grid.len(),
            x: grid
                .first()
                .expect("there must be at least one row in the grid")
                .len(),
        }
    }

    pub fn distance(&self, other: &Self) -> usize {
        (self.x.max(other.x) - self.x.min(other.x)) + (self.y.max(other.y) - self.y.min(other.y))
    }

    pub fn lookup(&self, grid: GridRef) -> Option<usize> {
        grid.get(self.y)
            .and_then(|r| r.get(self.x).map(|x| *x as usize))
    }

    pub fn neighbors(&self, bounds: &Self) -> impl Iterator<Item = Position> {
        std::iter::empty()
            .chain(self.x.checked_sub(1).map(|x| Position { x, ..*self }))
            .chain(self.y.checked_sub(1).map(|y| Position { y, ..*self }))
            .chain(
                (self.x + 1 < bounds.x)
                    .then(|| self.x + 1)
                    .map(|x| Position { x, ..*self }),
            )
            .chain(
                (self.y + 1 < bounds.y)
                    .then(|| self.y + 1)
                    .map(|y| Position { y, ..*self }),
            )
    }
}

impl std::ops::Sub<usize> for Position {
    type Output = Self;

    fn sub(self, rhs: usize) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl std::ops::Mul<usize> for Position {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Base;

impl GridStrategy for Base {
    fn cost(position: &Position, grid: GridRef) -> usize {
        position.lookup(grid).expect("position must be in range")
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Multiplier<P: GridStrategy, const M: usize> {
    inner: PhantomData<P>,
}

impl<P: GridStrategy, const M: usize> GridStrategy for Multiplier<P, M> {
    fn cost(position: &Position, grid: GridRef) -> usize {
        let p_end = P::end(grid);
        let x = position.x % (p_end.x + 1);
        let y = position.y % (p_end.y + 1);
        let offset_x = position.x / (p_end.x + 1);
        let offset_y = position.y / (p_end.y + 1);
        let part_1_cost = P::cost(&Position { x, y }, grid);

        ((part_1_cost + offset_x + offset_y - 1) % 9) + 1
    }

    fn bounds(grid: GridRef) -> Position {
        Base::bounds(grid) * M
    }
}
