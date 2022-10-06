use crate::error::ErrorKind::{ForbiddenIdentifier, SyntaxError, UnrecognizedCodePoint};
use crate::token::Keyword;
use crate::{EndOfStream, InvalidOrUnexpectedToken, Token};
use fajt_ast::Span;
use fajt_common::io::char_reader::Error as CharReaderError;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use std::{error, fmt};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Error {
    span: Span,
    kind: ErrorKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ErrorKind {
    InvalidOrUnexpectedToken(Token),
    ForbiddenIdentifier(Keyword),
    UnrecognizedCodePoint(char),
    SyntaxError(String),
    EndOfStream,
}

impl Error {
    pub fn unexpected_end_of_stream() -> Self {
        Error {
            span: Span::empty(),
            kind: EndOfStream,
        }
    }

    pub fn invalid_or_unexpected_token(token: Token) -> Self {
        Error {
            span: token.span.clone(),
            kind: InvalidOrUnexpectedToken(token),
        }
    }

    pub fn unrecognized_code_point<S: Into<Span>>(char: char, span: S) -> Self {
        Error {
            span: span.into(),
            kind: UnrecognizedCodePoint(char),
        }
    }

    pub fn syntax_error<S: Into<Span>>(error: String, span: S) -> Self {
        Error {
            span: span.into(),
            kind: SyntaxError(error),
        }
    }

    pub fn span(&self) -> &Span {
        &self.span
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.kind {
            EndOfStream => write!(f, "Unexpected end of input"),
            InvalidOrUnexpectedToken(t) => {
                write!(f, "Invalid or unexpected token {:?}", t)
            }
            ForbiddenIdentifier(k) => {
                write!(
                    f,
                    "Keyword '{:?}' is not allowed as identifier in this context.",
                    k
                )
            }
            UnrecognizedCodePoint(char) => {
                write!(f, "Unknown code point {char}")
            }
            SyntaxError(error) => {
                write!(f, "Syntax Error: {error}")
            }
        }
    }
}

impl error::Error for Error {}

impl From<CharReaderError> for Error {
    fn from(error: CharReaderError) -> Self {
        match error {
            CharReaderError::EndOfStream => Error::unexpected_end_of_stream(),
        }
    }
}

impl From<&CharReaderError> for Error {
    fn from(error: &CharReaderError) -> Self {
        match error {
            CharReaderError::EndOfStream => Error::unexpected_end_of_stream(),
        }
    }
}
