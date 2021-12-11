use crate::grid::Grid;

#[derive(Debug, Clone)]
pub struct Location<'a> {
    pub x: usize,
    pub y: usize,
    grid: &'a Grid,
}

impl<'a> Location<'a> {
    pub fn new(grid: &'a Grid, x: usize, y: usize) -> Self {
        Self { grid, x, y }
    }

    pub fn index(&self) -> usize {
        self.y * self.grid.size_x() + self.x
    }

    pub fn item(&self) -> u8 {
        self.grid
            .get(self.x, self.y)
            .expect("this location must be valid")
    }

    pub fn top(&'a self) -> Option<Location<'a>> {
        self.y
            .checked_sub(1)
            .and_then(|y| self.deref_grid_with_y(y))
    }

    pub fn bottom(&'a self) -> Option<Location<'a>> {
        self.y
            .checked_add(1)
            .and_then(|y| self.deref_grid_with_y(y))
    }

    pub fn left(&'a self) -> Option<Location<'a>> {
        self.x
            .checked_sub(1)
            .and_then(|x| self.deref_grid_with_x(x))
    }

    pub fn right(&'a self) -> Option<Location<'a>> {
        self.x
            .checked_add(1)
            .and_then(|x| self.deref_grid_with_x(x))
    }

    pub fn neighbors<'b>(&'a self, buf: &'b mut [Location<'a>; 4]) -> &'b [Location<'a>] {
        let mut len = 0;

        self.push_neighbor(buf, &mut len, self.top());
        self.push_neighbor(buf, &mut len, self.bottom());
        self.push_neighbor(buf, &mut len, self.left());
        self.push_neighbor(buf, &mut len, self.right());

        &buf[..len]
    }

    pub fn default_buffer(&self) -> [Location<'a>; 4] {
        [
            Location::new(self.grid, 0, 0),
            Location::new(self.grid, 0, 0),
            Location::new(self.grid, 0, 0),
            Location::new(self.grid, 0, 0),
        ]
    }

    pub fn is_low_point(&self) -> bool {
        self.neighbors(&mut self.default_buffer())
            .iter()
            .all(|i| i.item() > self.item())
    }

    pub fn risk(&self) -> u8 {
        self.item() + 1
    }

    fn push_neighbor(&self, buf: &mut [Location<'a>; 4], i: &mut usize, opt: Option<Location<'a>>) {
        if let Some(item) = opt {
            buf[*i] = item.clone();
            *i += 1;
        }
    }

    fn deref_grid_with_y(&self, y: usize) -> Option<Location> {
        self.grid
            .get(self.x, y)
            .map(|_| Self::new(self.grid, self.x, y))
    }

    fn deref_grid_with_x(&self, x: usize) -> Option<Location> {
        self.grid
            .get(x, self.y)
            .and_then(|_| (x < self.grid.size_x()).then(|| Self::new(self.grid, x, self.y)))
    }
}
