use std::{fmt::Display, str::FromStr};

use thiserror::Error;

#[derive(Debug, Default)]
pub struct Grid {
    width: usize,
    queue: Vec<usize>,
    octopodes: Vec<u8>,
}

impl Grid {
    pub fn first_synchronization(&mut self) -> usize {
        let len = self.octopodes.len();
        (1..)
            .map(|i| (i, self.step()))
            .find(|(_, count)| *count == len)
            .map(|(i, _)| i)
            .unwrap()
    }

    pub fn step_n(&mut self, n: usize) -> usize {
        (0..n).map(|_| self.step()).sum()
    }

    pub fn step(&mut self) -> usize {
        let mut buf = [0; 8];

        for i in 0..self.octopodes.len() {
            self.octopodes[i] += 1;
            if self.octopodes[i] > 9 {
                self.octopodes[i] = 0;
                self.queue.extend_from_slice(self.offsets_for(i, &mut buf));
            }
        }

        while !self.queue.is_empty() {
            let i = self
                .queue
                .pop()
                .expect("checked to see that queue was not empty");
            if self.octopodes[i] != 0 {
                self.octopodes[i] += 1;
                if self.octopodes[i] > 9 {
                    self.octopodes[i] = 0;
                    self.queue.extend_from_slice(self.offsets_for(i, &mut buf));
                }
            }
        }

        self.octopodes.iter().filter(|&&o| o == 0).count()
    }

    fn offsets_for<'a>(&self, index: usize, buf: &'a mut [usize; 8]) -> &'a [usize] {
        let mut len = 0;

        // left
        if index % self.width > 0 {
            if index > self.width {
                self.push_into_buf(index - 1 - self.width, &mut len, buf);
            }

            self.push_into_buf(index - 1, &mut len, buf);
            self.push_into_buf(index - 1 + self.width, &mut len, buf);
        }

        // right
        if (index + 1) % self.width > 0 {
            if (index + 1) > self.width {
                self.push_into_buf(index + 1 - self.width, &mut len, buf);
            }

            self.push_into_buf(index + 1, &mut len, buf);
            self.push_into_buf(index + 1 + self.width, &mut len, buf);
        }

        // center
        if index >= self.width {
            self.push_into_buf(index - self.width, &mut len, buf);
        }

        self.push_into_buf(index + self.width, &mut len, buf);

        &buf[..len]
    }

    fn push_into_buf(&self, data: usize, len: &mut usize, buf: &mut [usize]) {
        if data < self.octopodes.len() {
            buf[*len] = data;
            *len += 1;
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for octopodes in self.octopodes.chunks_exact(self.width) {
            for &octopus in octopodes {
                if octopus == 0 || octopus > 9 {
                    write!(f, " [{}] ", octopus)?;
                } else {
                    write!(f, "  {}  ", octopus)?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl FromStr for Grid {
    type Err = GridParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = Grid::default();
        for (line_number, line) in s.lines().enumerate() {
            if line_number == 0 {
                grid.width = line.len();
            } else if line.len() != grid.width {
                return Err(GridParseError::UnevenGridDimensions(
                    grid.width,
                    line_number + 1,
                    line.len(),
                ));
            }

            for octopus in line.chars().map(|c| {
                c.to_digit(10)
                    .ok_or(GridParseError::BadCharacter(c, line_number))
            }) {
                grid.octopodes.push(octopus? as u8);
            }
        }

        grid.queue.reserve_exact(grid.octopodes.len());
        Ok(grid)
    }
}

#[derive(Debug, Error)]
pub enum GridParseError {
    #[error("{0} was not an ascii digit on line {1}")]
    BadCharacter(char, usize),

    #[error("uneven grid dimensions. Expected {0} but line {1} was {2} characters long")]
    UnevenGridDimensions(usize, usize, usize),
}

#[cfg(test)]
mod tests {
    use super::Grid;

    #[test]
    fn test_offsets() {
        let grid = Grid {
            width: 3,
            queue: Vec::with_capacity(9),
            octopodes: vec![0; 9],
        };
        let mut buf = [0; 8];

        assert_eq!(&[0, 3, 6, 2, 5, 8, 1, 7], grid.offsets_for(4, &mut buf));
        assert_eq!(&[1, 4, 3], grid.offsets_for(0, &mut buf));
        assert_eq!(&[4, 7, 5], grid.offsets_for(8, &mut buf));
        assert_eq!(&[1, 4, 7, 2, 8], grid.offsets_for(5, &mut buf));
        assert_eq!(&[1, 4, 7, 0, 6], grid.offsets_for(3, &mut buf));
    }

    #[test]
    fn test_step() {
        let mut grid: Grid = include_str!("../input/testinput.txt")
            .parse()
            .expect("should be able to parse input");
        assert_eq!(0, grid.step());
        assert_eq!(35, grid.step());
        assert_eq!(45, grid.step());
        assert_eq!(16, grid.step());
        assert_eq!(8, grid.step());
        assert_eq!(1, grid.step());
        assert_eq!(7, grid.step());
        assert_eq!(24, grid.step());
    }

    #[test]
    fn test_step_100() {
        let mut grid: Grid = include_str!("../input/testinput.txt")
            .parse()
            .expect("should be able to parse input");

        assert_eq!(1656, grid.step_n(100));
    }

    #[test]
    fn test_first_synchronization() {
        let mut grid: Grid = include_str!("../input/testinput.txt")
            .parse()
            .expect("should be able to parse input");

        assert_eq!(195, grid.first_synchronization());
    }
}
