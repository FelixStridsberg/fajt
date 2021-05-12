pub mod literal;

pub use literal::*;

use crate::ast::Ident;
use fajt_lexer::token::Span;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Expression {
    ThisExpression(ThisExpression),
    IdentifierReference(IdentifierReference),
    Literal(LiteralExpression),
}

impl From<ThisExpression> for Expression {
    fn from(expr: ThisExpression) -> Self {
        Self::ThisExpression(expr)
    }
}

impl From<LiteralExpression> for Expression {
    fn from(expr: LiteralExpression) -> Self {
        Self::Literal(expr)
    }
}

impl From<IdentifierReference> for Expression {
    fn from(expr: IdentifierReference) -> Self {
        Self::IdentifierReference(expr)
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct LiteralExpression {
    pub span: Span,
    pub literal: Literal,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ThisExpression {
    pub span: Span,
}

impl ThisExpression {
    pub fn new<S>(span: S) -> Self
    where
        S: Into<Span>,
    {
        Self { span: span.into() }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum IdentifierReference {
    Ident(Ident),
    Yield,
    Await,
}

impl From<Ident> for IdentifierReference {
    fn from(ident: Ident) -> Self {
        Self::Ident(ident)
    }
}
