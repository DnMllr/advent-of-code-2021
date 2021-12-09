use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use model::Board;
use structopt::StructOpt;

mod model;
mod parser;

#[derive(StructOpt)]
#[structopt(
    name = "aoc2021-day-5",
    about = "The fifth day of advent of code. Note: this command will only solve part 2"
)]
struct Cli {
    #[structopt(parse(from_os_str), help = "the input file from adventofcode")]
    input: PathBuf,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let opts = Cli::from_args();

    let reader = BufReader::new(File::open(opts.input)?);

    let mut board = Board::default();

    for line in reader.lines() {
        board.push(&line?.parse()?);
    }

    println!("solution: {}", board.solutions().count());

    Ok(())
}
