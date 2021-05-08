use crate::ast::Ident;
use fajt_lexer::token::Span;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Expr {
    This(ThisExpr),
    Literal(LiteralExpr),
    IdentifierReference(IdentifierReference),
}

impl From<ThisExpr> for Expr {
    fn from(expr: ThisExpr) -> Self {
        Self::This(expr)
    }
}

impl From<LiteralExpr> for Expr {
    fn from(expr: LiteralExpr) -> Self {
        Self::Literal(expr)
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct LiteralExpr {
    span: Span,
    literal: Literal,
}

impl LiteralExpr {
    pub fn new<S>(literal: Literal, span: S) -> Self
    where
        S: Into<Span>,
    {
        Self {
            literal,
            span: span.into(),
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Literal {
    Null,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ThisExpr {
    pub span: Span,
}

impl ThisExpr {
    pub fn new<S>(span: S) -> Self
    where
        S: Into<Span>,
    {
        Self { span: span.into() }
    }
}

impl From<IdentifierReference> for Expr {
    fn from(expr: IdentifierReference) -> Self {
        Self::IdentifierReference(expr)
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
