use crate::token::Keyword;
use crate::{EndOfFile, InvalidOrUnexpectedToken, Token};
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

impl Error {
    pub fn unexpected_end_of_file(pos: usize) -> Self {
        Error {
            span: Span::new(pos, pos),
            kind: EndOfFile,
        }
    }

    pub fn invalid_or_unexpected_token(token: Token) -> Self {
        Error {
            span: token.span.clone(),
            kind: InvalidOrUnexpectedToken(token),
        }
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
            CommonError::EndOfStream(pos) => Error::unexpected_end_of_file(pos),
            CommonError::ReaderError(_) => unreachable!(),
        }
    }
}
