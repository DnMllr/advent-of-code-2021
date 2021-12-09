use std::{fs::File, io::Read, num::ParseIntError, path::PathBuf};

use rayon::prelude::*;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "aoc2021-day-6", about = "The sixth day of advent of code.")]
struct Cli {
    #[structopt(parse(from_os_str), help = "the input file from adventofcode")]
    input: PathBuf,

    #[structopt(short, long, help = "how many days to run the simulation for")]
    days: usize,

    #[structopt(
        short,
        long,
        help = "If set, will simulate individual fish rather than the entire group. Much slower."
    )]
    brute: bool,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let opts = Cli::from_args();
    let mut buf = String::new();

    File::open(opts.input)?.read_to_string(&mut buf)?;

    let mut fish = buf
        .split(',')
        .map(|n| n.trim().parse())
        .collect::<Result<Vec<i8>, ParseIntError>>()?;

    if opts.brute {
        let mut new_fish = 0;
        for _ in 0..opts.days {
            let next_new_fish = fish
                .par_iter_mut()
                .fold(
                    || 0,
                    |c, i| {
                        *i -= 1;
                        match *i {
                            0 => c + 1,
                            x if x < 0 => {
                                *i = 6;
                                c
                            }
                            _ => c,
                        }
                    },
                )
                .sum();
            fish.resize(fish.len() + new_fish, 8);
            new_fish = next_new_fish;
        }

        println!("number of fish: {}", fish.len());
    } else {
        let mut fish_count = [0usize; 9];

        for f in fish {
            fish_count[f as usize] += 1;
        }

        for _ in 0..opts.days {
            let next_fish = fish_count[0];
            fish_count.rotate_left(1);
            fish_count[6] += next_fish;
            fish_count[8] = next_fish;
        }

        println!("number of fish: {}", fish_count.iter().sum::<usize>());
    }

    Ok(())
}
