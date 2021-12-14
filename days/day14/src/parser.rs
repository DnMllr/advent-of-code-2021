use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, space0},
    combinator::map_res,
    multi::{many0, separated_list1},
    sequence::{preceded, separated_pair, terminated},
    AsBytes, Finish, IResult,
};
use thiserror::Error;

pub struct Input<'a> {
    pub polymer: &'a [u8],
    pub replacement_rule: HashMap<(u8, u8), u8>,
}

pub fn parse(input: &str) -> Result<Input, ParseError> {
    full_file(input)
        .finish()
        .map(|(_, (polymer, replacements))| Input {
            polymer,
            replacement_rule: replacements.into_iter().collect(),
        })
        .map_err(|e| ParseError::ParseError(nom::error::Error::new(e.input.to_string(), e.code)))
}

type File<'a> = (&'a [u8], Vec<((u8, u8), u8)>);

fn full_file(input: &str) -> IResult<&str, File> {
    separated_pair(
        polymer,
        many0(line_ending),
        separated_list1(line_ending, replacement_rule),
    )(input)
}

fn polymer(input: &str) -> IResult<&str, &[u8]> {
    map_res(alpha1, |r: &str| {
        r.is_ascii()
            .then(|| r.as_bytes())
            .ok_or(ParseError::NonAsciiInput)
    })(input)
}

fn left_hand_side(input: &str) -> IResult<&str, (u8, u8)> {
    map_res(alpha1, |r: &str| {
        r.is_ascii()
            .then(|| r.as_bytes())
            .ok_or(ParseError::NonAsciiInput)
            .and_then(|s| {
                (s.len() == 2)
                    .then(|| s.as_bytes())
                    .map(|bytes| (bytes[0], bytes[1]))
                    .ok_or(ParseError::TooManyLHSSymbols)
            })
    })(input)
}

fn right_hand_side(input: &str) -> IResult<&str, u8> {
    map_res(alpha1, |r: &str| {
        r.is_ascii()
            .then(|| r.as_bytes())
            .ok_or(ParseError::NonAsciiInput)
            .and_then(|s| {
                (s.len() == 1)
                    .then(|| s.as_bytes().first().copied())
                    .flatten()
                    .ok_or(ParseError::WrongNumberOfRHSSymbols)
            })
    })(input)
}

fn arrow(input: &str) -> IResult<&str, &str> {
    terminated(preceded(space0, tag("->")), space0)(input)
}

fn replacement_rule(input: &str) -> IResult<&str, ((u8, u8), u8)> {
    separated_pair(left_hand_side, arrow, right_hand_side)(input)
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("non ascii input")]
    NonAsciiInput,

    #[error("too many symbols on the left hand side")]
    TooManyLHSSymbols,

    #[error("there should be only one symbol on the right hand side")]
    WrongNumberOfRHSSymbols,

    #[error("parse error {0}")]
    ParseError(nom::error::Error<String>),
}
