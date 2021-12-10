use std::{fmt::Debug, str::FromStr};

use bit_iter::BitIter;
use thiserror::Error;

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pattern(u8);

impl Debug for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_tuple("Pattern");

        for c in self.chars() {
            d.field(&c);
        }

        d.finish()
    }
}

impl FromStr for Pattern {
    type Err = InvalidCharacter;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.chars()
                .map(|c| match c {
                    'a'..='g' => Ok(1 << char_index(c)),
                    bad_character => Err(InvalidCharacter { bad_character }),
                })
                .fold(Ok(0), |l, r| {
                    l.and_then(|l| r.map(|r| (l, r))).map(|(l, r)| l | r)
                })?,
        ))
    }
}

impl ToString for Pattern {
    fn to_string(&self) -> String {
        self.chars().collect()
    }
}

impl Pattern {
    const MASK: u8 = (1 << 7) - 1;

    pub const fn all() -> Self {
        Pattern(Self::MASK)
    }

    pub const fn from_char_list(list: &'static [char]) -> Self {
        let mut p = 0;
        let mut i = 0;

        while i < list.len() {
            p |= 1 << char_index(list[i]);
            i += 1;
        }

        Self(p)
    }

    pub const fn from_numbers_union(list: &'static [u8]) -> Self {
        let mut p = 0;
        let mut i = 0;

        while i < list.len() {
            p |= TABLE_NUMBER_TO_PATTERN[list[i] as usize].0;
            i += 1;
        }

        Self(p)
    }

    pub const fn from_numbers_intersection(list: &'static [u8]) -> Self {
        if list.is_empty() {
            return Pattern(0);
        }

        let mut p = (1 << 7) - 1;
        let mut i = 0;

        while i < list.len() {
            p &= TABLE_NUMBER_TO_PATTERN[list[i] as usize].0;
            i += 1;
        }

        Self(p)
    }

    pub const fn possible_numbers(&self) -> &'static [u8] {
        TABLE_LEN_TO_NUMS[(self.0.count_ones() as usize)]
    }

    pub const fn possible_chars(&self) -> Pattern {
        TABLE_LEN_TO_UNION[self.0.count_ones() as usize]
    }

    pub const fn required_chars(&self) -> Pattern {
        TABLE_LEN_TO_INTERSECTION[self.0.count_ones() as usize]
    }

    pub const fn not(&self) -> Self {
        Self((!self.0) & Self::MASK)
    }

    pub fn chars(&self) -> impl Iterator<Item = char> {
        self.bits().map(|b| (b + b'a') as char)
    }

    pub fn bits(&self) -> impl Iterator<Item = u8> {
        self.indicies().map(|b| b as u8)
    }

    pub fn indicies(&self) -> impl Iterator<Item = usize> {
        BitIter::from(self.0)
    }
}

#[derive(Debug)]
pub struct Line {
    patterns: Vec<Pattern>,
    output: Vec<Pattern>,
}

impl Line {
    pub fn output(&self) -> &[Pattern] {
        self.output.as_slice()
    }

    pub fn patterns(&self) -> &[Pattern] {
        self.patterns.as_slice()
    }
}

impl From<(Vec<Pattern>, Vec<Pattern>)> for Line {
    fn from((patterns, output): (Vec<Pattern>, Vec<Pattern>)) -> Self {
        assert_eq!(
            10,
            patterns.len(),
            "advent of code specified that there would be ten patterns"
        );
        assert_eq!(
            4,
            output.len(),
            "advent of code specified that there would be 4 outputs"
        );
        Line { patterns, output }
    }
}

#[derive(Clone)]
pub struct Solver {
    solutions: [Pattern; 7],
    solution: [Pattern; 7],
}

impl Debug for Solver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("Solver");

        for (l, r) in self.solutions.iter().enumerate() {
            let mut tmp = [0u8; 4];
            let name = (b'a' + (l as u8)) as char;
            d.field(name.encode_utf8(&mut tmp), r);
        }

        d.finish()
    }
}

impl Default for Solver {
    fn default() -> Self {
        Self {
            solutions: [Pattern::all(); 7],
            solution: [Pattern(0); 7],
        }
    }
}

impl From<Pattern> for Solver {
    fn from(pattern: Pattern) -> Self {
        let mut s = Solver::default();
        s.add(pattern);
        s
    }
}

impl Solver {
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn add(&mut self, pattern: Pattern) -> Option<Solution> {
        let union = pattern.possible_chars();
        let complete = self.completion_mask();

        for i in pattern.indicies() {
            self.solutions[i].0 &= union.0;
        }

        let intersection = pattern.required_chars();
        for i in pattern.not().indicies() {
            self.solutions[i].0 |= intersection.0;
            self.solutions[i].0 ^= intersection.0;
        }

        if pattern.possible_numbers().len() == 1 {
            for i in pattern.not().indicies() {
                self.solutions[i].0 |= union.0;
                self.solutions[i].0 ^= union.0;
            }
        }

        self.flatten_to(complete);
        self.return_solution_if_done()
    }

    fn return_solution_if_done(&mut self) -> Option<Solution> {
        self.solutions
            .iter()
            .all(|p| p.0.count_ones() == 1)
            .then(|| self.solution())
    }

    fn solution(&mut self) -> Solution {
        for (l, r) in self.solutions.iter().zip(self.solution.iter_mut()) {
            for i in l.indicies() {
                *r = Pattern(1 << i);
            }
        }

        Solution {
            map: &self.solution,
        }
    }

    fn flatten_to(&mut self, mut complete: u8) {
        let mut completed = (self.completion_mask() ^ complete) & Pattern::MASK;

        while completed > 0 {
            for i in BitIter::from(completed) {
                let mask = self.solutions[i].0;
                for sol in self.solutions.iter_mut().filter(|p| p.0 != mask) {
                    sol.0 |= mask;
                    sol.0 ^= mask;
                }
            }
            complete |= completed;
            completed = (self.completion_mask() ^ complete) & Pattern::MASK;
        }
    }

    fn completion_mask(&self) -> u8 {
        self.solutions
            .iter()
            .enumerate()
            .filter(|(_, &p)| p.0.count_ones() == 1)
            .fold(0u8, |l, (i, _)| l | (1 << i))
    }
}

#[derive(Debug)]
pub struct Solution<'a> {
    map: &'a [Pattern; 7],
}

impl<'a> Solution<'a> {
    pub fn solve(&self, pattern: Pattern) -> usize {
        let mut p = 0;
        for i in pattern.indicies() {
            p |= self.map[i].0;
        }
        TABLE_NUMBER_TO_PATTERN
            .iter()
            .position(|&l| l == Pattern(p))
            .expect("should be found")
    }
}

const fn char_index(c: char) -> usize {
    (c as usize) - ('a' as usize)
}

const TABLE_NUMBER_TO_PATTERN: [Pattern; 10] = [
    Pattern::from_char_list(&['a', 'b', 'c', 'e', 'f', 'g']),
    Pattern::from_char_list(&['c', 'f']),
    Pattern::from_char_list(&['a', 'c', 'd', 'e', 'g']),
    Pattern::from_char_list(&['a', 'c', 'd', 'f', 'g']),
    Pattern::from_char_list(&['b', 'c', 'd', 'f']),
    Pattern::from_char_list(&['a', 'b', 'd', 'f', 'g']),
    Pattern::from_char_list(&['a', 'b', 'd', 'e', 'f', 'g']),
    Pattern::from_char_list(&['a', 'c', 'f']),
    Pattern::from_char_list(&['a', 'b', 'c', 'd', 'e', 'f', 'g']),
    Pattern::from_char_list(&['a', 'b', 'c', 'd', 'f', 'g']),
];

const TABLE_LEN_TO_NUMS: &[&[u8]] = &[&[], &[], &[1], &[7], &[4], &[2, 3, 5], &[0, 6, 9], &[8]];
const TABLE_LEN_TO_UNION: [Pattern; 8] = [
    Pattern::from_numbers_union(TABLE_LEN_TO_NUMS[0]),
    Pattern::from_numbers_union(TABLE_LEN_TO_NUMS[1]),
    Pattern::from_numbers_union(TABLE_LEN_TO_NUMS[2]),
    Pattern::from_numbers_union(TABLE_LEN_TO_NUMS[3]),
    Pattern::from_numbers_union(TABLE_LEN_TO_NUMS[4]),
    Pattern::from_numbers_union(TABLE_LEN_TO_NUMS[5]),
    Pattern::from_numbers_union(TABLE_LEN_TO_NUMS[6]),
    Pattern::from_numbers_union(TABLE_LEN_TO_NUMS[7]),
];

const TABLE_LEN_TO_INTERSECTION: [Pattern; 8] = [
    Pattern::from_numbers_intersection(TABLE_LEN_TO_NUMS[0]),
    Pattern::from_numbers_intersection(TABLE_LEN_TO_NUMS[1]),
    Pattern::from_numbers_intersection(TABLE_LEN_TO_NUMS[2]),
    Pattern::from_numbers_intersection(TABLE_LEN_TO_NUMS[3]),
    Pattern::from_numbers_intersection(TABLE_LEN_TO_NUMS[4]),
    Pattern::from_numbers_intersection(TABLE_LEN_TO_NUMS[5]),
    Pattern::from_numbers_intersection(TABLE_LEN_TO_NUMS[6]),
    Pattern::from_numbers_intersection(TABLE_LEN_TO_NUMS[7]),
];

#[derive(Debug, Error)]
#[error("invalid character {bad_character} was found")]
pub struct InvalidCharacter {
    bad_character: char,
}

#[cfg(test)]
mod tests {
    use super::{Pattern, Solver};

    #[test]
    fn test_simple_pattern() {
        let input = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab";
        let mut solver = Solver::default();

        for s in input.split_whitespace() {
            let p: Pattern = s.parse().expect("should be able to parse");
            if let Some(s) = solver.add(p) {
                assert_eq!(5, s.solve("cdfeb".parse().expect("must be able to parse")));
                return;
            }
        }

        panic!("didn't find a solution");
    }
}
