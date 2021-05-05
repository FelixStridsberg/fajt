use super::Ident;
use crate::ast::Expr;
use fajt_lexer::token::Span;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Stmt {
    Empty(EmptyStmt),
    VariableStmt(VariableStmt),
}

impl From<EmptyStmt> for Stmt {
    fn from(stmt: EmptyStmt) -> Self {
        Self::Empty(stmt)
    }
}

impl From<VariableStmt> for Stmt {
    fn from(stmt: VariableStmt) -> Self {
        Self::VariableStmt(stmt)
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct EmptyStmt {
    pub span: Span,
}

impl EmptyStmt {
    pub fn new(span: Span) -> Self {
        EmptyStmt { span }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct VariableStmt {
    pub span: Span,
    pub kind: VariableKind,
    pub declarations: Vec<VariableDeclaration>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct VariableDeclaration {
    pub span: Span,
    pub identifier: BindingPattern,
    pub initializer: Option<Expr>,
}

impl VariableDeclaration {
    pub fn new<I, S>(identifier: I, _initializer: Option<Expr>, span: S) -> Self
    where
        I: Into<BindingPattern>,
        S: Into<Span>,
    {
        VariableDeclaration {
            identifier: identifier.into(),
            initializer: None,
            span: span.into(),
        }
    }
}

impl VariableStmt {
    pub fn new<S>(kind: VariableKind, declarations: Vec<VariableDeclaration>, span: S) -> Self
    where
        S: Into<Span>,
    {
        VariableStmt {
            kind,
            declarations,
            span: span.into(),
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum VariableKind {
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

impl From<Ident> for BindingIdentifier {
    fn from(ident: Ident) -> Self {
        Self::Ident(ident)
    }
}

impl<I: Into<BindingIdentifier>> From<I> for ObjectBindingProp {
    fn from(ident: I) -> Self {
        Self::Assign(ident.into())
    }
}

impl From<Ident> for BindingPattern {
    fn from(ident: Ident) -> Self {
        Self::Ident(BindingIdentifier::Ident(ident))
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum BindingPattern {
    Ident(BindingIdentifier),
    Object(ObjectBinding),
    Array(ArrayBinding),
}

impl From<ObjectBinding> for BindingPattern {
    fn from(binding: ObjectBinding) -> Self {
        Self::Object(binding.into())
    }
}

impl From<ArrayBinding> for BindingPattern {
    fn from(binding: ArrayBinding) -> Self {
        Self::Array(binding.into())
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ObjectBinding {
    pub span: Span,
    pub props: Vec<ObjectBindingProp>,
}

impl ObjectBinding {
    pub fn new<S>(props: Vec<ObjectBindingProp>, span: S) -> Self
    where
        S: Into<Span>,
    {
        Self {
            props,
            span: span.into(),
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum ObjectBindingProp {
    Assign(BindingIdentifier), // TODO this can have Initializer as well
                               // KeyValue
                               // Rest
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ArrayBinding {
    pub span: Span,
    pub elements: Vec<Option<BindingPattern>>,
}

impl ArrayBinding {
    pub fn new<S>(elements: Vec<Option<BindingPattern>>, span: S) -> Self
    where
        S: Into<Span>,
    {
        Self {
            elements,
            span: span.into(),
        }
    }
}
