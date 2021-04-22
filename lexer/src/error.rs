use std::fmt::Formatter;
use std::{error, fmt};
use crate::token::FilePosition;

#[derive(Debug, PartialEq)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    pub(crate) fn of(kind: ErrorKind) -> Self {
        Error { kind }
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    InvalidOrUnexpectedToken(FilePosition),
    EndOfFile,

    /// Hint to avoid relying on exhaustive match which would break if more errors are added.
    #[doc(hidden)]
    __NonExhaustive,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::EndOfFile => write!(f, "End of file reached."),
            ErrorKind::InvalidOrUnexpectedToken(p) => write!(f, "Invalid or unexpected token at {}", p),
            ErrorKind::__NonExhaustive => unreachable!(),
        }
    }
}

impl error::Error for Error {}
