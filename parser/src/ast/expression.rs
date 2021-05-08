use crate::ast::Ident;
use fajt_lexer::token::Span;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Expr {
    This(ThisExpr),
    IdentifierReference(IdentifierReference),
}

impl From<ThisExpr> for Expr {
    fn from(expr: ThisExpr) -> Self {
        Self::This(expr)
    }
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
