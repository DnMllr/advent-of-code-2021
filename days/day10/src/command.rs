use std::str::FromStr;

use thiserror::Error;

use crate::delimeter::Delimeter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Command {
    Open(Delimeter),
    Close(Delimeter),
    EndLine,
}

impl Command {
    pub fn stream(input: &str) -> impl Iterator<Item = Result<Self, <Self as FromStr>::Err>> + '_ {
        input.split("").filter(|s| !s.is_empty()).map(|s| s.parse())
    }
}

impl FromStr for Command {
    type Err = CommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 1 {
            return Err(CommandParseError::TooLong);
        }

        use crate::command::Command::*;
        use crate::delimeter::Delimeter::*;

        match s {
            "(" => Ok(Open(Paren)),
            ")" => Ok(Close(Paren)),
            "[" => Ok(Open(Square)),
            "]" => Ok(Close(Square)),
            "{" => Ok(Open(Curly)),
            "}" => Ok(Close(Curly)),
            "<" => Ok(Open(Angle)),
            ">" => Ok(Close(Angle)),
            "\n" => Ok(EndLine),
            x => Err(CommandParseError::UnknownSymbol(x.to_string())),
        }
    }
}

#[derive(Debug, Error)]
pub enum CommandParseError {
    #[error("too much input passed")]
    TooLong,

    #[error("a bad symbol was passed {0}")]
    UnknownSymbol(String),
}
