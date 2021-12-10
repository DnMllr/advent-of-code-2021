use std::str::FromStr;

use nom::{
    character::complete::{alpha1, char, line_ending, space1},
    combinator::{map, map_res},
    error::Error,
    multi::{count, many0, separated_list0, separated_list1},
    sequence::{separated_pair, terminated},
    Finish, IResult,
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::pattern::Pattern;

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
            "advent of code specified that there would be 10 patterns"
        );
        assert_eq!(
            4,
            output.len(),
            "advent of code specified that there would be 4 outputs"
        );
        Line { patterns, output }
    }
}

impl FromStr for Line {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        line(s)
            .finish()
            .map(|(_, l)| l)
            .map_err(|e| Error::new(e.input.to_string(), e.code))
    }
}

#[derive(Debug)]
pub struct Input {
    lines: Vec<Line>,
}

impl FromStr for Input {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(s)
            .map(|lines| Input { lines })
            .map_err(|e| Error::new(e.input.to_string(), e.code))
    }
}

impl Input {
    pub fn new(lines: Vec<Line>) -> Self {
        Self { lines }
    }

    pub fn outputs(&self) -> impl Iterator<Item = &[Pattern]> {
        self.lines().map(Line::output)
    }

    pub fn par_outputs(&self) -> impl ParallelIterator<Item = &[Pattern]> {
        self.par_lines().map(Line::output)
    }

    pub fn lines(&self) -> impl Iterator<Item = &Line> {
        self.lines.iter()
    }

    pub fn par_lines(&self) -> impl ParallelIterator<Item = &Line> {
        self.lines.par_iter()
    }
}

fn signal_pattern(input: &str) -> IResult<&str, Pattern> {
    map_res(alpha1, |s: &str| s.parse())(input)
}

fn bar(input: &str) -> IResult<&str, char> {
    terminated(char('|'), space1)(input)
}

fn signal_patterns(input: &str) -> IResult<&str, Vec<Pattern>> {
    count(terminated(signal_pattern, space1), 10)(input)
}

fn output(input: &str) -> IResult<&str, Vec<Pattern>> {
    separated_list1(space1, signal_pattern)(input)
}

fn line(input: &str) -> IResult<&str, Line> {
    map(separated_pair(signal_patterns, bar, output), Line::from)(input)
}

fn lines(input: &str) -> IResult<&str, Vec<Line>> {
    separated_list0(many0(line_ending), line)(input)
}

pub fn parse(input: &str) -> Result<Vec<Line>, Error<&str>> {
    lines(input).finish().map(|(_, lines)| lines)
}
