use std::{fs::File, io::Read, path::PathBuf};

use clap::Parser;
use grid::Grid;

mod grid;
mod iter;
mod location;

#[derive(Debug, Parser)]
/// Implements a solution to the 9th day of Advent of Code 2021.
struct Options {
    #[clap()]
    /// Path to the file that contains the input.
    input: PathBuf,

    #[clap(long)]
    /// Whether or not to run part_2
    part_2: bool,
}

impl Options {
    pub fn part_1(&self) -> bool {
        !self.part_2
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let opts = Options::parse();
    let mut buf = String::new();

    File::open(&opts.input)?.read_to_string(&mut buf)?;

    let grid = buf.parse()?;

    if opts.part_1() {
        println!("part 1: {}", part_1(grid));
    } else {
        println!("part 2: {}", part_2(grid));
    }

    Ok(())
}

fn part_1(grid: Grid) -> usize {
    grid.locations()
        .filter(|l| l.is_low_point())
        .map(|l| l.risk() as usize)
        .sum()
}

fn part_2(grid: Grid) -> usize {
    let mut highest = [0; 4];

    for size in grid.basins().map(|s| s.count()) {
        *highest.first_mut().unwrap() = size;
        highest.sort_unstable();
    }

    let top_three = &highest[1..];

    top_three.iter().product()
}
