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
        let span_start = body.first().map(|s| s.span().start).unwrap_or(0);
        let span_end = body.last().map(|s| s.span().end).unwrap_or(0);
        Program::Script(Body {
            span: Span::new(span_start, span_end),
            body,
        })
    }

    pub fn span(&self) -> &Span {
        match self {
            Program::Script(body) => &body.span,
            Program::Module(body) => &body.span,
        }
    }
}

ast_struct! {
    pub enum ModuleItem {
        ImportDeclaration(/* TODO */),
        ExportDeclaration(/* TODO */),
        Statement(Stmt),
    }
}

ast_struct! {
    pub struct Ident {
        pub span: Span,
        pub name: String,
    }
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
