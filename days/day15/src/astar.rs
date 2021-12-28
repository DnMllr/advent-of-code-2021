use crate::{
    grid::{GridRef, Position},
    search::Search,
};

pub trait GridStrategy {
    fn cost(position: &Position, grid: GridRef) -> usize;

    fn bounds(grid: GridRef) -> Position {
        Position {
            y: grid.len(),
            x: grid
                .first()
                .expect("there must be at least one row in the grid")
                .len(),
        }
    }

    fn end(grid: GridRef) -> Position {
        let mut bounds = Self::bounds(grid);
        bounds.x -= 1;
        bounds.y -= 1;
        bounds
    }
}

pub fn a_star<P: GridStrategy>(grid: GridRef) -> Option<usize> {
    let mut search = Search::<P>::start(grid);

    while let Some(current) = search.next() {
        let current_cost = search.cost(&current);

        if search.is_done(&current) {
            return current_cost;
        }

        for neighbor in neighbors::<P>(&current, grid) {
            search.visit(&neighbor, current_cost.unwrap() + P::cost(&neighbor, grid))
        }
    }

    None
}

fn neighbors<P: GridStrategy>(node: &Position, grid: GridRef) -> impl Iterator<Item = Position> {
    let bounds = P::bounds(grid);
    std::iter::empty()
        .chain(node.x.checked_sub(1).map(|x| Position { x, ..*node }))
        .chain(node.y.checked_sub(1).map(|y| Position { y, ..*node }))
        .chain(
            (node.x + 1 < bounds.x)
                .then(|| node.x + 1)
                .map(|x| Position { x, ..*node }),
        )
        .chain(
            (node.y + 1 < bounds.y)
                .then(|| node.y + 1)
                .map(|y| Position { y, ..*node }),
        )
}
