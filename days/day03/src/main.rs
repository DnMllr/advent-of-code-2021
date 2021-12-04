use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use bit_iter::BitIter;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "aoc2021-day-3", about = "The third day of advent of code")]
struct Cli {
    #[structopt(parse(from_os_str), help = "the input file from adventofcode")]
    input: PathBuf,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let opts = Cli::from_args();

    let reader = BufReader::new(File::open(opts.input)?);
    let mut numbers = [0; 1000];
    let mut count = [0; 12];

    populate_arrays(&mut numbers, &mut count, reader)?;

    let (gamma, epsilon) = part_1(&count, numbers.len());
    println!("part 1: {}", gamma * epsilon);

    let (oxygen, co2) = part_2(&numbers, count[11] >= numbers.len() / 2, 11);
    println!("part 2: {}", oxygen * co2);

    Ok(())
}

fn populate_arrays<R: BufRead>(
    numbers: &mut [u16],
    count: &mut [usize],
    reader: R,
) -> color_eyre::Result<()> {
    for (line, slot) in reader.lines().zip(numbers.iter_mut()) {
        *slot = u16::from_str_radix(&line?, 2)?;
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

// TODO(Dan) I'm not proud of this solution, there has to be a way to make this cleaner, but it works.
fn part_2(numbers: &[u16], initial_bit: bool, initial_pos: usize) -> (usize, usize) {
    let mut o2 = Vec::new();
    let mut co2 = Vec::new();

    for n in numbers {
        let (l, r) = if initial_bit {
            (&mut o2, &mut co2)
        } else {
            (&mut co2, &mut o2)
        };
        if n & (1 << initial_pos) > 0 {
            l.push(*n);
        } else {
            r.push(*n);
        }
    }

    let mut bit_index = initial_pos;

    while o2.len() > 1 {
        bit_index -= 1;
        let count = o2.iter().filter(|&n| n & (1 << bit_index) > 0).count();
        let include = count >= (o2.len() + 1) / 2;
        o2.retain(|&o| {
            if include {
                o & (1 << bit_index) > 0
            } else {
                o & (1 << bit_index) == 0
            }
        });
    }

    bit_index = initial_pos;

    while co2.len() > 1 {
        bit_index -= 1;
        let count = co2.iter().filter(|&n| n & (1 << bit_index) > 0).count();
        let include = count < (co2.len() + 1) / 2;
        co2.retain(|&o| {
            if include {
                o & (1 << bit_index) > 0
            } else {
                o & (1 << bit_index) == 0
            }
        });
    }

    (o2[0] as usize, co2[0] as usize)
}

#[cfg(test)]
mod tests {
    use crate::part_2;

    #[test]
    fn tests() {
        let input = [
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ];

        let r = part_2(&input, true, 4);

        assert_eq!((23, 10), r);
    }
}
