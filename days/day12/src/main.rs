use std::{fs::File, io::Read, path::PathBuf};

use clap::Parser;
use pathfinding::count_all_paths;

use crate::pathfinding::{Part1, Part2};

mod parser;
mod pathfinding;

#[derive(Debug, Parser)]
/// Implements a solution to the 12th day of Advent of Code 2021.
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

    let input = parser::Parser::parse(&buf).expect("advent of code input is always valid");

    if opts.part_2 {
        println!("part 2 {}", count_all_paths(&input, Part2));
    } else {
        println!("part 1 {}", count_all_paths(&input, Part1));
    }

    Ok(())
}
