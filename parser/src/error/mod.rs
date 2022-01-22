use fajt_ast::{Ident, Span};
use fajt_common::io::Error as CommonError;
use fajt_lexer::error::Error as LexerError;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use std::{error, fmt};
use fajt_lexer::token::Token;
use fajt_lexer::token_matches;
use crate::UnexpectedToken;

pub mod emitter;

#[deprecated]
#[macro_export]
macro_rules! err {
    ($error_kind:expr) => {{
        Err($crate::error::Error::of($error_kind, fajt_ast::Span::empty()))
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

    pub(crate) fn unexpected_token(token: Token) -> Self {
        let span = token.span.clone();
        Error {
            kind: UnexpectedToken(token),
            span,
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
