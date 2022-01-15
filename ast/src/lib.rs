pub mod traverse;
#[macro_use]
mod macros;
pub mod binding;
pub mod span;
#[macro_use]
pub mod expr;
pub mod class;
pub mod literal;
pub mod method;
pub mod stmt;

use serde::{Deserialize, Serialize};

pub use crate::binding::*;
pub use crate::class::*;
pub use crate::expr::*;
pub use crate::literal::*;
pub use crate::method::*;
pub use crate::span::*;
pub use crate::stmt::*;

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
pub struct StmtList<T> {
    pub span: Span,
    pub body: Vec<T>,
}

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum Program {
    Script(StmtList<Stmt>),
    Module(StmtList<Stmt>),
}

impl Program {
    pub fn from_body(source_type: SourceType, body: Vec<Stmt>) -> Self {
        let span_start = body.first().map(|s| s.span().start).unwrap_or(0);
        let span_end = body.last().map(|s| s.span().end).unwrap_or(0);
        let span = Span::new(span_start, span_end);

        if source_type == SourceType::Module {
            Program::Module(StmtList { span, body })
        } else {
            Program::Script(StmtList { span, body })
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
