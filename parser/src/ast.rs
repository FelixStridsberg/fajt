use fajt_lexer::token::{Span, Token, TokenValue};
use std::convert::TryFrom;
use std::process::id;

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
pub enum ModuleItem {}

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

impl From<Ident> for BindingIdentifier {
    fn from(ident: Ident) -> Self {
        Self::Ident(ident)
    }
}

impl From<Ident> for BindingProperty {
    fn from(ident: Ident) -> Self {
        Self::Single(BindingIdentifier::Ident(ident))
    }
}

impl From<Ident> for BindingPattern {
    fn from(ident: Ident) -> Self {
        Self::Ident(BindingIdentifier::Ident(ident))
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum VariableType {
    Const,
    Let,
    Var,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum BindingIdentifier {
    Ident(Ident),
    Yield,
    Await,
}

impl Into<BindingProperty> for BindingIdentifier {
    fn into(self) -> BindingProperty {
        BindingProperty::Single(self)
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum BindingProperty {
    Single(BindingIdentifier), // TODO this can have Initializer as well
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ObjectBinding {
    bindings: Vec<BindingProperty>,
    // TODO rest name
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum BindingPattern {
    Ident(BindingIdentifier),
    Object(Vec<BindingProperty>),
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct VariableDeclaration {
    pub identifier: BindingPattern,
    //pub initializer: TODO
}

impl VariableDeclaration {
    pub fn new<I: Into<BindingPattern>>(identifier: I) -> Self {
        VariableDeclaration {
            identifier: identifier.into(),
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct VariableStmt {
    // TODO can have multiple declarations
    pub variable_type: VariableType,
    pub declarations: Vec<VariableDeclaration>,
}

impl VariableStmt {
    pub fn new(variable_type: VariableType, declarations: Vec<VariableDeclaration>) -> Self {
        VariableStmt {
            variable_type,
            declarations,
        }
    }
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
