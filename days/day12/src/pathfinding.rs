use std::collections::{HashMap, HashSet};

use rayon::prelude::*;

use crate::parser::{Cave, Parse, Passage};

pub fn count_all_paths<'a, T: SmallCaveVisitor<'a> + Sync>(
    parse: &'a Parse<'a>,
    strategy: T,
) -> usize {
    let mut pathfinder = PathFinder::new(strategy);
    pathfinder.load_paths(parse.passages());
    pathfinder.count_all_paths()
}

pub trait SmallCaveVisitor<'a> {
    fn visit_small_cave<T: CaveVisitor<'a>>(
        name: &'a str,
        seen: &mut HashSet<&'a str>,
        visitor: &T,
    ) -> usize;
}

pub trait CaveVisitor<'a> {
    fn visit_cave(&self, cave: &Cave<'a>, seen: &mut HashSet<&'a str>) -> usize;
}

#[derive(Debug)]
pub struct Part1;

impl<'a> SmallCaveVisitor<'a> for Part1 {
    fn visit_small_cave<T: CaveVisitor<'a>>(
        name: &'a str,
        seen: &mut HashSet<&'a str>,
        visitor: &T,
    ) -> usize {
        if seen.insert(name) {
            let sum = visitor.visit_cave(&Cave::Small(name), seen);
            seen.remove(name);
            sum
        } else {
            0
        }
    }
}

#[derive(Debug)]
pub struct Part2;

impl Part2 {
    const DOUBLE_VISIT: &'static str = "__double_cave__";

    pub fn has_visited_cave_twice(seen: &HashSet<&str>) -> bool {
        seen.contains(Self::DOUBLE_VISIT)
    }

    pub fn set_visited_twice(seen: &mut HashSet<&str>) -> bool {
        seen.insert(Self::DOUBLE_VISIT)
    }

    pub fn remove_visited_twice(seen: &mut HashSet<&str>) -> bool {
        seen.remove(Self::DOUBLE_VISIT)
    }
}

impl<'a> SmallCaveVisitor<'a> for Part2 {
    fn visit_small_cave<T: CaveVisitor<'a>>(
        name: &'a str,
        seen: &mut HashSet<&'a str>,
        visitor: &T,
    ) -> usize {
        if seen.insert(name) {
            let sum = visitor.visit_cave(&Cave::Small(name), seen);
            seen.remove(name);
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
struct PathFinder<'a, T> {
    map: HashMap<Cave<'a>, Vec<Cave<'a>>>,
    small_cave_strategy: T,
}

impl<'a, T: SmallCaveVisitor<'a> + Sync> PathFinder<'a, T> {
    pub fn new(strategy: T) -> Self {
        Self {
            map: HashMap::new(),
            small_cave_strategy: strategy,
        }
    }

    pub fn load_paths(&mut self, passages: &'a [Passage]) {
        for passage in passages {
            self.map
                .entry(passage.from())
                .or_default()
                .push(passage.to());
            self.map
                .entry(passage.to())
                .or_default()
                .push(passage.from());
        }
    }

    pub fn count_all_paths(&self) -> usize {
        self.inner_count_all_paths(&Cave::Start, &mut HashSet::new())
    }

    fn inner_count_all_paths(&self, cave: &Cave<'a>, seen: &mut HashSet<&'a str>) -> usize {
        match cave.clone() {
            Cave::End => 1,
            Cave::Small(name) => T::visit_small_cave(name, seen, self),
            other => self.visit_cave(&other, seen),
        }
    }
}

impl<'a, T: SmallCaveVisitor<'a> + Sync> CaveVisitor<'a> for PathFinder<'a, T> {
    fn visit_cave(&self, cave: &Cave<'a>, seen: &mut HashSet<&'a str>) -> usize {
        self.map
            .get(cave)
            .unwrap()
            .par_iter()
            .filter(|c| !matches!(*c, &Cave::Start))
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
        parser::parse,
        pathfinding::{count_all_paths, Part1, Part2},
    };

    #[test]
    fn test_part_1() {
        let small = parse(include_str!("../input/small.txt")).expect("must be able to parse");
        let medium = parse(include_str!("../input/medium.txt")).expect("must be able to parse");
        let large = parse(include_str!("../input/large.txt")).expect("must be able to parse");

        assert_eq!(10, count_all_paths(&small, Part1));
        assert_eq!(19, count_all_paths(&medium, Part1));
        assert_eq!(226, count_all_paths(&large, Part1));
    }

    #[test]
    fn test_part_2() {
        let small = parse(include_str!("../input/small.txt")).expect("must be able to parse");
        let medium = parse(include_str!("../input/medium.txt")).expect("must be able to parse");
        let large = parse(include_str!("../input/large.txt")).expect("must be able to parse");

        assert_eq!(36, count_all_paths(&small, Part2));
        assert_eq!(103, count_all_paths(&medium, Part2));
        assert_eq!(3509, count_all_paths(&large, Part2));
    }
}
