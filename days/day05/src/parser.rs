use std::str::FromStr;

use nom::{
    bytes::complete::tag,
    character::complete::char,
    character::complete::{digit1, space0},
    combinator::{map, map_res},
    sequence::{delimited, separated_pair},
    Finish, IResult,
};

use crate::model::{Arrow, Point};

fn number(input: &str) -> IResult<&str, i32> {
    map_res(digit1, FromStr::from_str)(input)
}

fn point(input: &str) -> IResult<&str, Point> {
    map(separated_pair(number, char(','), number), From::from)(input)
}

fn arrow(input: &str) -> IResult<&str, &str> {
    delimited(space0, tag("->"), space0)(input)
}

fn statement(input: &str) -> IResult<&str, Arrow> {
    map(separated_pair(point, arrow, point), From::from)(input)
}

pub fn parse(input: &str) -> Result<Arrow, nom::error::Error<String>> {
    statement(input.trim())
        .finish()
        .map(|(_, a)| a)
        .map_err(|e| nom::error::Error::new(e.input.to_string(), e.code))
}
