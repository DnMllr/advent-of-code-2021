use std::{fs::File, io::Read, path::PathBuf};

use clap::Parser;
use model::Solver;
use nom::error::Error;
use parser::{Input, Line};
use rayon::{
    iter::{IntoParallelRefIterator, ParallelIterator},
    str::ParallelString,
};

use crate::model::solve;

mod model;
mod parser;
mod pattern;
mod tables;

#[derive(Debug, Parser)]
/// Implements a solution to the 8th day of Advent of Code 2021.
struct Options {
    #[clap()]
    /// Path to the file that contains the input.
    input: PathBuf,

    #[clap(long)]
    /// Whether or not to run part_2
    part_2: bool,

    #[clap(short, long)]
    /// Whether to run in parallel or not
    parallel: bool,
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

    if opts.parallel {
        run_parallel(&opts, buf)
    } else {
        run_serial(&opts, buf)
    }
}

fn run_serial(opts: &Options, buf: String) -> color_eyre::Result<()> {
    let input: Input = buf.parse()?;
    if opts.part_1() {
        let part_1 = input
            .outputs()
            .flat_map(|p| p.iter())
            .filter(|&s| s.possible_numbers().len() == 1)
            .count();

        println!("part 1: {}", part_1);
    } else {
        let mut solver = Solver::default();
        let sum: usize = input
            .lines()
            .map(|line| {
                solver.reset();
                solve(line, &mut solver).expect("advent of code input is well formed")
            })
            .sum();

        println!("part 2: {}", sum);
    }

    Ok(())
}

fn run_parallel(opts: &Options, buf: String) -> color_eyre::Result<()> {
    let input = Input::new(
        buf.par_lines()
            .map(|l| l.parse())
            .collect::<Result<Vec<Line>, Error<String>>>()?,
    );

    if opts.part_1() {
        let part_1 = input
            .par_outputs()
            .flat_map(|p| p.par_iter())
            .filter(|p| p.possible_numbers().len() == 1)
            .count();

        println!("part 1: {}", part_1);
    } else {
        let part_2: usize = input
            .par_lines()
            .map_init(Solver::default, |s, l| {
                s.reset();
                solve(l, s).expect("advent of code input is well formed")
            })
            .sum();

        println!("part 2: {}", part_2);
    }

    Ok(())
}
