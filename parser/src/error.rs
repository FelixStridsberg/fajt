use std::fmt::Formatter;
use std::{error, fmt};

use fajt_common::io::Error as CommonError;
use fajt_lexer::error::Error as LexerError;
use fajt_lexer::token::Span;

// TODO this macro should expand without debug info for optimized build.
#[macro_export]
macro_rules! error {
    ($expr:expr) => {
        Err(crate::error::Error::with_debug($expr, file!(), (line!(), column!())))
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub struct Error {
    kind: ErrorKind,
    file: Option<&'static str>,
    location: Option<(u32, u32)>,
}

impl Error {
    pub(crate) fn of(kind: ErrorKind) -> Self {
        Error { kind, file: None, location: None }
    }

    pub(crate) fn with_debug(kind: ErrorKind, file: &'static str, location: (u32, u32)) -> Self {
        Error {
            kind, file: Some(file), location: Some(location)
        }
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum ErrorKind {
    EndOfStream,
    LexerError(LexerError),
    SyntaxError(String, Span),
    UnexpectedToken(fajt_lexer::token::Token),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::EndOfStream => write!(f, "End of file reached.")?,
            ErrorKind::LexerError(e) => write!(f, "Lexer error '{}'", e)?,
            ErrorKind::SyntaxError(msg, span) => write!(f, "{}:{:?}", msg, span)?,
            ErrorKind::UnexpectedToken(t) => write!(f, "Unexpected token: {:?}", t)?,
        }

        if let (Some(file), Some((row, col))) = (self.file, self.location) {
            write!(f, " at {}:{}:{}", file, row, col)?;
        }

        Ok(())
    }
}

impl error::Error for Error {}

impl From<LexerError> for Error {
    fn from(lexer_err: LexerError) -> Self {
        Error::of(ErrorKind::LexerError(lexer_err))
    }
}

impl From<CommonError<LexerError>> for Error {
    fn from(error: CommonError<LexerError>) -> Self {
        match error {
            CommonError::EndOfStream => Error::of(ErrorKind::EndOfStream),
            CommonError::ReaderError(lexer_error) => lexer_error.into(),
        }
    }
}
