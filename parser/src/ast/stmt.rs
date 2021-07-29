use super::Ident;
use crate::ast::{BindingElement, BindingPattern, Expr};
use fajt_lexer::token::Span;

ast_enum! {
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
}

impl Stmt {
    pub fn expr<E>(expr: E) -> Self
    where
        E: Into<Expr>,
    {
        Self::Expression(expr.into())
    }

    pub fn unwrap_block_statement(self) -> StmtBlock {
        if let Stmt::Block(block) = self {
            block
        } else {
            panic!("Tried to unwrap {:?} as a block statement", self);
        }
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
