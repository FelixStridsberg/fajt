use super::Ident;
use crate::ast::{BindingElement, BindingPattern, Expr};
use fajt_lexer::token::Span;

/// Note: Declarations are handles as statements since they can appear in the same contexts.
#[derive(Debug, PartialOrd, PartialEq)]
pub enum Stmt {
    FunctionDecl(DeclFunction),
    Block(StmtBlock),
    Break(StmtBreak),
    Continue(StmtContinue),
    Debugger(StmtDebugger),
    DoWhile(StmtDoWhile),
    Empty(StmtEmpty),
    Expression(Expr),
    For(StmtFor),
    ForIn(StmtForIn),
    ForOf(StmtForOf),
    If(StmtIf),
    Return(StmtReturn),
    Switch(StmtSwitch),
    Throw(StmtThrow),
    Try(StmtTry),
    Variable(StmtVariable),
    While(StmtWhile),
    With(StmtWith),
}

impl Stmt {
    pub fn expression<E>(expression: E) -> Self
    where
        E: Into<Expr>,
    {
        Self::Expression(expression.into())
    }

    pub fn unwrap_block_statement(self) -> StmtBlock {
        if let Stmt::Block(block) = self {
            block
        } else {
            panic!("Tried to unwrap {:?} as a block statement", self);
        }
    }
}

// TODO macrofy this
impl From<StmtBlock> for Stmt {
    fn from(stmt: StmtBlock) -> Self {
        Self::Block(stmt)
    }
}

impl Into<Box<Stmt>> for StmtEmpty {
    fn into(self) -> Box<Stmt> {
        Box::new(Stmt::Empty(self))
    }
}

impl Into<Box<Stmt>> for StmtBlock {
    fn into(self) -> Box<Stmt> {
        Box::new(Stmt::Block(self))
    }
}

impl From<StmtEmpty> for Stmt {
    fn from(stmt: StmtEmpty) -> Self {
        Self::Empty(stmt)
    }
}

impl From<StmtVariable> for Stmt {
    fn from(stmt: StmtVariable) -> Self {
        Self::Variable(stmt)
    }
}

impl From<Expr> for Stmt {
    fn from(expr: Expr) -> Self {
        Self::Expression(expr)
    }
}

impl From<DeclFunction> for Stmt {
    fn from(decl: DeclFunction) -> Self {
        Self::FunctionDecl(decl)
    }
}

impl From<StmtReturn> for Stmt {
    fn from(stmt: StmtReturn) -> Self {
        Self::Return(stmt)
    }
}

impl From<StmtBreak> for Stmt {
    fn from(stmt: StmtBreak) -> Self {
        Self::Break(stmt)
    }
}

impl From<StmtContinue> for Stmt {
    fn from(stmt: StmtContinue) -> Self {
        Self::Continue(stmt)
    }
}

impl From<StmtThrow> for Stmt {
    fn from(stmt: StmtThrow) -> Self {
        Self::Throw(stmt)
    }
}

impl From<StmtDebugger> for Stmt {
    fn from(stmt: StmtDebugger) -> Self {
        Self::Debugger(stmt)
    }
}

impl From<StmtIf> for Stmt {
    fn from(stmt: StmtIf) -> Self {
        Self::If(stmt)
    }
}

impl From<StmtWith> for Stmt {
    fn from(stmt: StmtWith) -> Self {
        Self::With(stmt)
    }
}

impl From<StmtTry> for Stmt {
    fn from(stmt: StmtTry) -> Self {
        Self::Try(stmt)
    }
}

impl From<StmtSwitch> for Stmt {
    fn from(stmt: StmtSwitch) -> Self {
        Self::Switch(stmt)
    }
}

impl From<StmtDoWhile> for Stmt {
    fn from(stmt: StmtDoWhile) -> Self {
        Self::DoWhile(stmt)
    }
}

impl From<StmtWhile> for Stmt {
    fn from(stmt: StmtWhile) -> Self {
        Self::While(stmt)
    }
}

impl From<StmtFor> for Stmt {
    fn from(stmt: StmtFor) -> Self {
        Self::For(stmt)
    }
}

impl From<StmtForIn> for Stmt {
    fn from(stmt: StmtForIn) -> Self {
        Self::ForIn(stmt)
    }
}

impl From<StmtForOf> for Stmt {
    fn from(stmt: StmtForOf) -> Self {
        Self::ForOf(stmt)
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
    pub consequent: Box<Stmt>,
    pub alternate: Option<Box<Stmt>>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct StmtWith {
    pub span: Span,
    pub object: Expr,
    pub body: Box<Stmt>,
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
    pub body: Box<Stmt>,
    pub test: Expr,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct StmtWhile {
    pub span: Span,
    pub test: Expr,
    pub body: Box<Stmt>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct StmtFor {
    pub span: Span,
    pub init: Option<ForInit>,
    pub test: Option<Expr>,
    pub update: Option<Expr>,
    pub body: Box<Stmt>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct StmtForIn {
    pub span: Span,
    pub left: ForInit,
    pub right: Expr,
    pub body: Box<Stmt>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct StmtForOf {
    pub span: Span,
    pub left: ForInit,
    pub right: Expr,
    pub body: Box<Stmt>,
    pub wait: bool,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum ForInit {
    Expression(Expr),
    Declaration(StmtVariable),
}
