use std::fmt::Debug;

use bit_iter::BitIter;

use crate::{pattern::Pattern, tables::NUMBER_TO_PATTERN};

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
            solutions: [Pattern::ALL; 7],
            solution: [Pattern::ZERO; 7],
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
        let complete = self.completion_mask();
        self.propagate_constraints(pattern);
        self.flatten_to(complete);
        self.return_solution_if_done()
    }

    fn propagate_constraints(&mut self, pattern: Pattern) {
        let union = pattern.possible_chars();
        for i in pattern.indicies() {
            self.solutions[i] &= union;
        }

        let intersection = pattern.required_chars();
        for i in pattern.not().indicies() {
            self.solutions[i] &= !intersection;
        }

        // optimization for the single option case
        if pattern.possible_numbers().len() == 1 {
            for i in pattern.not().indicies() {
                self.solutions[i] &= !union;
            }
        }
    }

    fn return_solution_if_done(&mut self) -> Option<Solution> {
        self.solutions
            .iter()
            .all(Pattern::is_done)
            .then(|| self.solution())
    }

    fn solution(&mut self) -> Solution {
        for (l, r) in self.solutions.iter().zip(self.solution.iter_mut()) {
            *r = Pattern::new(
                1 << l
                    .indicies()
                    .next()
                    .expect("solution should only be called when each inner pattern is done"),
            );
        }

        Solution {
            map: &self.solution,
        }
    }

    fn flatten_to(&mut self, mut complete: u8) {
        let mut completed = self.compute_remains_to_be_flattened(complete);

        while completed > 0 {
            for i in BitIter::from(completed) {
                self.flatten_out(i);
            }

            complete |= completed;
            completed = self.compute_remains_to_be_flattened(complete);
        }
    }

    fn compute_remains_to_be_flattened(&self, complete: u8) -> u8 {
        (self.completion_mask() ^ complete) & Pattern::MASK
    }

    fn flatten_out(&mut self, i: usize) {
        let mask = self.solutions[i];

        for sol in self.need_to_be_flattened(mask) {
            *sol &= !mask;
        }
    }

    fn need_to_be_flattened(&mut self, mask: Pattern) -> impl Iterator<Item = &mut Pattern> {
        self.solutions.iter_mut().filter(move |p| *p != &mask)
    }

    fn completion_mask(&self) -> u8 {
        self.solutions
            .iter()
            .enumerate()
            .filter(|(_, &p)| p.is_done())
            .fold(0u8, |l, (i, _)| l | (1 << i))
    }
}

#[derive(Debug)]
pub struct Solution<'a> {
    map: &'a [Pattern; 7],
}

impl<'a> Solution<'a> {
    pub fn solve(&self, pattern: Pattern) -> usize {
        let mapped = self.map_pattern(pattern);
        NUMBER_TO_PATTERN
            .iter()
            .position(|&l| l == mapped)
            .expect("should be found")
    }

    fn map_pattern(&self, pattern: Pattern) -> Pattern {
        let mut p = Pattern::ZERO;

        for i in pattern.indicies() {
            p |= self.map[i];
        }

        p
    }
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
