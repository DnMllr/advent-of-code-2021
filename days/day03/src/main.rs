use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use bit_iter::BitIter;
use structopt::StructOpt;

// These numbers were arrived at by inspecting the input file
const INPUT_WIDTH: usize = 12;
const INPUT_LENGTH: usize = 1000;

#[derive(StructOpt)]
#[structopt(name = "aoc2021-day-3", about = "The third day of advent of code")]
struct Cli {
    #[structopt(parse(from_os_str), help = "the input file from adventofcode")]
    input: PathBuf,
}

trait Stat {
    fn calc(&mut self) -> u16;
}

#[derive(Debug)]
struct O2(Vec<u16>);

impl From<Vec<u16>> for O2 {
    fn from(v: Vec<u16>) -> Self {
        Self(v)
    }
}

impl Stat for O2 {
    fn calc(&mut self) -> u16 {
        strain(
            &mut self.0,
            INPUT_WIDTH - 1,
            &[Ordering::Greater, Ordering::Equal],
        )
        .expect("expects valid input")
    }
}

#[derive(Debug)]
struct CO2(Vec<u16>);

impl From<Vec<u16>> for CO2 {
    fn from(v: Vec<u16>) -> Self {
        Self(v)
    }
}

impl Stat for CO2 {
    fn calc(&mut self) -> u16 {
        strain(&mut self.0, INPUT_WIDTH - 1, &[Ordering::Less]).expect("expects valid input")
    }
}

fn strain(data: &mut Vec<u16>, mut width: usize, comparison: &[Ordering]) -> Option<u16> {
    while data.len() > 1 {
        width -= 1;
        let count = data.iter().filter(|&n| n & (1 << width) > 0).count();
        let include = comparison.contains(&count.cmp(&((data.len() + 1) / 2)));
        data.retain(|&o| {
            if include {
                o & (1 << width) > 0
            } else {
                o & (1 << width) == 0
            }
        });
    }

    data.first().copied()
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let opts = Cli::from_args();

    let reader = BufReader::new(File::open(opts.input)?);
    let mut numbers = [0; INPUT_LENGTH];
    let mut count = [0; INPUT_WIDTH];

    populate_arrays(&mut numbers, &mut count, reader)?;

    let (gamma, epsilon) = part_1(&count, numbers.len());
    println!("part 1: {}", gamma * epsilon);

    let (oxygen, co2) = part_2(&numbers, &count);
    println!("part 2: {}", oxygen * co2);

    Ok(())
}

fn populate_arrays<R: BufRead>(
    numbers: &mut [u16],
    count: &mut [usize],
    reader: R,
) -> color_eyre::Result<()> {
    for (line, slot) in reader.lines().zip(numbers.iter_mut()) {
        *slot = u16::from_str_radix(line?.trim(), 2)?;
        for index in BitIter::from(*slot) {
            count[index] += 1;
        }
    }

    Ok(())
}

fn part_1(count: &[usize], len: usize) -> (usize, usize) {
    let gamma = count
        .iter()
        .enumerate()
        .filter_map(|(index, &i)| (i >= (len / 2)).then(|| index))
        .fold(0usize, |g, i| g | (1 << i));

    let mask = (1 << 12) - 1;
    let epsilon = !gamma & mask;

    (gamma, epsilon)
}

fn part_2(numbers: &[u16], count: &[usize]) -> (usize, usize) {
    let (mut o2, mut co2) = initial_sort(numbers, count);

    (o2.calc() as usize, co2.calc() as usize)
}

fn initial_sort(numbers: &[u16], count: &[usize]) -> (O2, CO2) {
    let mut o2 = Vec::new();
    let mut co2 = Vec::new();

    let last = *count.last().expect("a last should have been produced");
    let initial_bit = last >= (numbers.len() + 1) / 2;

    for n in numbers {
        let (l, r) = if initial_bit {
            (&mut o2, &mut co2)
        } else {
            (&mut co2, &mut o2)
        };
        if n & (1 << (count.len() - 1)) > 0 {
            l.push(*n);
        } else {
            r.push(*n);
        }
    }

    (o2.into(), co2.into())
}
