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

enum State {
    Empty,
    First(i32),
    Rest(usize, i32),
}

impl State {
    fn next(&mut self, num: i32) {
        *self = self.make_next(num);
    }

    fn make_next(&self, num: i32) -> Self {
        match *self {
            State::Empty => State::First(num),
            State::First(prev) if num > prev => State::Rest(1, num),
            State::First(_) => State::Rest(0, num),
            State::Rest(sum, prev) if num > prev => State::Rest(sum + 1, num),
            State::Rest(sum, _) => State::Rest(sum, num),
        }
    }

    fn solution(&self) -> usize {
        match self {
            State::Rest(sum, _) => *sum,
            _ => 0,
        }
    }
}

#[derive(Debug)]
struct Window<const S: usize> {
    window: [i32; S],
    count: usize,
}

impl<const S: usize> Window<S> {
    fn new() -> Self {
        Self {
            window: [0; S],
            count: 0,
        }
    }

    fn next(&mut self, num: i32) -> Option<i32> {
        self.window.rotate_left(1);
        self.window[S - 1] = num;
        self.count += 1;
        (self.count >= S).then(|| self.window.iter().sum())
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let opt = Cli::from_args();

    let reader = BufReader::new(File::open(&opt.input)?);
    let mut state = State::Empty;

    if opt.part_1() {
        for line in reader.lines() {
            state.next(line?.trim().parse()?);
        }
    } else {
        let mut window = Window::<3>::new();

        for line in reader.lines() {
            if let Some(sum) = window.next(line?.trim().parse()?) {
                state.next(sum);
            }
        }
    }

    println!("answer: {}", state.solution());

    Ok(())
}
