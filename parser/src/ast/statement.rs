use super::Ident;
use crate::ast::{BindingElement, BindingPattern, Expression};
use fajt_lexer::token::Span;

// TODO harmonize this so it looks like Expression
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
    With(Box<WithStatement>),
    Try(Box<TryStatement>),
    Switch(Box<SwitchStatement>),
    DoWhile(Box<DoWhileStatement>),
    While(Box<WhileStatement>),
    For(Box<ForStatement>),
    ForIn(Box<ForInStatement>),
}

impl Statement {
    pub fn unwrap_block_statement(self) -> BlockStatement {
        if let Statement::Block(block) = self {
            block
        } else {
            panic!("Tried to unwrap {:?} as a block statement", self);
        }
    }
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

impl From<WithStatement> for Statement {
    fn from(expr: WithStatement) -> Self {
        Self::With(Box::new(expr))
    }
}

impl From<TryStatement> for Statement {
    fn from(expr: TryStatement) -> Self {
        Self::Try(Box::new(expr))
    }
}

impl From<SwitchStatement> for Statement {
    fn from(expr: SwitchStatement) -> Self {
        Self::Switch(Box::new(expr))
    }
}

impl From<DoWhileStatement> for Statement {
    fn from(expr: DoWhileStatement) -> Self {
        Self::DoWhile(Box::new(expr))
    }
}

impl From<WhileStatement> for Statement {
    fn from(expr: WhileStatement) -> Self {
        Self::While(Box::new(expr))
    }
}

impl From<ForStatement> for Statement {
    fn from(expr: ForStatement) -> Self {
        Self::For(Box::new(expr))
    }
}

impl From<ForInStatement> for Statement {
    fn from(expr: ForInStatement) -> Self {
        Self::ForIn(Box::new(expr))
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

#[derive(Debug, PartialOrd, PartialEq)]
pub struct WithStatement {
    pub span: Span,
    pub object: Expression,
    pub body: Statement,
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
    pub discriminant: Expression,
    pub cases: Vec<SwitchCase>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct SwitchCase {
    pub span: Span,
    pub test: Option<Expression>,
    pub consequent: Vec<Statement>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct DoWhileStatement {
    pub span: Span,
    pub body: Statement,
    pub test: Expression,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct WhileStatement {
    pub span: Span,
    pub test: Expression,
    pub body: Statement,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ForStatement {
    pub span: Span,
    pub init: Option<ForInit>,
    pub test: Option<Expression>,
    pub update: Option<Expression>,
    pub body: Statement,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ForInStatement {
    pub span: Span,
    pub left: ForInit,
    pub right: Expression,
    pub body: Statement,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum ForInit {
    Expression(Expression),
    Declaration(VariableStatement),
}
