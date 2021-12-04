use std::{num::ParseIntError, str::FromStr};

use thiserror::Error;

pub enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}

struct CommandParser<'a, I: Iterator<Item = &'a str>> {
    iter: I,
}

impl<'a, I: Iterator<Item = &'a str>> CommandParser<'a, I> {
    fn new(iter: I) -> Self {
        Self { iter }
    }

    fn next(&mut self) -> Result<&'a str, CommandParseError> {
        self.iter.next().ok_or(CommandParseError::IncompleteCommand)
    }

    fn parse_amount(&mut self) -> Result<i32, CommandParseError> {
        self.next()?.parse().map_err(CommandParseError::BadInteger)
    }

    fn parse(&mut self) -> Result<Command, CommandParseError> {
        let cmd = match self.next()? {
            "forward" => Command::Forward,
            "up" => Command::Up,
            "down" => Command::Down,
            unknown => return Err(CommandParseError::UnknownVerb(unknown.into())),
        };

        self.parse_amount().map(cmd)
    }
}

impl FromStr for Command {
    type Err = CommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        CommandParser::new(s.split_whitespace()).parse()
    }
}

#[derive(Debug, Error)]
pub enum CommandParseError {
    #[error("{0} is not a valid command verb")]
    UnknownVerb(String),

    #[error("failed to parse command amount with error {0}")]
    BadInteger(ParseIntError),

    #[error("incomplete command")]
    IncompleteCommand,
}
