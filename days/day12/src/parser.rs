use fxhash::FxHashMap;
use nom::{
    character::complete::{alpha1, char, line_ending},
    error::Error,
    multi::separated_list1,
    sequence::separated_pair,
    Finish, IResult,
};
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
    map: FxHashMap<&'a str, u16>,
    code: u16,
}

impl<'a> Parser<'a> {
    pub fn parse(input: &'a str) -> Result<Parse, ParseError> {
        Self::default().parse_str(input)
    }

    pub fn parse_str(&mut self, input: &'a str) -> Result<Parse, ParseError> {
        separated_list1(line_ending, Self::passage)(input)
            .finish()
            .map(|(_, passages)| Parse {
                passages: self.build_passages(passages),
            })
            .map_err(|e| ParseError::ParsingFailure(Error::new(e.to_string(), e.code)))
    }

    fn build_passages(&mut self, data: Vec<(&'a str, &'a str)>) -> Vec<Passage> {
        data.into_iter()
            .map(|(from, to)| Passage {
                from: self.parse_cave(from),
                to: self.parse_cave(to),
            })
            .collect()
    }

    fn parse_cave(&mut self, input: &'a str) -> Cave {
        let code = *self.map.entry(input).or_insert_with(|| {
            self.code += 1;
            self.code
        });

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
