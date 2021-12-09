use std::{iter::repeat, str::FromStr};

use fxhash::FxHashMap;

use crate::parser;

#[derive(Debug, Default)]
pub struct Board {
    data: FxHashMap<Point, usize>,
}

impl Board {
    pub fn push(&mut self, arrow: &Arrow) {
        for point in arrow.iter() {
            *self.data.entry(point).or_default() += 1;
        }
    }

    pub fn solutions(&self) -> impl Iterator<Item = (&Point, &usize)> {
        self.data.iter().filter(|(_, &c)| c > 1)
    }
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

impl std::ops::Sub for &Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Debug)]
pub struct Arrow {
    pub from: Point,
    pub to: Point,
}

impl FromStr for Arrow {
    type Err = nom::error::Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parser::parse(s)
    }
}

impl From<(Point, Point)> for Arrow {
    fn from((from, to): (Point, Point)) -> Self {
        Self { from, to }
    }
}

impl Arrow {
    pub fn iter(&self) -> Box<dyn Iterator<Item = Point>> {
        match &self.to - &self.from {
            Point { x: 0, y } => Box::new(
                repeat(self.from.x)
                    .zip(Self::diff_range(self.from.y, y))
                    .map(Point::from),
            ),
            Point { x, y: 0 } => Box::new(
                repeat(self.from.y)
                    .zip(Self::diff_range(self.from.x, x))
                    .map(|(y, x)| Point::from((x, y))),
            ),
            Point { x, y } => Box::new(
                Self::signed_diff_range(self.from.x, x)
                    .zip(Self::signed_diff_range(self.from.y, y))
                    .map(Point::from),
            ),
        }
    }

    fn signed_diff_range(base: i32, diff: i32) -> Box<dyn Iterator<Item = i32>> {
        if diff < 0 {
            Box::new(Self::diff_range(base, diff).rev())
        } else {
            Box::new(Self::diff_range(base, diff))
        }
    }

    fn diff_range(base: i32, diff: i32) -> std::ops::RangeInclusive<i32> {
        base.min(base + diff)..=base.max(base + diff)
    }
}
