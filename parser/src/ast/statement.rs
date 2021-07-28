use super::Ident;
use crate::ast::{BindingElement, BindingPattern, Expression};
use fajt_lexer::token::Span;

/// Note: Declarations are handles as statements since they can appear in the same contexts.
#[derive(Debug, PartialOrd, PartialEq)]
pub enum Statement {
    Block(BlockStatement),
    Empty(EmptyStatement),
    Variable(VariableStatement),
    Expression(Expression),
    FunctionDeclaration(FunctionDeclaration),
    Return(ReturnStatement),
    Break(BreakStatement),
    Continue(ContinueStatement),
    Throw(ThrowStatement),
    Debugger(DebuggerStatement),
    If(Box<IfStatement>),
}

impl From<BlockStatement> for Statement {
    fn from(stmt: BlockStatement) -> Self {
        Self::Block(stmt)
    }
}

impl From<EmptyStatement> for Statement {
    fn from(stmt: EmptyStatement) -> Self {
        Self::Empty(stmt)
    }
}

impl From<VariableStatement> for Statement {
    fn from(stmt: VariableStatement) -> Self {
        Self::Variable(stmt)
    }
}

impl From<Expression> for Statement {
    fn from(expr: Expression) -> Self {
        Self::Expression(expr)
    }
}

impl From<FunctionDeclaration> for Statement {
    fn from(expr: FunctionDeclaration) -> Self {
        Self::FunctionDeclaration(expr)
    }
}

impl From<ReturnStatement> for Statement {
    fn from(expr: ReturnStatement) -> Self {
        Self::Return(expr)
    }
}

impl From<BreakStatement> for Statement {
    fn from(expr: BreakStatement) -> Self {
        Self::Break(expr)
    }
}

impl From<ContinueStatement> for Statement {
    fn from(expr: ContinueStatement) -> Self {
        Self::Continue(expr)
    }
}

impl From<ThrowStatement> for Statement {
    fn from(expr: ThrowStatement) -> Self {
        Self::Throw(expr)
    }
}

impl From<DebuggerStatement> for Statement {
    fn from(expr: DebuggerStatement) -> Self {
        Self::Debugger(expr)
    }
}

impl From<IfStatement> for Statement {
    fn from(expr: IfStatement) -> Self {
        Self::If(Box::new(expr))
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct FunctionDeclaration {
    pub span: Span,
    pub asynchronous: bool,
    pub generator: bool,
    pub identifier: Ident,
    pub parameters: FormalParameters,
    pub body: Vec<Statement>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct FormalParameters {
    pub span: Span,
    pub bindings: Vec<BindingElement>,
    pub rest: Option<BindingPattern>,
}

impl FormalParameters {
    pub fn empty<S>(span: S) -> Self
    where
        S: Into<Span>,
    {
        FormalParameters {
            span: span.into(),
            bindings: vec![],
            rest: None,
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct BlockStatement {
    pub span: Span,
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct EmptyStatement {
    pub span: Span,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct VariableStatement {
    pub span: Span,
    pub kind: VariableKind,
    pub declarations: Vec<VariableDeclaration>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct VariableDeclaration {
    pub span: Span,
    pub pattern: BindingPattern,
    pub initializer: Option<Expression>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum VariableKind {
    Const,
    Let,
    Var,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ReturnStatement {
    pub span: Span,
    pub argument: Option<Expression>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct BreakStatement {
    pub span: Span,
    pub label: Option<Ident>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ContinueStatement {
    pub span: Span,
    pub label: Option<Ident>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ThrowStatement {
    pub span: Span,
    pub argument: Option<Expression>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct DebuggerStatement {
    pub span: Span,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct IfStatement {
    pub span: Span,
    pub condition: Expression,
    pub consequent: Statement,
    pub alternate: Option<Statement>,
}
