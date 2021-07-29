use super::Ident;
use crate::ast::{BindingElement, BindingPattern, Expr};
use fajt_lexer::token::Span;

// TODO harmonize this so it looks like Expression
/// Note: Declarations are handles as statements since they can appear in the same contexts.
#[derive(Debug, PartialOrd, PartialEq)]
pub enum Stmt {
    FunctionDecl(Box<DeclFunction>),
    Block(Box<StmtBlock>),
    Break(Box<StmtBreak>),
    Continue(Box<StmtContinue>),
    Debugger(Box<StmtDebugger>),
    DoWhile(Box<StmtDoWhile>),
    Empty(Box<StmtEmpty>),
    Expression(Box<Expr>),
    For(Box<StmtFor>),
    ForIn(Box<StmtForIn>),
    ForOf(Box<StmtForOf>),
    If(Box<StmtIf>),
    Return(Box<StmtReturn>),
    Switch(Box<StmtSwitch>),
    Throw(Box<StmtThrow>),
    Try(Box<StmtTry>),
    Variable(Box<StmtVariable>),
    While(Box<StmtWhile>),
    With(Box<StmtWith>),
}

impl Stmt {
    pub fn expression<E>(expression: E) -> Self
    where
        E: Into<Expr>,
    {
        Self::Expression(Box::new(expression.into()))
    }

    pub fn unwrap_block_statement(self) -> StmtBlock {
        if let Stmt::Block(block) = self {
            *block
        } else {
            panic!("Tried to unwrap {:?} as a block statement", self);
        }
    }
}

impl From<StmtBlock> for Stmt {
    fn from(stmt: StmtBlock) -> Self {
        Self::Block(Box::new(stmt))
    }
}

impl From<StmtEmpty> for Stmt {
    fn from(stmt: StmtEmpty) -> Self {
        Self::Empty(Box::new(stmt))
    }
}

impl From<StmtVariable> for Stmt {
    fn from(stmt: StmtVariable) -> Self {
        Self::Variable(Box::new(stmt))
    }
}

impl From<Expr> for Stmt {
    fn from(expr: Expr) -> Self {
        Self::Expression(Box::new(expr))
    }
}

impl From<DeclFunction> for Stmt {
    fn from(expr: DeclFunction) -> Self {
        Self::FunctionDecl(Box::new(expr))
    }
}

impl From<StmtReturn> for Stmt {
    fn from(expr: StmtReturn) -> Self {
        Self::Return(Box::new(expr))
    }
}

impl From<StmtBreak> for Stmt {
    fn from(expr: StmtBreak) -> Self {
        Self::Break(Box::new(expr))
    }
}

impl From<StmtContinue> for Stmt {
    fn from(expr: StmtContinue) -> Self {
        Self::Continue(Box::new(expr))
    }
}

impl From<StmtThrow> for Stmt {
    fn from(expr: StmtThrow) -> Self {
        Self::Throw(Box::new(expr))
    }
}

impl From<StmtDebugger> for Stmt {
    fn from(expr: StmtDebugger) -> Self {
        Self::Debugger(Box::new(expr))
    }
}

impl From<StmtIf> for Stmt {
    fn from(expr: StmtIf) -> Self {
        Self::If(Box::new(expr))
    }
}

impl From<StmtWith> for Stmt {
    fn from(expr: StmtWith) -> Self {
        Self::With(Box::new(expr))
    }
}

impl From<StmtTry> for Stmt {
    fn from(expr: StmtTry) -> Self {
        Self::Try(Box::new(expr))
    }
}

impl From<StmtSwitch> for Stmt {
    fn from(expr: StmtSwitch) -> Self {
        Self::Switch(Box::new(expr))
    }
}

impl From<StmtDoWhile> for Stmt {
    fn from(expr: StmtDoWhile) -> Self {
        Self::DoWhile(Box::new(expr))
    }
}

impl From<StmtWhile> for Stmt {
    fn from(expr: StmtWhile) -> Self {
        Self::While(Box::new(expr))
    }
}

impl From<StmtFor> for Stmt {
    fn from(expr: StmtFor) -> Self {
        Self::For(Box::new(expr))
    }
}

impl From<StmtForIn> for Stmt {
    fn from(expr: StmtForIn) -> Self {
        Self::ForIn(Box::new(expr))
    }
}

impl From<StmtForOf> for Stmt {
    fn from(expr: StmtForOf) -> Self {
        Self::ForOf(Box::new(expr))
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct DeclFunction {
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
pub struct StmtBlock {
    pub span: Span,
    pub statements: Vec<Stmt>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct StmtEmpty {
    pub span: Span,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct StmtVariable {
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
pub struct StmtReturn {
    pub span: Span,
    pub argument: Option<Expr>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct StmtBreak {
    pub span: Span,
    pub label: Option<Ident>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct StmtContinue {
    pub span: Span,
    pub label: Option<Ident>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct StmtThrow {
    pub span: Span,
    pub argument: Option<Expr>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct StmtDebugger {
    pub span: Span,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct StmtIf {
    pub span: Span,
    pub condition: Expr,
    pub consequent: Stmt,
    pub alternate: Option<Stmt>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct StmtWith {
    pub span: Span,
    pub object: Expr,
    pub body: Stmt,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct StmtTry {
    pub span: Span,
    pub block: StmtBlock,
    pub handler: Option<CatchClause>,
    pub finalizer: Option<StmtBlock>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct CatchClause {
    pub span: Span,
    pub parameter: Option<BindingPattern>,
    pub body: StmtBlock,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct StmtSwitch {
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
pub struct StmtDoWhile {
    pub span: Span,
    pub body: Stmt,
    pub test: Expr,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct StmtWhile {
    pub span: Span,
    pub test: Expr,
    pub body: Stmt,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct StmtFor {
    pub span: Span,
    pub init: Option<ForInit>,
    pub test: Option<Expr>,
    pub update: Option<Expr>,
    pub body: Stmt,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct StmtForIn {
    pub span: Span,
    pub left: ForInit,
    pub right: Expr,
    pub body: Stmt,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct StmtForOf {
    pub span: Span,
    pub left: ForInit,
    pub right: Expr,
    pub body: Stmt,
    pub wait: bool,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum ForInit {
    Expression(Expr),
    Declaration(StmtVariable),
}
