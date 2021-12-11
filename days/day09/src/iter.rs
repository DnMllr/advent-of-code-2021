use crate::{grid::Grid, location::Location};

pub struct LocationIter<'a> {
    grid: &'a Grid,
    x: usize,
    y: usize,
}

impl<'a> Iterator for LocationIter<'a> {
    type Item = Location<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.grid.get(self.x, self.y)?;

        let option = Location::new(self.grid, self.x, self.y);

        self.x += 1;
        if self.x >= self.grid.size_x() {
            self.x = 0;
            self.y += 1;
        }

        Some(option)
    }
}

impl<'a> LocationIter<'a> {
    pub fn new(board: &'a Grid) -> Self {
        Self {
            grid: board,
            x: 0,
            y: 0,
        }
    }
}
