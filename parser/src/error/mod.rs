use crate::error::ErrorKind::{
    ArrowFunctionNotAllowed, EndOfStream, ExpectedIdentifier, ForbiddenIdentifier,
    InitializedNameNotAllowed, SyntaxError, UnexpectedIdent, UnexpectedToken,
};
use crate::LexerErrorKind;
use fajt_ast::{Expr, Ident, Span, Spanned};
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
    pub(crate) fn lexer_error(error: LexerError) -> Self {
        match &error.kind() {
            LexerErrorKind::EndOfStream => Error {
                span: Span::empty(),
                kind: EndOfStream,
            },
            LexerErrorKind::SyntaxError(error_msg) => Error {
                span: error.span().clone(),
                kind: SyntaxError(error_msg.clone()),
            },
            _ => Error {
                span: error.span().clone(),
                kind: ErrorKind::LexerError(error),
            },
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

    pub(crate) fn arrow_function_not_allowed(expr: Expr) -> Self {
        Error {
            span: expr.span().clone(),
            kind: ArrowFunctionNotAllowed(expr),
        }
    }

    pub(crate) fn initialized_name_not_allowed(span: Span) -> Self {
        Error {
            span,
            kind: InitializedNameNotAllowed,
        }
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    pub(crate) fn span(&self) -> &Span {
        &self.span
    }

    pub(crate) fn from_kind(kind: ErrorKind, span: Span) -> Error {
        Error { kind, span }
    }

    pub(crate) fn into_kind(self) -> ErrorKind {
        self.kind
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

    /// The arrow function can be parsed from cover production at lower
    /// levels than it is allowed on.
    ArrowFunctionNotAllowed(Expr),

    /// Initializer pattern can be parsed in cover productions in
    /// production that does not allow them.
    InitializedNameNotAllowed,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.kind {
            EndOfStream => write!(f, "Syntax error: Unexpected end of input")?,
            ErrorKind::LexerError(e) => write!(f, "Lexer error '{}'", e)?,
            SyntaxError(msg) => write!(f, "Syntax error: {}", msg)?,
            ExpectedIdentifier(token) | UnexpectedToken(token, _) => {
                if let Some(token_str) = expected_token_to_string(token) {
                    write!(f, "Syntax error: Unexpected token `{}`", token_str)?
                } else {
                    write!(f, "Syntax error: Unexpected token")?
                }
            }
            UnexpectedIdent(ident) => {
                write!(f, "Syntax Error: Unexpected identifier `{}`", ident.name)?
            }
            ForbiddenIdentifier(identifier) => {
                write!(f, "Syntax error: Forbidden identifier `{}`", identifier)?
            }
            ArrowFunctionNotAllowed(_) => {
                write!(f, "Syntax error: Arrow function not allowed here")?
            }
            InitializedNameNotAllowed => write!(f, "Syntax error: Initializer not allowed here")?,
        }

        Ok(())
    }
}

impl error::Error for Error {}

impl From<LexerError> for Error {
    fn from(error: LexerError) -> Self {
        Error::lexer_error(error)
    }
}

impl From<&LexerError> for Error {
    fn from(error: &LexerError) -> Self {
        let error = error.clone();
        Error::lexer_error(error)
    }
}

// TODO Unexpected string, Unexpected number, etc?
fn expected_token_to_string(token: &TokenValue) -> Option<&str> {
    Some(match token {
        TokenValue::Keyword(keyword) => keyword.as_str(),
        TokenValue::Identifier(ident) => ident.as_str(),
        TokenValue::Punctuator(punct) => punct.as_str(),
        _ => return None,
    })
}
