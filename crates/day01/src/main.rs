use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "aoc2021-day-1", about = "The first day of advent of code")]
struct Cli {
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    #[structopt(short = "2")]
    part_2: bool,
}

impl Cli {
    fn part_1(&self) -> bool {
        !self.part_2
    }
}

trait Solver {
    fn next(&mut self, num: i32);
    fn solution(&self) -> usize;
}

#[derive(Default)]
struct State(Option<(usize, i32)>);

impl State {
    fn transition(&self, num: i32) -> Self {
        State(match self.0 {
            None => Some((0, num)),
            Some((sum, prev)) if num > prev => Some((sum + 1, num)),
            Some((sum, _)) => Some((sum, num)),
        })
    }
}

impl Solver for State {
    fn next(&mut self, num: i32) {
        *self = self.transition(num);
    }

    fn solution(&self) -> usize {
        match self.0 {
            Some((sum, _)) => sum,
            _ => 0,
        }
    }
}

#[derive(Debug)]
struct Buffer<I: Solver, const S: usize> {
    window: [i32; S],
    count: usize,
    next: I,
}

impl<I: Solver, const S: usize> Buffer<I, S> {
    fn new(next: I) -> Self {
        Self {
            window: [0; S],
            count: 0,
            next,
        }
    }

    fn push(&mut self, num: i32) -> Option<i32> {
        self.window.rotate_left(1);
        self.window[S - 1] = num;
        self.count += 1;
        (self.count >= S).then(|| self.window.iter().sum())
    }
}

impl<I: Solver, const S: usize> Solver for Buffer<I, S> {
    fn next(&mut self, num: i32) {
        if let Some(sum) = self.push(num) {
            self.next.next(sum);
        }
    }

    fn solution(&self) -> usize {
        self.next.solution()
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let opt = Cli::from_args();

    let reader = BufReader::new(File::open(&opt.input)?);

    let solution = if opt.part_1() {
        run(State::default(), reader)?
    } else {
        run(Buffer::<State, 3>::new(State::default()), reader)?
    };

    println!("answer: {}", solution);

    Ok(())
}

fn run<S: Solver, B: BufRead>(mut solver: S, reader: B) -> color_eyre::Result<usize> {
    for line in reader.lines() {
        solver.next(line?.trim().parse()?);
    }

    Ok(solver.solution())
}
