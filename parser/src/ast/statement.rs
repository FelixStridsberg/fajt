use super::Ident;
use crate::ast::{BindingElement, BindingPattern, Expr};
use fajt_lexer::token::Span;

// TODO harmonize this so it looks like Expression
/// Note: Declarations are handles as statements since they can appear in the same contexts.
#[derive(Debug, PartialOrd, PartialEq)]
pub enum Stmt {
    FunctionDecl(Box<FunctionDeclaration>),
    Block(Box<BlockStatement>),
    Break(Box<BreakStatement>),
    Continue(Box<ContinueStatement>),
    Debugger(Box<DebuggerStatement>),
    DoWhile(Box<DoWhileStatement>),
    Empty(Box<EmptyStatement>),
    Expression(Box<Expr>),
    For(Box<ForStatement>),
    ForIn(Box<ForInStatement>),
    ForOf(Box<ForOfStatement>),
    If(Box<IfStatement>),
    Return(Box<ReturnStatement>),
    Switch(Box<SwitchStatement>),
    Throw(Box<ThrowStatement>),
    Try(Box<TryStatement>),
    Variable(Box<VariableStatement>),
    While(Box<WhileStatement>),
    With(Box<WithStatement>),
}

impl Stmt {
    pub fn expression<E>(expression: E) -> Self
    where
        E: Into<Expr>,
    {
        Self::Expression(Box::new(expression.into()))
    }

    pub fn unwrap_block_statement(self) -> BlockStatement {
        if let Stmt::Block(block) = self {
            *block
        } else {
            panic!("Tried to unwrap {:?} as a block statement", self);
        }
    }
}

impl From<BlockStatement> for Stmt {
    fn from(stmt: BlockStatement) -> Self {
        Self::Block(Box::new(stmt))
    }
}

impl From<EmptyStatement> for Stmt {
    fn from(stmt: EmptyStatement) -> Self {
        Self::Empty(Box::new(stmt))
    }
}

impl From<VariableStatement> for Stmt {
    fn from(stmt: VariableStatement) -> Self {
        Self::Variable(Box::new(stmt))
    }
}

impl From<Expr> for Stmt {
    fn from(expr: Expr) -> Self {
        Self::Expression(Box::new(expr))
    }
}

impl From<FunctionDeclaration> for Stmt {
    fn from(expr: FunctionDeclaration) -> Self {
        Self::FunctionDecl(Box::new(expr))
    }
}

impl From<ReturnStatement> for Stmt {
    fn from(expr: ReturnStatement) -> Self {
        Self::Return(Box::new(expr))
    }
}

impl From<BreakStatement> for Stmt {
    fn from(expr: BreakStatement) -> Self {
        Self::Break(Box::new(expr))
    }
}

impl From<ContinueStatement> for Stmt {
    fn from(expr: ContinueStatement) -> Self {
        Self::Continue(Box::new(expr))
    }
}

impl From<ThrowStatement> for Stmt {
    fn from(expr: ThrowStatement) -> Self {
        Self::Throw(Box::new(expr))
    }
}

impl From<DebuggerStatement> for Stmt {
    fn from(expr: DebuggerStatement) -> Self {
        Self::Debugger(Box::new(expr))
    }
}

impl From<IfStatement> for Stmt {
    fn from(expr: IfStatement) -> Self {
        Self::If(Box::new(expr))
    }
}

impl From<WithStatement> for Stmt {
    fn from(expr: WithStatement) -> Self {
        Self::With(Box::new(expr))
    }
}

impl From<TryStatement> for Stmt {
    fn from(expr: TryStatement) -> Self {
        Self::Try(Box::new(expr))
    }
}

impl From<SwitchStatement> for Stmt {
    fn from(expr: SwitchStatement) -> Self {
        Self::Switch(Box::new(expr))
    }
}

impl From<DoWhileStatement> for Stmt {
    fn from(expr: DoWhileStatement) -> Self {
        Self::DoWhile(Box::new(expr))
    }
}

impl From<WhileStatement> for Stmt {
    fn from(expr: WhileStatement) -> Self {
        Self::While(Box::new(expr))
    }
}

impl From<ForStatement> for Stmt {
    fn from(expr: ForStatement) -> Self {
        Self::For(Box::new(expr))
    }
}

impl From<ForInStatement> for Stmt {
    fn from(expr: ForInStatement) -> Self {
        Self::ForIn(Box::new(expr))
    }
}

impl From<ForOfStatement> for Stmt {
    fn from(expr: ForOfStatement) -> Self {
        Self::ForOf(Box::new(expr))
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct FunctionDeclaration {
    pub span: Span,
    pub asynchronous: bool,
    pub generator: bool,
    pub identifier: Ident,
    pub parameters: FormalParameters,
    pub body: Vec<Stmt>,
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
    pub statements: Vec<Stmt>,
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
    pub initializer: Option<Expr>,
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
    pub argument: Option<Expr>,
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
    pub argument: Option<Expr>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct DebuggerStatement {
    pub span: Span,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct IfStatement {
    pub span: Span,
    pub condition: Expr,
    pub consequent: Stmt,
    pub alternate: Option<Stmt>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct WithStatement {
    pub span: Span,
    pub object: Expr,
    pub body: Stmt,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct TryStatement {
    pub span: Span,
    pub block: BlockStatement,
    pub handler: Option<CatchClause>,
    pub finalizer: Option<BlockStatement>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct CatchClause {
    pub span: Span,
    pub parameter: Option<BindingPattern>,
    pub body: BlockStatement,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct SwitchStatement {
    pub span: Span,
    pub discriminant: Expr,
    pub cases: Vec<SwitchCase>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct SwitchCase {
    pub span: Span,
    pub test: Option<Expr>,
    pub consequent: Vec<Stmt>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct DoWhileStatement {
    pub span: Span,
    pub body: Stmt,
    pub test: Expr,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct WhileStatement {
    pub span: Span,
    pub test: Expr,
    pub body: Stmt,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ForStatement {
    pub span: Span,
    pub init: Option<ForInit>,
    pub test: Option<Expr>,
    pub update: Option<Expr>,
    pub body: Stmt,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ForInStatement {
    pub span: Span,
    pub left: ForInit,
    pub right: Expr,
    pub body: Stmt,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ForOfStatement {
    pub span: Span,
    pub left: ForInit,
    pub right: Expr,
    pub body: Stmt,
    pub wait: bool,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum ForInit {
    Expression(Expr),
    Declaration(VariableStatement),
}
