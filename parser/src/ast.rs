pub mod statement;

pub use statement::*;

use fajt_lexer::token::{Span, Token, TokenValue};
use std::convert::TryFrom;

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Body<T> {
    span: Span,
    body: Vec<T>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Program {
    Script(Body<Stmt>),
    Module(Body<ModuleItem>),
}

impl Program {
    pub fn from_body(body: Vec<Stmt>) -> Self {
        Program::Script(Body {
            span: (0, 0).into(),
            body,
        })
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum ModuleItem {}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Ident {
    pub span: Span,
    pub name: String,
}

impl Ident {
    pub fn new<N, S>(name: N, span: S) -> Self
    where
        N: Into<String>,
        S: Into<Span>,
    {
        Ident {
            name: name.into(),
            span: span.into(),
        }
    }
}

impl TryFrom<&Token> for Ident {
    type Error = ();

    fn try_from(token: &Token) -> Result<Self, Self::Error> {
        match &token.value {
            TokenValue::Identifier(name) => Ok(Ident {
                span: token.location.clone(),
                name: name.clone(),
            }),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Expr {}
