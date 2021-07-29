use super::Ident;
use crate::ast::{BindingElement, BindingPattern, Expr};
use fajt_lexer::token::Span;

// TODO harmonize this so it looks like Expression
/// Note: Declarations are handles as statements since they can appear in the same contexts.
#[derive(Debug, PartialOrd, PartialEq)]
pub enum Stmt {
    FunctionDeclaration(Box<FunctionDeclaration>),
    BlockStatement(Box<BlockStatement>),
    BreakStatement(Box<BreakStatement>),
    ContinueStatement(Box<ContinueStatement>),
    DebuggerStatement(Box<DebuggerStatement>),
    DoWhileStatement(Box<DoWhileStatement>),
    EmptyStatement(Box<EmptyStatement>),
    ExpressionStatement(Box<Expr>),
    ForInStatement(Box<ForInStatement>),
    ForOfStatement(Box<ForOfStatement>),
    ForStatement(Box<ForStatement>),
    IfStatement(Box<IfStatement>),
    ReturnStatement(Box<ReturnStatement>),
    SwitchStatement(Box<SwitchStatement>),
    ThrowStatement(Box<ThrowStatement>),
    TryStatement(Box<TryStatement>),
    VariableStatement(Box<VariableStatement>),
    WhileStatement(Box<WhileStatement>),
    WithStatement(Box<WithStatement>),
}

impl Stmt {
    pub fn expression<E>(expression: E) -> Self
    where
        E: Into<Expr>,
    {
        Self::ExpressionStatement(Box::new(expression.into()))
    }

    pub fn unwrap_block_statement(self) -> BlockStatement {
        if let Stmt::BlockStatement(block) = self {
            *block
        } else {
            panic!("Tried to unwrap {:?} as a block statement", self);
        }
    }
}

impl From<BlockStatement> for Stmt {
    fn from(stmt: BlockStatement) -> Self {
        Self::BlockStatement(Box::new(stmt))
    }
}

impl From<EmptyStatement> for Stmt {
    fn from(stmt: EmptyStatement) -> Self {
        Self::EmptyStatement(Box::new(stmt))
    }
}

impl From<VariableStatement> for Stmt {
    fn from(stmt: VariableStatement) -> Self {
        Self::VariableStatement(Box::new(stmt))
    }
}

impl From<Expr> for Stmt {
    fn from(expr: Expr) -> Self {
        Self::ExpressionStatement(Box::new(expr))
    }
}

impl From<FunctionDeclaration> for Stmt {
    fn from(expr: FunctionDeclaration) -> Self {
        Self::FunctionDeclaration(Box::new(expr))
    }
}

impl From<ReturnStatement> for Stmt {
    fn from(expr: ReturnStatement) -> Self {
        Self::ReturnStatement(Box::new(expr))
    }
}

impl From<BreakStatement> for Stmt {
    fn from(expr: BreakStatement) -> Self {
        Self::BreakStatement(Box::new(expr))
    }
}

impl From<ContinueStatement> for Stmt {
    fn from(expr: ContinueStatement) -> Self {
        Self::ContinueStatement(Box::new(expr))
    }
}

impl From<ThrowStatement> for Stmt {
    fn from(expr: ThrowStatement) -> Self {
        Self::ThrowStatement(Box::new(expr))
    }
}

impl From<DebuggerStatement> for Stmt {
    fn from(expr: DebuggerStatement) -> Self {
        Self::DebuggerStatement(Box::new(expr))
    }
}

impl From<IfStatement> for Stmt {
    fn from(expr: IfStatement) -> Self {
        Self::IfStatement(Box::new(expr))
    }
}

impl From<WithStatement> for Stmt {
    fn from(expr: WithStatement) -> Self {
        Self::WithStatement(Box::new(expr))
    }
}

impl From<TryStatement> for Stmt {
    fn from(expr: TryStatement) -> Self {
        Self::TryStatement(Box::new(expr))
    }
}

impl From<SwitchStatement> for Stmt {
    fn from(expr: SwitchStatement) -> Self {
        Self::SwitchStatement(Box::new(expr))
    }
}

impl From<DoWhileStatement> for Stmt {
    fn from(expr: DoWhileStatement) -> Self {
        Self::DoWhileStatement(Box::new(expr))
    }
}

impl From<WhileStatement> for Stmt {
    fn from(expr: WhileStatement) -> Self {
        Self::WhileStatement(Box::new(expr))
    }
}

impl From<ForStatement> for Stmt {
    fn from(expr: ForStatement) -> Self {
        Self::ForStatement(Box::new(expr))
    }
}

impl From<ForInStatement> for Stmt {
    fn from(expr: ForInStatement) -> Self {
        Self::ForInStatement(Box::new(expr))
    }
}

impl From<ForOfStatement> for Stmt {
    fn from(expr: ForOfStatement) -> Self {
        Self::ForOfStatement(Box::new(expr))
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
