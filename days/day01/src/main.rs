/// Code for [AOC day 1](https://adventofcode.com/2021/day/1).
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use structopt::StructOpt;

/// Cli is the options for the application. Uses the [StructOpt] macro
/// to derive functionality for parsing command line args and generating
/// help text. Check the docs [here](https://docs.rs/structopt/latest/structopt/).
#[derive(StructOpt)]
#[structopt(name = "aoc2021-day-1", about = "The first day of advent of code")]
struct Cli {
    /// This field will hold the input file location.
    #[structopt(parse(from_os_str), help = "input file path")]
    input: PathBuf,

    /// This flag allows the user to run part 2
    #[structopt(short = "2", help = "whether to run part 2 or not")]
    part_2: bool,
}

impl Cli {
    /// a little helper method for reporting whether part 1 should be run
    fn part_1(&self) -> bool {
        !self.part_2
    }
}

/// This trait is for a type which can be used to solve today's problem.
trait Solver {
    /// This method is for feeding a particular input digit into the type.
    /// Today's problem has a long list of numbers (here represented as 32 bit signed integers (i32))
    /// which are fed into the computation one by one. This method is for feeding the next int
    /// into the solver.
    fn next(&mut self, num: i32);

    /// Once all of the ints have been fed into the solver. Calling this method will return the solution.
    /// Since we're counting, we use an unsigned int.
    fn solution(&self) -> usize;
}

/// This type holds the current state of the computation. Before we've seen any values
/// The internal [Option] will be None. Once we've seen input we're able to set the state
/// to a tuple where the first value is the current count of rising depths and the second
/// number is the previous depth. This process is modeled as a state machine.
#[derive(Default)]
struct State(Option<(usize, i32)>);

impl State {
    /// Given a number, produce the next State.
    fn transition(&self, num: i32) -> Self {
        // Rust's match statement is super convenient for modeling state transitions.
        State(match self.0 {
            // if we haven't seen any input yet, then we initialize the state to Some((0, whatever the first number is)).
            None => Some((0, num)),

            // If we have seen input and the last number is smaller than the current number, then we increase the sum by one
            // and store the current number.
            Some((sum, prev)) if num > prev => Some((sum + 1, num)),

            // If the number isn't greater than the previous number, then we don't increment the sum and we just store
            // the current number.
            Some((sum, _)) => Some((sum, num)),
        })
    }
}

/// Implementation of solver for our state
impl Solver for State {
    fn next(&mut self, num: i32) {
        // Given new input, we set ourselves to whatever the next state is.
        *self = self.transition(num);
    }

    fn solution(&self) -> usize {
        // the solution is whatever the sum is at the given moment, or 0 if we haven't seen any input
        self.0.map(|(s, _)| s).unwrap_or(0)
    }
}

/// Part 2 of this problem asks that we buffer up numbers into groups of 3. This type implements a buffer
/// which is able to be placed in front of a downstream solver. This buffer is parameterized by the downstream
/// solver and also the size of the buffer, which is passed at compile time as a const usize.
#[derive(Debug)]
struct Buffer<I: Solver, const S: usize> {
    /// The buffer itself. This is an array which is instantiated inline into this type. Since this type
    /// will be on the stack this buffer doesn't require any allocation.
    window: [i32; S],

    /// The count of inputs this buffer has seen
    count: usize,

    /// The next solver in the chain
    next: I,
}

impl<I: Solver, const S: usize> Buffer<I, S> {
    /// Constructs a new buffer which is placed in front of some downstream solver.
    fn new(next: I) -> Self {
        Self {
            window: [0; S],
            count: 0,
            next,
        }
    }

    /// Pushes new input into the buffer. Returns `Some(i32)` if the buffer is full
    /// and the sum of the buffer is able to be reported. Otherwise returns `None`.
    /// Where `S == 3`, this method will return `None` until 3 input numbers have
    /// been pushed.
    fn push(&mut self, num: i32) -> Option<i32> {
        // since order inside the buffer doesn't matter we can just write to it like a
        // [ring buffer](https://en.wikipedia.org/wiki/Circular_buffer)
        self.window[self.count % S] = num;
        self.count += 1;
        (self.count >= S).then(|| self.window.iter().sum())
    }
}

impl<I: Solver, const S: usize> Solver for Buffer<I, S> {
    fn next(&mut self, num: i32) {
        // if there is a sum ready from the buffer then send it to the downstream Solver.
        if let Some(sum) = self.push(num) {
            self.next.next(sum);
        }
    }

    fn solution(&self) -> usize {
        // The buffer's solution is whatever the downstream solution is.
        self.next.solution()
    }
}

/// The main function which is the entrypoint into the executable
fn main() -> color_eyre::Result<()> {
    // color_eyre is an error handling library for Rust which makes error reporting
    // a little prettier and more convenient. This install() call sets up panic handlers
    // so that if the application panics it'll report the error the same way as if there
    // was a handled exception. Docs are here: https://docs.rs/color-eyre/latest/color_eyre/
    color_eyre::install()?;

    // Read the command line args.
    let opt = Cli::from_args();

    // Open the input file and wrap it in buffered reader
    let reader = BufReader::new(File::open(&opt.input)?);

    // If the user has not requested part 2 then run part 1
    let solution = if opt.part_1() {
        // part 1 has unbuffered input and we can just use our state machine on each line directly
        run(State::default(), reader)?
    } else {
        // in part 2 we wrap our state machine in the above buffer but otherwise proceed as normal
        run(Buffer::<State, 3>::new(State::default()), reader)?
    };

    // print the answer
    println!("answer: {}", solution);

    // exit success
    Ok(())
}

/// The `run` function takes a solver and input (in the form of some type which implements BufRead)
/// and reads each line of the input into the solver. It then reports the solution if it succeeds.
fn run<S: Solver, B: BufRead>(mut solver: S, reader: B) -> color_eyre::Result<usize> {
    // for each line of the input
    // Note: this is the only place in the application where there is any heap allocation. We could
    // potentially optimize that by reusing a buffer, but why?
    for line in reader.lines() {
        // trim the line, parse it (bubbling errors up along the way if anything fails), and then
        // pass the parsed number into the solver.
        solver.next(line?.trim().parse()?);
    }

    // return the solution once there are no more remaining lines.
    Ok(solver.solution())
}
