use super::Ident;
use crate::ast::Expr;
use fajt_lexer::token::Span;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Stmt {
    Empty(EmptyStmt),
    VariableStmt(VariableStmt),
    ExpressionStmt(Expr),
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

impl From<Expr> for Stmt {
    fn from(expr: Expr) -> Self {
        Self::ExpressionStmt(expr)
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct EmptyStmt {
    pub span: Span,
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

#[derive(Debug, PartialOrd, PartialEq)]
pub enum VariableKind {
    Const,
    Let,
    Var,
}

impl From<Ident> for BindingPattern {
    fn from(ident: Ident) -> Self {
        Self::Ident(ident)
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum BindingPattern {
    Ident(Ident),
    Object(ObjectBinding),
    Array(ArrayBinding),
}

impl From<ObjectBinding> for BindingPattern {
    fn from(binding: ObjectBinding) -> Self {
        Self::Object(binding)
    }
}

impl From<ArrayBinding> for BindingPattern {
    fn from(binding: ArrayBinding) -> Self {
        Self::Array(binding)
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ObjectBinding {
    pub span: Span,
    pub props: Vec<ObjectBindingProp>,
    pub rest: Option<Ident>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum ObjectBindingProp {
    Assign(Ident), // TODO this can have Initializer as well
                   // KeyValue
}

impl From<Ident> for ObjectBindingProp {
    fn from(ident: Ident) -> Self {
        Self::Assign(ident)
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ArrayBinding {
    pub span: Span,
    pub elements: Vec<Option<BindingPattern>>,
    pub rest: Option<Ident>,
}
