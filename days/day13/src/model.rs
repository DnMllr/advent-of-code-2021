use std::{collections::HashSet, fmt::Display};

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Point {
    fn from((x, y): (usize, usize)) -> Self {
        Point { x, y }
    }
}

pub enum Fold {
    Y(usize),
    X(usize),
}

impl Fold {
    pub fn apply(&self, point: &mut Point) {
        match *self {
            Fold::Y(y) if point.y > y => {
                point.y = y - (point.y - y);
            }
            Fold::X(x) if point.x > x => {
                point.x = x - (point.x - x);
            }
            _ => {}
        }
    }
}

pub struct Input {
    folds: Vec<Fold>,
    from: HashSet<Point>,
    to: HashSet<Point>,
}

impl Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_x = self.from.iter().map(|p| p.x).max().unwrap_or(0);
        let max_y = self.from.iter().map(|p| p.y).max().unwrap_or(0);

        for y in 0..=max_y {
            for x in 0..=max_x {
                if self.from.contains(&Point { x, y }) {
                    write!(f, "#")?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl From<(Vec<Point>, Vec<Fold>)> for Input {
    fn from((points, mut folds): (Vec<Point>, Vec<Fold>)) -> Self {
        folds.reverse();
        let (mut from, to) = (
            HashSet::with_capacity(points.len()),
            HashSet::with_capacity(points.len()),
        );

        for point in points {
            from.insert(point);
        }
        Self { folds, from, to }
    }
}

impl Input {
    pub fn fold(&mut self) -> Option<usize> {
        self.folds.pop().map(|fold| {
            for mut point in self.from.drain() {
                fold.apply(&mut point);
                self.to.insert(point);
            }

            std::mem::swap(&mut self.from, &mut self.to);
            self.count_points()
        })
    }

    pub fn fold_all(&mut self) -> usize {
        std::iter::repeat(())
            .map(|_| self.fold())
            .take_while(Option::is_some)
            .map(Option::unwrap)
            .sum()
    }

    pub fn count_points(&self) -> usize {
        self.from.len()
    }
}
