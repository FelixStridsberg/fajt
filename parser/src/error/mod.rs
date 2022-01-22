use crate::error::ErrorKind::ForbiddenIdentifier;
use crate::UnexpectedToken;
use fajt_ast::{Ident, Span};
use fajt_common::io::Error as CommonError;
use fajt_lexer::error::Error as LexerError;
use fajt_lexer::token::Token;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use std::{error, fmt};

pub mod emitter;

#[deprecated]
#[macro_export]
macro_rules! err {
    ($error_kind:expr) => {{
        Err($crate::error::Error::of(
            $error_kind,
            fajt_ast::Span::empty(),
        ))
    }};
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub struct Error {
    kind: ErrorKind,
    span: Span,
    pub diagnostic: Option<Diagnostic>,
}

#[derive(Debug, PartialEq)]
pub struct Diagnostic {
    pub label: String,
    pub span: Span,
}

impl Error {
    pub(crate) fn of(kind: ErrorKind, span: Span) -> Self {
        Error {
            kind,
            span,
            diagnostic: None,
        }
    }

    pub(crate) fn lexer_error(error: LexerError, span: Span) -> Self {
        Error {
            kind: ErrorKind::LexerError(error),
            span,
            diagnostic: None,
        }
    }

    pub(crate) fn unexpected_token(token: Token) -> Self {
        let span = token.span.clone();
        Error {
            kind: UnexpectedToken(token),
            span,
            diagnostic: None,
        }
    }

    pub(crate) fn forbidden_identifier(identifier: String, span: Span) -> Self {
        Error {
            kind: ForbiddenIdentifier(identifier),
            span,
            diagnostic: None,
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
    ForbiddenIdentifier(String),
}

impl ErrorKind {
    fn get_description(&self) -> Option<String> {
        Some(match self {
            ForbiddenIdentifier(keyword) => {
                format!(
                    "`{}` is not allowed as an identifier in this context",
                    keyword
                )
            }
            UnexpectedToken(_) => "Unexpected token".to_string(),
            _ => return None,
        })
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SyntaxError {
    span: Span,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::EndOfStream => write!(f, "End of file reached.")?,
            ErrorKind::LexerError(e) => write!(f, "Lexer error '{}'", e)?,
            ErrorKind::SyntaxError(msg, span) => write!(f, "{}:{:?}", msg, span)?,
            ErrorKind::UnexpectedToken(token) => write!(
                f,
                "Syntax error: Unexpected token `{}`",
                token.value.to_string()
            )?,
            ErrorKind::UnexpectedIdent(i) => write!(f, "Unexpected identifier: {:?}", i)?,
            ErrorKind::ForbiddenIdentifier(identifier) => {
                write!(f, "Syntax error: Forbidden identifier `{}`", identifier)?
            }
        }

        Ok(())
    }
}

impl error::Error for Error {}

impl From<LexerError> for Error {
    fn from(lexer_err: LexerError) -> Self {
        Error::of(ErrorKind::LexerError(lexer_err), Span::empty())
    }
}

impl From<CommonError<LexerError>> for Error {
    fn from(error: CommonError<LexerError>) -> Self {
        match error {
            CommonError::EndOfStream => Error::of(ErrorKind::EndOfStream, Span::empty()),
            CommonError::ReaderError(lexer_error) => lexer_error.into(),
        }
    }
}
