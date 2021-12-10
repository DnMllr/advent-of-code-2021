use std::{fs::File, io::Read, path::PathBuf};

use clap::Parser;
use model::Solver;
use parser::Input;

mod model;
mod parser;

#[derive(Debug, Parser)]
/// Implements a solution to the 8th day of Advent of Code 2021.
struct Options {
    #[clap()]
    /// Path to the file that contains the input.
    input: PathBuf,

    #[clap(short, long)]
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

    let input: Input = buf.parse()?;

    if opts.part_1() {
        let part_1 = input
            .outputs()
            .flat_map(|p| p.iter())
            .filter(|&s| s.possible_numbers().len() == 1)
            .count();

        println!("part 1: {}", part_1);
    } else {
        let mut sum = 0;
        let mut solver = Solver::default();
        'OUTER: for line in input.lines() {
            solver.reset();
            for pattern in line.patterns() {
                if let Some(solution) = solver.add(*pattern) {
                    let mut answer = 0;
                    for output in line.output() {
                        answer *= 10;
                        answer += solution.solve(*output);
                    }
                    sum += answer;
                    continue 'OUTER;
                }
            }
        }

        println!("part 2: {}", sum);
    }

    Ok(())
}
