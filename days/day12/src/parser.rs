use nom::{
    character::complete::{alpha1, char, line_ending},
    combinator::map,
    error::Error,
    multi::separated_list1,
    sequence::separated_pair,
    Finish, IResult,
};
use thiserror::Error;

#[derive(Debug)]
pub struct Parse<'a> {
    passages: Vec<Passage<'a>>,
}

impl<'a> Parse<'a> {
    pub fn passages(&self) -> &[Passage<'a>] {
        self.passages.as_slice()
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Cave<'a> {
    Large(&'a str),
    Small(&'a str),
    Start,
    End,
}

impl<'a> Cave<'a> {
    pub fn from_str(input: &'a str) -> Self {
        match input {
            "start" => Cave::Start,
            "end" => Cave::End,
            s => {
                if s.chars().all(|c| c.is_uppercase()) {
                    Cave::Large(s)
                } else {
                    Cave::Small(s)
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Passage<'a> {
    from: Cave<'a>,
    to: Cave<'a>,
}

impl<'a> Passage<'a> {
    pub fn from(&self) -> Cave {
        self.from.clone()
    }

    pub fn to(&self) -> Cave {
        self.to.clone()
    }
}

#[derive(Debug, Error)]
pub enum ParseError<'a> {
    #[error("failed to parse input with error {0}")]
    ParsingFailure(Error<&'a str>),
}

pub fn parse(input: &str) -> Result<Parse, ParseError> {
    separated_list1(line_ending, passage)(input)
        .finish()
        .map(|(_, passages)| Parse { passages })
        .map_err(ParseError::ParsingFailure)
}

fn cave(input: &str) -> IResult<&str, Cave> {
    map(alpha1, Cave::from_str)(input)
}

fn passage(input: &str) -> IResult<&str, Passage> {
    map(separated_pair(cave, char('-'), cave), |(from, to)| {
        Passage { from, to }
    })(input)
}
