use std::{fs::File, io::Read, path::PathBuf};

use clap::Parser;
use parser::parse;
use replacer::Replacer;

mod parser;
mod replacer;

#[derive(Debug, Parser)]
/// Implements a solution to the 14th day of Advent of Code 2021.
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

    let input = parse(&buf)?;

    let mut replacer = Replacer::new(&input);

    replacer.apply_n(10);
    println!("part 1 {}", replacer.frequency_diff());

    replacer.apply_n(30);
    println!("part 2 {}", replacer.frequency_diff());

    Ok(())
}
