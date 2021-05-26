pub use binding::*;
pub use expression::*;
pub use literal::*;
pub use statement::*;

use fajt_lexer::token::Span;

mod binding;
mod expression;
mod literal;
mod statement;

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Body<T> {
    span: Span,
    body: Vec<T>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Program {
    Script(Body<Statement>),
    Module(Body<ModuleItem>),
}

impl Program {
    pub fn from_body(body: Vec<Statement>) -> Self {
        Program::Script(Body {
            span: (0, 0).into(),
            body,
        })
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum ModuleItem {
    ImportDeclaration(/* TODO */),
    ExportDeclaration(/* TODO */),
    Statement(Statement),
}

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
