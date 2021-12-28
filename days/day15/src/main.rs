use std::path::PathBuf;

use clap::Parser;

use crate::{
    astar::a_star,
    grid::{Base, Multiplier},
};

mod astar;
mod grid;
mod search;

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

type Part1 = Base;
type Part2 = Multiplier<Base, 5>;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let opts = Options::parse();

    let numbers = grid::read_grid(&opts.input)?;
    let answer = if opts.part_2 {
        a_star::<Part2>(&numbers)
    } else {
        a_star::<Part1>(&numbers)
    };

    println!(
        "{} {:?}",
        if opts.part_2 { "part 2" } else { "part 1" },
        answer
    );

    Ok(())
}
