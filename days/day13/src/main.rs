use std::{fs::File, io::Read, path::PathBuf};

use clap::Parser;
use parser::parse;

mod model;
mod parser;

#[derive(Debug, Parser)]
/// Implements a solution to the 13th day of Advent of Code 2021.
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

    let mut input = parse(&buf).expect("advent of code input is always valid");

    if opts.part_2 {
        input.fold_all();
        println!("part 2\n{}", input);
    } else {
        println!("part 1 {}", input.fold().unwrap());
    }

    Ok(())
}
