pub use binding::*;
pub use class::*;
pub(crate) use cover::*;
pub use expr::*;
pub use literal::*;
pub use stmt::*;

use fajt_lexer::token::Span;

#[macro_use]
mod macros;

mod binding;
#[macro_use]
mod expr;
mod class;
mod cover;
mod literal;
mod stmt;

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
pub enum ModuleItem {
    ImportDeclaration(/* TODO */),
    ExportDeclaration(/* TODO */),
    Statement(Stmt),
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
