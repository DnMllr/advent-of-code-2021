use fxhash::{FxHashMap, FxHashSet};
use rayon::prelude::*;

use crate::parser::{Cave, Parse, Passage};

pub fn count_all_paths<T: SmallCaveVisitor + Sync>(parse: &Parse, strategy: T) -> usize {
    let mut pathfinder = PathFinder::new(strategy);
    pathfinder.load_paths(parse.passages());
    pathfinder.count_all_paths()
}

pub trait SmallCaveVisitor {
    fn visit_small_cave<T: CaveVisitor>(name: u16, seen: &mut FxHashSet<u16>, visitor: &T)
        -> usize;
}

pub trait CaveVisitor {
    fn visit_cave(&self, cave: &Cave, seen: &mut FxHashSet<u16>) -> usize;
}

#[derive(Debug)]
pub struct Part1;

impl SmallCaveVisitor for Part1 {
    fn visit_small_cave<T: CaveVisitor>(
        name: u16,
        seen: &mut FxHashSet<u16>,
        visitor: &T,
    ) -> usize {
        if seen.insert(name) {
            let sum = visitor.visit_cave(&Cave::Small(name), seen);
            seen.remove(&name);
            sum
        } else {
            0
        }
    }
}

#[derive(Debug)]
pub struct Part2;

impl Part2 {
    // assume that we're never going to have more caves than u16::MAX - 1.
    const DOUBLE_VISIT_SENTINEL: u16 = u16::MAX;

    pub fn has_visited_cave_twice(seen: &FxHashSet<u16>) -> bool {
        seen.contains(&Self::DOUBLE_VISIT_SENTINEL)
    }

    pub fn set_visited_twice(seen: &mut FxHashSet<u16>) -> bool {
        seen.insert(Self::DOUBLE_VISIT_SENTINEL)
    }

    pub fn remove_visited_twice(seen: &mut FxHashSet<u16>) -> bool {
        seen.remove(&Self::DOUBLE_VISIT_SENTINEL)
    }
}

impl SmallCaveVisitor for Part2 {
    fn visit_small_cave<T: CaveVisitor>(
        name: u16,
        seen: &mut FxHashSet<u16>,
        visitor: &T,
    ) -> usize {
        if seen.insert(name) {
            let sum = visitor.visit_cave(&Cave::Small(name), seen);
            seen.remove(&name);
            sum
        } else if !Self::has_visited_cave_twice(seen) {
            Self::set_visited_twice(seen);
            let sum = visitor.visit_cave(&Cave::Small(name), seen);
            Self::remove_visited_twice(seen);
            sum
        } else {
            0
        }
    }
}

#[derive(Debug)]
struct PathFinder<T> {
    map: FxHashMap<Cave, Vec<Cave>>,
    small_cave_strategy: T,
}

impl<T: SmallCaveVisitor + Sync> PathFinder<T> {
    pub fn new(strategy: T) -> Self {
        Self {
            map: FxHashMap::default(),
            small_cave_strategy: strategy,
        }
    }

    pub fn load_paths(&mut self, passages: &[Passage]) {
        for passage in passages {
            self.load_passage(passage.from(), passage.to());
            self.load_passage(passage.to(), passage.from());
        }
    }

    fn load_passage(&mut self, from: Cave, to: Cave) {
        if !matches!(to, Cave::Start) {
            self.map.entry(from).or_default().push(to);
        }
    }

    pub fn count_all_paths(&self) -> usize {
        self.inner_count_all_paths(&Cave::Start, &mut FxHashSet::default())
    }

    fn inner_count_all_paths(&self, cave: &Cave, seen: &mut FxHashSet<u16>) -> usize {
        match cave.clone() {
            Cave::End => 1,
            Cave::Small(name) => T::visit_small_cave(name, seen, self),
            other => self.visit_cave(&other, seen),
        }
    }
}

impl<T: SmallCaveVisitor + Sync> CaveVisitor for PathFinder<T> {
    fn visit_cave(&self, cave: &Cave, seen: &mut FxHashSet<u16>) -> usize {
        self.map
            .get(cave)
            .unwrap()
            .par_iter()
            .map_init(
                || seen.clone(),
                |seen, c| self.inner_count_all_paths(c, seen),
            )
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        parser::Parser,
        pathfinding::{count_all_paths, Part1, Part2},
    };

    #[test]
    fn test_part_1() {
        let mut parser = Parser::default();
        let small = parser
            .parse_str(include_str!("../input/small.txt"))
            .expect("must be able to parse");
        let medium = parser
            .parse_str(include_str!("../input/medium.txt"))
            .expect("must be able to parse");
        let large = parser
            .parse_str(include_str!("../input/large.txt"))
            .expect("must be able to parse");

        assert_eq!(10, count_all_paths(&small, Part1));
        assert_eq!(19, count_all_paths(&medium, Part1));
        assert_eq!(226, count_all_paths(&large, Part1));
    }

    #[test]
    fn test_part_2() {
        let small =
            Parser::parse(include_str!("../input/small.txt")).expect("must be able to parse");
        let medium =
            Parser::parse(include_str!("../input/medium.txt")).expect("must be able to parse");
        let large =
            Parser::parse(include_str!("../input/large.txt")).expect("must be able to parse");

        assert_eq!(36, count_all_paths(&small, Part2));
        assert_eq!(103, count_all_paths(&medium, Part2));
        assert_eq!(3509, count_all_paths(&large, Part2));
    }
}
