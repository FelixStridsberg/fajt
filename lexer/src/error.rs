use crate::error::ErrorKind::{ForbiddenIdentifier, UnrecognizedCodePoint};
use crate::token::Keyword;
use crate::{EndOfStream, InvalidOrUnexpectedToken, Token};
use fajt_ast::Span;
use fajt_common::io::Error as CommonError;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use std::{error, fmt};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Error {
    span: Span,
    kind: ErrorKind,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ErrorKind {
    InvalidOrUnexpectedToken(Token),
    ForbiddenIdentifier(Keyword),
    UnrecognizedCodePoint(char),
    EndOfStream,
}

impl Error {
    pub fn unexpected_end_of_stream(pos: usize) -> Self {
        Error {
            span: Span::new(pos, pos),
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
            EndOfStream => write!(f, "Unexpected end of stream"),
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
        }
    }
}

impl error::Error for Error {}

impl From<CommonError<()>> for Error {
    fn from(error: CommonError<()>) -> Self {
        match error {
            CommonError::EndOfStream(pos) => Error::unexpected_end_of_stream(pos),
            CommonError::ReaderError(_) => unreachable!(),
        }
    }
}
