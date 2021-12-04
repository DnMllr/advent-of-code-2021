use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use color_eyre::Result;
use command::Command;
use structopt::StructOpt;

mod command;

#[derive(StructOpt)]
#[structopt(name = "aoc2021-day-2", about = "The second day of advent of code")]
struct Cli {
    #[structopt(parse(from_os_str), help = "the input file from adventofcode")]
    input: PathBuf,

    #[structopt(short = "2", help = "pass to run part 2")]
    part_2: bool,
}

impl Cli {
    pub fn part_1(&self) -> bool {
        !self.part_2
    }
}

trait Solver {
    fn input(&mut self, cmd: Command);
    fn position(&self) -> &Position;
}

#[derive(Debug, Default)]
struct Position {
    pub depth: i32,
    pub horizontal: i32,
}

impl Position {
    fn solution(&self) -> i32 {
        self.depth * self.horizontal
    }
}

#[derive(Debug, Default)]
struct Part1 {
    pos: Position,
}

impl Solver for Part1 {
    fn input(&mut self, cmd: Command) {
        match cmd {
            Command::Forward(x) => self.pos.horizontal += x,
            Command::Up(x) => self.pos.depth -= x,
            Command::Down(x) => self.pos.depth += x,
        }
    }

    fn position(&self) -> &Position {
        &self.pos
    }
}

#[derive(Debug, Default)]
struct Part2 {
    pos: Position,
    aim: i32,
}

impl Solver for Part2 {
    fn input(&mut self, cmd: Command) {
        match cmd {
            Command::Forward(x) => {
                self.pos.horizontal += x;
                self.pos.depth += self.aim * x;
            }
            Command::Up(x) => self.aim -= x,
            Command::Down(x) => self.aim += x,
        }
    }

    fn position(&self) -> &Position {
        &self.pos
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let opts = Cli::from_args();

    let reader = BufReader::new(File::open(&opts.input)?);

    let solution = if opts.part_1() {
        run(Part1::default(), reader)?
    } else {
        run(Part2::default(), reader)?
    };

    println!("solution: {}", solution);

    Ok(())
}

fn run<S: Solver, B: BufRead>(mut s: S, reader: B) -> Result<i32> {
    for line in reader.lines() {
        s.input(line?.parse()?);
    }

    Ok(s.position().solution())
}
