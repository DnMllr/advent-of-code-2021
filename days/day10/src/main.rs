use std::{fs::File, io::Read, path::PathBuf};

use clap::Parser;
use command::Command;
use interpreter::Interpreter;

mod command;
mod delimeter;
mod interpreter;

#[derive(Debug, Parser)]
/// Implements a solution to the 10th day of Advent of Code 2021.
struct Options {
    #[clap()]
    /// Path to the file that contains the input.
    input: PathBuf,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let opts = Options::parse();
    let mut buf = String::new();

    File::open(&opts.input)?.read_to_string(&mut buf)?;

    println!("part 1 {}", part_1(&buf));
    println!("part 2 {}", part_2(&buf));

    Ok(())
}

fn part_1(source: &str) -> usize {
    Interpreter::new(Command::stream(source))
        .filter_map(Result::err)
        .filter(|e| e.is_corrupted())
        .filter_map(|e| e.score())
        .sum()
}

fn part_2(source: &str) -> usize {
    let mut results: Vec<usize> = Interpreter::new(Command::stream(source))
        .filter_map(Result::err)
        .filter(|e| e.is_incomplete())
        .filter_map(|e| e.score())
        .collect();

    results.sort_unstable();
    results[results.len() / 2]
}

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2};

    #[test]
    fn test_part_1() {
        let input = include_str!("../input/test.txt");
        assert_eq!(26397, part_1(input));
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("../input/test.txt");
        assert_eq!(288957, part_2(input));
    }
}
