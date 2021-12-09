use std::{fs::File, io::Read, num::ParseIntError, path::PathBuf};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "aoc2021-day-6", about = "The sixth day of advent of code.")]
struct Cli {
    #[structopt(parse(from_os_str), help = "the input file from adventofcode")]
    input: PathBuf,

    #[structopt(short, long, help = "whether or not to run part 2")]
    part_2: bool,
}

impl Cli {
    pub fn part_1(&self) -> bool {
        !self.part_2
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let opts = Cli::from_args();
    let mut buf = String::new();

    File::open(&opts.input)?.read_to_string(&mut buf)?;

    let mut data = buf
        .split(',')
        .map(|s| s.trim().parse())
        .collect::<Result<Vec<i32>, ParseIntError>>()?;

    data.sort_unstable();

    let (&start, &end) = data.first().zip(data.last()).expect("there was no input");

    let min = if opts.part_1() {
        let mut score = (data.len() as i32) + data.iter().map(|n| n - start).sum::<i32>();
        let mut min = score;
        let mut index = 0;

        for i in start..=end {
            while data[index] < i {
                index += 1;
            }

            let left = &data[..index];
            let right = &data[index..];

            score += left.len() as i32;
            score -= right.len() as i32;
            min = min.min(score);
        }

        min
    } else {
        let mut min = i32::MAX;

        //TODO(Dan): is there a faster way to do this similar to the way we do it for part 1?
        for i in start..=end {
            let cost = data
                .iter()
                .map(|n| (n - i).abs())
                .map(|n| n + 1)
                .map(|n| (n * (n - 1)) / 2)
                .sum();

            min = min.min(cost);
        }

        min
    };

    println!("min cost {}", min);

    Ok(())
}
