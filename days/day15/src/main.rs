use std::{marker::PhantomData, path::PathBuf};

use astar::GridStrategy;
use clap::Parser;
use grid::{GridRef, Position};

use crate::astar::a_star;

mod astar;
mod grid;
mod search;

#[derive(Debug, Parser)]
/// Implements a solution to the 15th day of Advent of Code 2021.
struct Options {
    #[clap()]
    /// Path to the file that contains the input.
    input: PathBuf,

    #[clap(long)]
    /// whether to run part 2 or not
    part_2: bool,
}

#[derive(Debug, Clone, Copy)]
struct Base;

impl GridStrategy for Base {
    fn cost(position: &Position, grid: GridRef) -> usize {
        position.lookup(grid).expect("position must be in range")
    }
}

#[derive(Debug, Clone, Copy)]
struct Multiplier<P: GridStrategy, const M: usize> {
    inner: PhantomData<P>,
}

impl<P: GridStrategy, const M: usize> GridStrategy for Multiplier<P, M> {
    fn cost(position: &Position, grid: GridRef) -> usize {
        let p_end = P::end(grid);
        let x = position.x % (p_end.x + 1);
        let y = position.y % (p_end.y + 1);
        let offset_x = position.x / (p_end.x + 1);
        let offset_y = position.y / (p_end.y + 1);
        let part_1_cost = P::cost(&Position { x, y }, grid);

        ((part_1_cost + offset_x + offset_y - 1) % 9) + 1
    }

    fn bounds(grid: GridRef) -> Position {
        let mut base = Base::bounds(grid);
        base.x *= M;
        base.y *= M;
        base
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let opts = Options::parse();

    let numbers = grid::read_grid(&opts.input)?;
    let answer = if opts.part_2 {
        a_star::<Multiplier<Base, 5>>(&numbers)
    } else {
        a_star::<Base>(&numbers)
    };

    println!(
        "{} {:?}",
        if opts.part_2 { "part 2" } else { "part 1" },
        answer
    );

    Ok(())
}
