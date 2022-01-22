use fajt_ast::{Ident, Span};
use fajt_common::io::Error as CommonError;
use fajt_lexer::error::Error as LexerError;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use std::{error, fmt};

pub mod emitter;

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
    pub diagnostic: Option<Diagnostic>,
}

#[derive(Debug, PartialEq)]
pub struct Diagnostic {
    pub label: String,
    pub span: Span,
}

impl Error {
    pub(crate) fn of(kind: ErrorKind) -> Self {
        Error {
            kind,
            diagnostic: None,
        }
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    pub fn get_span(&self) -> &Span {
        match &self.kind {
            ErrorKind::UnexpectedToken(t) => &t.span,
            ErrorKind::UnexpectedIdent(t) => &t.span,
            _ => todo!("Move span to error struct"),
        }
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
            ErrorKind::UnexpectedToken(_) => write!(f, "Syntax error: Unexpected token")?,
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
