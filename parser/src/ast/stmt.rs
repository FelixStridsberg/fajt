use super::Ident;
use crate::ast::{BindingElement, BindingPattern, Expr};
use fajt_lexer::token::Span;

ast_mapping! {
    /// Note: Declarations are handles as statements since they can appear in the same contexts.
    pub enum Stmt {
        FunctionDecl(DeclFunction),
        Block(StmtBlock),
        Break(StmtBreak),
        Continue(StmtContinue),
        Debugger(StmtDebugger),
        DoWhile(StmtDoWhile),
        Empty(StmtEmpty),
        Expr(StmtExpr),
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
    pub fn unwrap_block_stmt(self) -> StmtBlock {
        if let Stmt::Block(block) = self {
            block
        } else {
            panic!("Tried to unwrap {:?} as a block statement", self);
        }
    }
}

ast_struct! {
    pub struct StmtExpr {
        pub span: Span,
        pub expr: Expr,
    }
}

ast_struct! {
    pub struct DeclFunction {
        pub span: Span,
        pub asynchronous: bool,
        pub generator: bool,
        pub identifier: Ident,
        pub parameters: FormalParameters,
        pub body: Vec<Stmt>,
    }
}

ast_struct! {
    pub struct FormalParameters {
        pub span: Span,
        pub bindings: Vec<BindingElement>,
        pub rest: Option<BindingPattern>,
    }
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

ast_struct! {
    pub struct StmtBlock {
        pub span: Span,
        pub statements: Vec<Stmt>,
    }
}

ast_struct! {
    pub struct StmtEmpty {
        pub span: Span,
    }
}

ast_struct! {
    pub struct StmtVariable {
        pub span: Span,
        pub kind: VariableKind,
        pub declarations: Vec<VariableDeclaration>,
    }
}

ast_struct! {
    pub struct VariableDeclaration {
        pub span: Span,
        pub pattern: BindingPattern,
        pub initializer: Option<Expr>,
    }
}

ast_struct! {
    pub enum VariableKind {
        Const,
        Let,
        Var,
    }
}

ast_struct! {
    pub struct StmtReturn {
        pub span: Span,
        pub argument: Option<Expr>,
    }
}

ast_struct! {
    pub struct StmtBreak {
        pub span: Span,
        pub label: Option<Ident>,
    }
}

ast_struct! {
    pub struct StmtContinue {
        pub span: Span,
        pub label: Option<Ident>,
    }
}

ast_struct! {
    pub struct StmtThrow {
        pub span: Span,
        pub argument: Option<Expr>,
    }
}

ast_struct! {
    pub struct StmtDebugger {
        pub span: Span,
    }
}

ast_struct! {
    pub struct StmtIf {
        pub span: Span,
        pub condition: Expr,
        pub consequent: Box<Stmt>,
        pub alternate: Option<Box<Stmt>>,
    }
}

ast_struct! {
    pub struct StmtWith {
        pub span: Span,
        pub object: Expr,
        pub body: Box<Stmt>,
    }
}

ast_struct! {
    pub struct StmtTry {
        pub span: Span,
        pub block: StmtBlock,
        pub handler: Option<CatchClause>,
        pub finalizer: Option<StmtBlock>,
    }
}

ast_struct! {
    pub struct CatchClause {
        pub span: Span,
        pub parameter: Option<BindingPattern>,
        pub body: StmtBlock,
    }
}

ast_struct! {
    pub struct StmtSwitch {
        pub span: Span,
        pub discriminant: Expr,
        pub cases: Vec<SwitchCase>,
    }
}

ast_struct! {
    pub struct SwitchCase {
        pub span: Span,
        pub test: Option<Expr>,
        pub consequent: Vec<Stmt>,
    }
}

ast_struct! {
    pub struct StmtDoWhile {
        pub span: Span,
        pub body: Box<Stmt>,
        pub test: Expr,
    }
}

ast_struct! {
    pub struct StmtWhile {
        pub span: Span,
        pub test: Expr,
        pub body: Box<Stmt>,
    }
}

ast_struct! {
    pub struct StmtFor {
        pub span: Span,
        pub init: Option<ForInit>,
        pub test: Option<Expr>,
        pub update: Option<Expr>,
        pub body: Box<Stmt>,
    }
}

ast_struct! {
    pub struct StmtForIn {
        pub span: Span,
        pub left: ForInit,
        pub right: Expr,
        pub body: Box<Stmt>,
    }
}

ast_struct! {
    pub struct StmtForOf {
        pub span: Span,
        pub left: ForInit,
        pub right: Expr,
        pub body: Box<Stmt>,
        pub wait: bool,
    }
}

ast_struct! {
    pub enum ForInit {
        Expr(Expr),
        Declaration(StmtVariable),
    }
}
