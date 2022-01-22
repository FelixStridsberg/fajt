use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use std::{error, fmt};

use fajt_ast::{Ident, Span};
use fajt_common::io::Error as CommonError;
use fajt_lexer::error::Error as LexerError;

#[macro_export]
macro_rules! err {
    ($error_kind:expr) => {{
        Err($crate::error::Error::of($error_kind))
    }};
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub struct Error {
    kind: ErrorKind,
}

/// Keeps diagnostics about error location in parser for easy debugging.
#[cfg(debug_assertions)]
#[derive(Debug, PartialEq)]
pub struct InternalDiagnostic {
    pub file: &'static str,
    pub location: (u32, u32),
}

impl Error {
    pub(crate) fn of(kind: ErrorKind) -> Self {
        Error { kind }
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
