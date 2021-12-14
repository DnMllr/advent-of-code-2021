use std::sync::atomic::{AtomicU16, Ordering};

use dashmap::DashMap;
use fxhash::FxBuildHasher;
use nom::{
    character::complete::{alpha1, char},
    error::Error,
    sequence::separated_pair,
    Finish, IResult,
};
use rayon::{iter::ParallelIterator, str::ParallelString};
use thiserror::Error;

#[derive(Debug)]
pub struct Parse {
    passages: Vec<Passage>,
}

impl Parse {
    pub fn passages(&self) -> &[Passage] {
        self.passages.as_slice()
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Cave {
    Large(u16),
    Small(u16),
    Start,
    End,
}

#[derive(Debug)]
pub struct Passage {
    from: Cave,
    to: Cave,
}

impl Passage {
    pub fn from(&self) -> Cave {
        self.from.clone()
    }

    pub fn to(&self) -> Cave {
        self.to.clone()
    }
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("failed to parse input with error {0}")]
    ParsingFailure(Error<String>),
}

#[derive(Debug, Default)]
pub struct Parser<'a> {
    map: DashMap<&'a str, u16, FxBuildHasher>,
    code: AtomicU16,
}

impl<'a> Parser<'a> {
    pub fn parse(input: &'a str) -> Result<Parse, ParseError> {
        Self::default().parse_str(input)
    }

    pub fn parse_str(&mut self, input: &'a str) -> Result<Parse, ParseError> {
        let data: Result<Vec<Passage>, ParseError> = input
            .par_lines()
            .map(Self::passage)
            .map(|s| {
                s.finish()
                    .map(|(_, (from, to))| Passage {
                        from: self.parse_cave(from),
                        to: self.parse_cave(to),
                    })
                    .map_err(|e| ParseError::ParsingFailure(Error::new(e.to_string(), e.code)))
            })
            .collect();

        Ok(Parse { passages: data? })
    }

    fn parse_cave(&self, input: &'a str) -> Cave {
        let code = *self
            .map
            .entry(input)
            .or_insert_with(|| self.code.fetch_add(1, Ordering::Relaxed));

        match input.trim() {
            "start" => Cave::Start,
            "end" => Cave::End,
            s => {
                if s.chars().all(|c| c.is_uppercase()) {
                    Cave::Large(code)
                } else {
                    Cave::Small(code)
                }
            }
        }
    }

    fn cave(input: &str) -> IResult<&str, &str> {
        alpha1(input)
    }

    fn passage(input: &str) -> IResult<&str, (&str, &str)> {
        separated_pair(Self::cave, char('-'), Self::cave)(input)
    }
}
