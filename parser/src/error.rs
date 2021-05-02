use crate::error::ErrorKind::LexerError;
use std::fmt::Formatter;
use std::{error, fmt};

pub type Result<T> = std::result::Result<T, Error>;

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
    EndOfStream,
    LexerError(fajt_lexer::error::Error),
    UnexpectedToken(fajt_lexer::token::Token),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::EndOfStream => write!(f, "End of file reached."),
            ErrorKind::LexerError(e) => write!(f, "Lexer error '{}'", e),
            ErrorKind::UnexpectedToken(t) => write!(f, "Unexpected token: {:?}", t),
        }
    }
}

impl error::Error for Error {}

impl From<fajt_lexer::error::Error> for Error {
    fn from(lexer_err: fajt_lexer::error::Error) -> Self {
        Error::of(LexerError(lexer_err))
    }
}
