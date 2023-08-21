pub mod traverse;
#[macro_use]
mod macros;
pub mod binding;
pub mod span;
#[macro_use]
pub mod expr;
pub mod assignment;
pub mod class;
pub mod literal;
pub mod method;
pub mod stmt;

use serde::{Deserialize, Serialize};

pub use crate::assignment::*;
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
}

pub trait Spanned {
    fn span(&self) -> &Span;
}

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct StmtList<T> {
    pub span: Span,
    pub directives: Vec<LitString>,
    pub body: Vec<T>,
}

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum Program {
    Script(StmtList<Stmt>),
    Module(StmtList<Stmt>),
}

impl Program {
    pub fn new(source_type: SourceType, stmt_list: StmtList<Stmt>) -> Self {
        if source_type == SourceType::Module {
            Program::Module(stmt_list)
        } else {
            Program::Script(stmt_list)
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

    pub fn dummy(position: usize) -> Self {
        Self::new("", Span::new(position, position))
    }
}
