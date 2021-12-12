use std::error::Error;

use thiserror::Error;

use crate::{command::Command, delimeter::Delimeter};

pub struct Interpreter<E: Error, S: Iterator<Item = Result<Command, E>>> {
    stream: S,
    line: usize,
    column: usize,
    stack: Vec<Delimeter>,
    state: Option<InterpreterError<E>>,
}

impl<E: Error, S: Iterator<Item = Result<Command, E>>> Iterator for Interpreter<E, S> {
    type Item = Result<(), InterpreterError<E>>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.stream.next()?;
            self.column += 1;
            match (self.state.take(), next) {
                (state, Ok(Command::EndLine)) => return self.handle_line_ending(state),
                (Some(e), _) => self.state = Some(e),
                (None, Ok(Command::Close(c))) => self.handle_close(c),
                (None, Ok(Command::Open(o))) => self.stack.push(o),
                (None, Err(e)) => {
                    self.state = Some(InterpreterError::StreamError(e, self.line, self.column))
                }
            };
        }
    }
}

impl<E: Error, S: Iterator<Item = Result<Command, E>>> Interpreter<E, S> {
    pub fn new(stream: S) -> Self {
        Self {
            stream,
            line: 0,
            column: 0,
            stack: Vec::new(),
            state: None,
        }
    }

    pub fn handle_line_ending(
        &mut self,
        state: Option<InterpreterError<E>>,
    ) -> Option<Result<(), InterpreterError<E>>> {
        let res = state.map(Err).or_else(|| {
            if self.stack.is_empty() {
                None
            } else {
                self.stack.reverse();
                Some(Err(InterpreterError::LineIncomplete(
                    self.stack.clone(),
                    self.line,
                    self.column,
                )))
            }
        });

        self.line += 1;
        self.column = 0;
        self.stack.clear();

        res
    }

    pub fn handle_close(&mut self, c: Delimeter) {
        if let Some(o) = self.stack.pop() {
            if o != c {
                self.state = Some(InterpreterError::MismatchClosed(
                    o,
                    c,
                    self.line,
                    self.column,
                ));
            }
        } else {
            self.state = Some(InterpreterError::ClosedWhileBalanced(
                c,
                self.line,
                self.column,
            ));
        }
    }
}

#[derive(Debug, Error)]
pub enum InterpreterError<E: Error> {
    #[error("[{1}:{2}] error occured in the stream: {0}.")]
    StreamError(E, usize, usize),

    #[error("[{1}:{2}] line was incomplete. Expected: {0:?}.")]
    LineIncomplete(Vec<Delimeter>, usize, usize),

    #[error("[{1}:{2}] saw a unexpected closing delimeter: {0}.")]
    ClosedWhileBalanced(Delimeter, usize, usize),

    #[error("[{2}:{3}] mismatched closing delimeter: expected {0} but saw {1}.")]
    MismatchClosed(Delimeter, Delimeter, usize, usize),
}

impl<E: Error> InterpreterError<E> {
    pub fn score(&self) -> Option<usize> {
        match self {
            InterpreterError::LineIncomplete(f, _, _) => Some(
                f.iter()
                    .fold(0, |score, d| (score * 5) + d.autocomplete_score()),
            ),
            InterpreterError::ClosedWhileBalanced(_, _, _) => Some(0),
            InterpreterError::MismatchClosed(_, f, _, _) => Some(f.syntax_score()),
            _ => None,
        }
    }

    pub fn is_corrupted(&self) -> bool {
        matches!(self, InterpreterError::MismatchClosed(_, _, _, _))
    }

    pub fn is_incomplete(&self) -> bool {
        matches!(self, InterpreterError::LineIncomplete(_, _, _))
    }
}
