use std::str::FromStr;

use partitions::{partition_vec::AllSets, PartitionVec};
use thiserror::Error;

use crate::iter::LocationIter;

#[derive(Debug)]
pub struct Grid {
    x: usize,
    data: PartitionVec<u8>,
}

impl Grid {
    fn build(data: PartitionVec<u8>, x: usize) -> Self {
        let mut s = Self { data, x };

        // TODO: it's definitely possible to remove this queue. Just need a little rearchitecting.
        let mut queue = Vec::new();

        for location in s.locations().filter(|l| l.item() != 9) {
            for neighbor in location
                .neighbors(&mut location.default_buffer())
                .iter()
                .filter(|l| l.item() != 9)
            {
                queue.push((location.index(), neighbor.index()));
            }
        }

        for (l, r) in queue {
            s.data.union(l, r);
        }

        s
    }

    pub fn basins(&self) -> AllSets<u8> {
        self.data.all_sets()
    }

    pub fn size_x(&self) -> usize {
        self.x
    }

    pub fn get(&self, x: usize, y: usize) -> Option<u8> {
        self.data.get(y * self.x + x).copied()
    }

    pub fn locations(&self) -> LocationIter {
        LocationIter::new(self)
    }
}

impl FromStr for Grid {
    type Err = ParseGridError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .lines()
            .flat_map(|line| {
                line.chars().map(|c| {
                    c.to_digit(10)
                        .map(|d| d as u8)
                        .ok_or(ParseGridError::BadInt(c))
                })
            })
            .collect::<Result<PartitionVec<u8>, ParseGridError>>()?;

        let x = s
            .lines()
            .next()
            .ok_or(ParseGridError::BadDimensions)?
            .chars()
            .count();

        Ok(Self::build(data, x))
    }
}

#[derive(Debug, Error)]
pub enum ParseGridError {
    #[error("{0} is not a valid digit")]
    BadInt(char),
    #[error("grid did not have even dimensions")]
    BadDimensions,
}
