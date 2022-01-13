use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use std::{error, fmt};

use fajt_ast::{Ident, Span};
use fajt_common::io::Error as CommonError;
use fajt_lexer::error::Error as LexerError;

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! err {
    ($expr:expr) => {
        Err($crate::error::Error::with_debug(
            $expr,
            file!(),
            (line!(), column!()),
        ))
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! err {
    ($expr:expr) => {
        Err($crate::error::Error::of($expr))
    };
}

pub type Result<T> = std::result::Result<T, Error>;

/// Similar trait to bool.then, but handles closures returning `Result`.
pub trait ThenTry {
    fn then_try<T, F>(self, f: F) -> Result<Option<T>>
    where
        F: FnOnce() -> Result<T>;
}

impl ThenTry for bool {
    fn then_try<T, F>(self, f: F) -> Result<Option<T>>
    where
        F: FnOnce() -> Result<T>,
    {
        if self {
            f().map(Some)
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Error {
    kind: ErrorKind,
    file: Option<&'static str>,
    location: Option<(u32, u32)>,
}

impl Error {
    pub(crate) fn of(kind: ErrorKind) -> Self {
        Error {
            kind,
            file: None,
            location: None,
        }
    }

    #[cfg(debug_assertions)]
    pub(crate) fn with_debug(kind: ErrorKind, file: &'static str, location: (u32, u32)) -> Self {
        Error {
            kind,
            file: Some(file),
            location: Some(location),
        }
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ErrorKind {
    EndOfStream,
    LexerError(LexerError),
    SyntaxError(String, Span),
    UnexpectedToken(fajt_lexer::token::Token),
    UnexpectedIdent(Ident),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::EndOfStream => write!(f, "End of file reached.")?,
            ErrorKind::LexerError(e) => write!(f, "Lexer error '{}'", e)?,
            ErrorKind::SyntaxError(msg, span) => write!(f, "{}:{:?}", msg, span)?,
            ErrorKind::UnexpectedToken(t) => write!(f, "Unexpected token: {:?}", t)?,
            ErrorKind::UnexpectedIdent(i) => write!(f, "Unexpected identifier: {:?}", i)?,
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
