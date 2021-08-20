#[macro_use]
mod macros;
pub mod binding;
#[macro_use]
pub mod expr;
pub mod class;
pub mod cover;
pub mod literal;
pub mod stmt;

use serde::{Deserialize, Serialize};

pub use crate::binding::*;
pub use crate::class::*;
pub use crate::expr::*;
pub use crate::literal::*;
pub use crate::stmt::*;

// TODO move to ast
use fajt_lexer::token::Span;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum SourceType {
    Module,
    Script,
    Unknown,
}

pub trait Spanned {
    fn span(&self) -> &Span;
}

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct StatementList<T> {
    span: Span,
    body: Vec<T>,
}

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum Program {
    Script(StatementList<Stmt>),
    Module(StatementList<Stmt>),
}

impl Program {
    pub fn from_body(source_type: SourceType, body: Vec<Stmt>) -> Self {
        let span_start = body.first().map(|s| s.span().start).unwrap_or(0);
        let span_end = body.last().map(|s| s.span().end).unwrap_or(0);
        let span = Span::new(span_start, span_end);

        if source_type == SourceType::Module {
            Program::Module(StatementList { span, body })
        } else {
            Program::Script(StatementList { span, body })
        }
    }

    pub fn span(&self) -> &Span {
        match self {
            Program::Script(body) => &body.span,
            Program::Module(body) => &body.span,
        }
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
