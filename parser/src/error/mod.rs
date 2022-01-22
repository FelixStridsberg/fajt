use crate::error::ErrorKind::{
    EndOfStream, ExpectedIdentifier, ForbiddenIdentifier, SyntaxError, UnexpectedIdent,
    UnexpectedToken,
};
use fajt_ast::{Ident, Span};
use fajt_common::io::Error as CommonError;
use fajt_lexer::error::Error as LexerError;
use fajt_lexer::token::{Token, TokenValue};
use std::fmt::Formatter;
use std::{error, fmt};

pub mod emitter;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub struct Error {
    kind: ErrorKind,
    span: Span,
}

#[derive(Debug, PartialEq)]
pub struct Diagnostic {
    pub label: String,
    pub span: Span,
}

impl Error {
    pub(crate) fn lexer_error(error: LexerError, span: Span) -> Self {
        Error {
            kind: ErrorKind::LexerError(error),
            span,
        }
    }

    pub(crate) fn syntax_error(message: String, span: Span) -> Self {
        Error {
            kind: SyntaxError(message),
            span,
        }
    }

    pub(crate) fn unexpected_identifier(ident: Ident) -> Self {
        let span = ident.span.clone();
        Error {
            kind: UnexpectedIdent(ident),
            span,
        }
    }

    pub(crate) fn unexpected_token(token: Token) -> Self {
        let span = token.span.clone();
        Error {
            kind: UnexpectedToken(token.value, None),
            span,
        }
    }

    pub(crate) fn expected_other_token(token: Token, expected: &'static TokenValue) -> Self {
        let span = token.span.clone();
        Error {
            kind: UnexpectedToken(token.value, Some(expected)),
            span,
        }
    }

    pub(crate) fn expected_ident(token: Token) -> Self {
        let span = token.span.clone();
        Error {
            kind: ExpectedIdentifier(token.value),
            span,
        }
    }

    pub(crate) fn forbidden_identifier(identifier: String, span: Span) -> Self {
        Error {
            kind: ForbiddenIdentifier(identifier),
            span,
        }
    }

    pub(crate) fn end_of_stream(pos: usize) -> Self {
        Error {
            kind: EndOfStream,
            span: Span::new(pos, pos),
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
    SyntaxError(String),
    ExpectedIdentifier(TokenValue),
    UnexpectedToken(TokenValue, Option<&'static TokenValue>),
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
            UnexpectedToken(_, None) => "Unexpected token".to_string(),
            UnexpectedToken(token, Some(expected)) => format!(
                "Unexpected token, found `{}`, expected `{}`",
                token.to_string(),
                expected.to_string()
            ),
            ExpectedIdentifier(token) => format!(
                "Unexpected token, found `{}`, expected identifier",
                token.to_string(),
            ),
            _ => return None,
        })
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::EndOfStream => write!(f, "Syntax error: Unexpected end of input")?,
            ErrorKind::LexerError(e) => write!(f, "Lexer error '{}'", e)?,
            ErrorKind::SyntaxError(msg) => write!(f, "Syntax error: {}", msg)?,
            ErrorKind::ExpectedIdentifier(token) | ErrorKind::UnexpectedToken(token, _) => {
                write!(f, "Syntax error: Unexpected token `{}`", token.to_string())?
            }
            ErrorKind::UnexpectedIdent(ident) => {
                write!(f, "Syntax Error: Unexpected identifier `{}`", ident.name)?
            }
            ErrorKind::ForbiddenIdentifier(identifier) => {
                write!(f, "Syntax error: Forbidden identifier `{}`", identifier)?
            }
        }

        Ok(())
    }
}

impl error::Error for Error {}

impl From<LexerError> for Error {
    fn from(error: LexerError) -> Self {
        let span = error.span().clone();
        Error::lexer_error(error, span)
    }
}

impl From<CommonError<LexerError>> for Error {
    fn from(error: CommonError<LexerError>) -> Self {
        match error {
            CommonError::EndOfStream(pos) => Error::end_of_stream(pos),
            CommonError::ReaderError(lexer_error) => lexer_error.into(),
        }
    }
}
