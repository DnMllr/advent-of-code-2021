use crate::{
    grid::{GridRef, Position},
    search::Search,
};

pub trait GridStrategy {
    fn cost(position: &Position, grid: GridRef) -> usize;

    fn bounds(grid: GridRef) -> Position {
        Position::bounds(grid)
    }

    fn end(grid: GridRef) -> Position {
        Self::bounds(grid) - 1
    }
}

pub fn a_star<P: GridStrategy>(grid: GridRef) -> Option<usize> {
    let bounds = P::bounds(grid);
    let mut search = Search::start(P::end(grid));

    while let Some(current) = search.next() {
        let current_cost = search.cost(&current);

        if search.is_done(&current) {
            return Some(current_cost);
        }

        for neighbor in current.neighbors(&bounds) {
            search.visit(&neighbor, current_cost + P::cost(&neighbor, grid))
        }
    }

    None
}
