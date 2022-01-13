use crate::token::Keyword;
use crate::Token;
use fajt_common::io::Error as CommonError;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use std::{error, fmt};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    pub fn of(kind: ErrorKind) -> Self {
        Error { kind }
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ErrorKind {
    InvalidOrUnexpectedToken(Token),
    ForbiddenIdentifier(Keyword),
    EndOfFile,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::EndOfFile => write!(f, "End of file reached."),
            ErrorKind::InvalidOrUnexpectedToken(t) => {
                write!(f, "Invalid or unexpected token {:?}", t)
            }
            ErrorKind::ForbiddenIdentifier(k) => {
                write!(
                    f,
                    "Keyword '{:?}' is not allowed as identifier in this context.",
                    k
                )
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
