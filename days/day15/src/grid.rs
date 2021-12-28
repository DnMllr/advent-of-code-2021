use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

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
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn distance(&self, other: &Self) -> usize {
        (self.x.max(other.x) - self.x.min(other.x)) + (self.y.max(other.y) - self.y.min(other.y))
    }

    pub fn lookup(&self, grid: GridRef) -> Option<usize> {
        grid.get(self.y)
            .and_then(|r| r.get(self.x).map(|x| *x as usize))
    }
}
