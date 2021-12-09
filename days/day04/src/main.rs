use std::{fs::File, io::Read, path::PathBuf};

use index::Index;
use structopt::StructOpt;

mod index;
mod parser;

#[derive(StructOpt)]
#[structopt(name = "aoc2021-day-4", about = "The fourth day of advent of code")]
struct Cli {
    #[structopt(parse(from_os_str), help = "the input file from adventofcode")]
    input: PathBuf,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let opts = Cli::from_args();
    let mut buf = String::new();

    File::open(opts.input)?.read_to_string(&mut buf)?;

    let parser::Parse { numbers, boards } = parser::parse(&buf)?;

    let mut index: Index = boards.into();

    for (i, (board, score)) in numbers
        .into_iter()
        .filter_map(|n| index.call_number(n))
        .flat_map(|w| w.into_iter())
        .enumerate()
    {
        println!("{}. board {} won with score of {}", i, board, score);
    }

    Ok(())
}
