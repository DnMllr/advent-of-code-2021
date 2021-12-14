use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, line_ending},
    combinator::{map, map_res},
    error::Error,
    multi::{many1, separated_list1},
    sequence::{preceded, separated_pair},
    Finish, IResult,
};

use crate::model::{Fold, Input, Point};

pub fn parse(input: &str) -> Result<Input, Error<&str>> {
    all_input(input).finish().map(|(_, i)| i)
}

fn number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |d: &str| d.parse())(input)
}

fn point(input: &str) -> IResult<&str, Point> {
    map(separated_pair(number, char(','), number), Point::from)(input)
}

fn points(input: &str) -> IResult<&str, Vec<Point>> {
    separated_list1(line_ending, point)(input)
}

fn fold(input: &str) -> IResult<&str, Fold> {
    map(
        separated_pair(alt((char('x'), char('y'))), char('='), number),
        |(dim, num)| match dim {
            'x' => Fold::X(num),
            'y' => Fold::Y(num),
            other => panic!("unexpected dimension {}", other), // TODO(dan) remove this panic
        },
    )(input)
}

fn fold_line(input: &str) -> IResult<&str, Fold> {
    preceded(tag("fold along "), fold)(input)
}

fn folds(input: &str) -> IResult<&str, Vec<Fold>> {
    separated_list1(line_ending, fold_line)(input)
}

fn all_input(input: &str) -> IResult<&str, Input> {
    map(
        separated_pair(points, many1(line_ending), folds),
        Input::from,
    )(input)
}
