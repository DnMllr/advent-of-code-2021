use std::{fs::File, io::Read, path::PathBuf};

use clap::Parser;
use grid::Grid;

mod grid;

#[derive(Debug, Parser)]
/// Implements a solution to the 10th day of Advent of Code 2021.
struct Options {
    #[clap()]
    /// Path to the file that contains the input.
    input: PathBuf,

    #[clap(long)]
    /// whether or not to run part 2
    part_2: bool,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let opts = Options::parse();
    let mut buf = String::new();

    File::open(&opts.input)?.read_to_string(&mut buf)?;

    let mut grid: Grid = buf.parse()?;

    if opts.part_2 {
        println!("part 2 {}", grid.first_synchronization());
    } else {
        println!("part 1 {}", grid.step_n(100));
    }

    Ok(())
}
