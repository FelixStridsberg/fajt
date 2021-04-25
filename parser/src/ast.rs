use fajt_lexer::token::{Span, Token, TokenValue};
use std::convert::TryFrom;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum ModuleItem {}

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
pub struct Ident {
    pub span: Span,
    pub name: String,
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
pub enum VariableType {
    Const,
    Let,
    Var,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum BindingPattern {
    Ident(Ident),
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct VariableStmt {
    // TODO can have multiple declarations
    pub variable_type: VariableType,
    pub identifier: BindingPattern, // TODO supports other than identifiers
                                    //pub initializer: TODO
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Stmt {
    Empty(EmptyStmt),
    VariableStmt(VariableStmt),
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct EmptyStmt {
    span: Span,
}

impl EmptyStmt {
    pub fn new(span: Span) -> Self {
        EmptyStmt { span }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Expr {}
