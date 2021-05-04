use crate::token::Span;
use std::fmt::Formatter;
use std::{error, fmt};

use fajt_common::io::Error as CommonError;

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
#[non_exhaustive]
pub enum ErrorKind {
    InvalidOrUnexpectedToken(Span),
    EndOfFile,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::EndOfFile => write!(f, "End of file reached."),
            ErrorKind::InvalidOrUnexpectedToken(p) => {
                write!(f, "Invalid or unexpected token at {}:{}", p.start, p.end)
            }
        }
    }
}

impl error::Error for Error {}

impl From<CommonError<()>> for Error {
    fn from(error: CommonError<()>) -> Self {
        match error {
            CommonError::EndOfStream => Error::of(ErrorKind::EndOfFile),
            CommonError::ReaderError(_) => unreachable!(),
        }
    }
}
