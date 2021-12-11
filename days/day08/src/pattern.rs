use std::{fmt::Debug, str::FromStr};

use bit_iter::BitIter;
use thiserror::Error;

use crate::tables::{LEN_TO_INTERSECTION, LEN_TO_NUMS, LEN_TO_UNION, NUMBER_TO_PATTERN};

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
        let b = s
            .chars()
            .map(|c| match c {
                'a'..='g' => Ok(1 << char_index(c)),
                bad_character => Err(InvalidCharacter { bad_character }),
            })
            .fold(Ok(0), |l, r| {
                l.and_then(|l| r.map(|r| (l, r))).map(|(l, r)| l | r)
            })?;

        Ok(Self(b))
    }
}

impl ToString for Pattern {
    fn to_string(&self) -> String {
        self.chars().collect()
    }
}

impl std::ops::BitAndAssign for Pattern {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl std::ops::Not for Pattern {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl std::ops::BitOrAssign for Pattern {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl std::ops::BitOr for Pattern {
    type Output = Pattern;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl Pattern {
    pub const MASK: u8 = (1 << 7) - 1;
    pub const ALL: Self = Self(Self::MASK);
    pub const ZERO: Self = Self(0);

    pub const fn new(data: u8) -> Self {
        Self(data)
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
            p |= NUMBER_TO_PATTERN[list[i] as usize].0;
            i += 1;
        }

        Self(p)
    }

    pub const fn from_numbers_intersection(list: &'static [u8]) -> Self {
        if list.is_empty() {
            return Pattern(0);
        }

        let mut p = Pattern::MASK;
        let mut i = 0;

        while i < list.len() {
            p &= NUMBER_TO_PATTERN[list[i] as usize].0;
            i += 1;
        }

        Self(p)
    }

    pub const fn possible_numbers(&self) -> &'static [u8] {
        LEN_TO_NUMS[self.count_ones()]
    }

    pub const fn possible_chars(&self) -> Pattern {
        LEN_TO_UNION[self.count_ones()]
    }

    pub const fn required_chars(&self) -> Pattern {
        LEN_TO_INTERSECTION[self.count_ones()]
    }

    pub const fn not(&self) -> Self {
        Self((!self.0) & Self::MASK)
    }

    pub const fn count_ones(&self) -> usize {
        self.0.count_ones() as usize
    }

    pub const fn is_done(&self) -> bool {
        self.count_ones() == 1
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

#[derive(Debug, Error)]
#[error("invalid character {bad_character} was found")]
pub struct InvalidCharacter {
    bad_character: char,
}

const fn char_index(c: char) -> usize {
    (c as usize) - ('a' as usize)
}
