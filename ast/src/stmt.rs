use crate::{
    AssignmentPattern, BindingElement, BindingPattern, DeclClass, Expr, Ident, LitString, Span,
};
use fajt_macros::FromString;

ast_mapping! {
    /// Note: Declarations are handles as statements since they can appear in the same contexts.
    pub enum Stmt {
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
        Labeled(StmtLabeled),
        Return(StmtReturn),
        Switch(StmtSwitch),
        Throw(StmtThrow),
        Try(StmtTry),
        Variable(StmtVariable),
        While(StmtWhile),
        With(StmtWith),
        ClassDecl(DeclClass),
        FunctionDecl(DeclFunction),
        ImportDecl(DeclImport), // Only applicable for Module
        ExportDecl(DeclExport), // Only applicable for Module
    }
}

#[test]
fn size_of_stmt() {
    // To avoid unexpected increase in node size.
    assert_eq!(std::mem::size_of::<Stmt>(), 176);
}

impl Stmt {
    pub fn unwrap_block_stmt(self) -> StmtBlock {
        if let Stmt::Block(block) = self {
            block
        } else {
            panic!("Tried to unwrap {self:?} as a block statement");
        }
    }

    pub fn unwrap_expr_stmt(self) -> StmtExpr {
        if let Stmt::Expr(expr) = self {
            expr
        } else {
            panic!("Tried to unwrap {self:?} as an expression statement");
        }
    }
}

ast_struct! {
    pub struct StmtExpr {
        pub span: Span,
        pub expr: Box<Expr>,
    }
}

ast_struct! {
    pub struct DeclFunction {
        pub span: Span,
        pub asynchronous: bool,
        pub generator: bool,
        pub identifier: Ident,
        pub parameters: FormalParameters,
        pub body: Body,
    }
}

ast_struct! {
    /// FunctionBody, ScriptBody or ModuleBody.
    pub struct Body {
        pub span: Span,
        pub directives: Vec<LitString>,
        pub statements: Vec<Stmt>,
    }
}

ast_struct! {
    pub struct FormalParameters {
        pub span: Span,
        pub bindings: Vec<BindingElement>,
        pub rest: Option<Box<BindingPattern>>,
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

ast_node! {
    #[derive(FromString)]
    pub enum VariableKind {
        #[from_string("const")]
        Const,
        #[from_string("let")]
        Let,
        #[from_string("var")]
        Var,
    }
}

ast_struct! {
    pub struct StmtReturn {
        pub span: Span,
        pub argument: Option<Box<Expr>>,
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
        pub argument: Box<Expr>,
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
        pub condition: Box<Expr>,
        pub consequent: Box<Stmt>,
        pub alternate: Option<Box<Stmt>>,
    }
}

ast_struct! {
    pub struct StmtWith {
        pub span: Span,
        pub object: Box<Expr>,
        pub body: Box<Stmt>,
    }
}

ast_struct! {
    pub struct StmtTry {
        pub span: Span,
        pub block: StmtBlock,
        pub handler: Option<Box<CatchClause>>,
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
        pub discriminant: Box<Expr>,
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
        pub test: Box<Expr>,
    }
}

ast_struct! {
    pub struct StmtWhile {
        pub span: Span,
        pub test: Box<Expr>,
        pub body: Box<Stmt>,
    }
}

ast_struct! {
    pub struct StmtFor {
        pub span: Span,
        pub init: Option<ForInit>,
        pub test: Option<Box<Expr>>,
        pub update: Option<Box<Expr>>,
        pub body: Box<Stmt>,
    }
}

ast_node! {
    pub enum ForInit {
        Expr(Box<Expr>),
        Declaration(StmtVariable),
    }
}

ast_struct! {
    pub struct StmtForIn {
        pub span: Span,
        pub left: ForDeclaration,
        pub right: Box<Expr>,
        pub body: Box<Stmt>,
    }
}

ast_struct! {
    pub struct StmtForOf {
        pub span: Span,
        pub left: ForDeclaration,
        pub right: Box<Expr>,
        pub body: Box<Stmt>,
        pub asynchronous: bool,
    }
}

ast_node! {
    pub enum ForDeclaration {
        Expr(Box<Expr>),
        AssignmentPattern(AssignmentPattern),
        Declaration(ForBinding),
    }
}

ast_struct! {
    pub struct ForBinding {
        pub span: Span,
        pub kind: VariableKind,
        pub binding: BindingPattern,
    }
}

ast_struct! {
    pub struct StmtLabeled {
        pub span: Span,
        pub label: Ident,
        pub body: Box<Stmt>,
    }
}

ast_struct! {
    pub struct DeclImport {
        pub span: Span,
        pub default_binding: Option<Ident>,
        pub namespace_binding: Option<Ident>,
        pub named_imports: Option<Vec<NamedImport>>,
        pub from: LitString,
    }
}

ast_struct! {
    /// The `name` is the name of the imported field, `alias` is the name of that import within the
    /// module.
    ///
    /// The reason for this is so that there always is a straight mapping between `NamedImport.name`
    /// and `NamedExport.name` no matter if aliases are used, the `alias` and `alias_of` are only
    /// used for mapping within each module.
    pub struct NamedImport {
        pub span: Span,
        pub name: Ident,
        pub alias: Option<Ident>,
    }
}

ast_mapping! {
    pub enum DeclExport {
        Decl(ExportDecl),
        DefaultExpr(ExportDefaultExpr),
        DefaultDecl(ExportDefaultDecl),
        Named(ExportNamed),
        Namespace(ExportNamespace),
    }
}

ast_struct! {
    pub struct ExportDecl {
        pub span: Span,
        pub decl: Box<Stmt>,
    }
}

ast_struct! {
    pub struct ExportDefaultDecl {
        pub span: Span,
        pub decl: Box<Stmt>,
    }
}

ast_struct! {
    pub struct ExportDefaultExpr {
        pub span: Span,
        pub expr: Box<Expr>,
    }
}

ast_struct! {
    pub struct ExportNamed {
        pub span: Span,
        pub named_exports: Vec<NamedExport>,
        pub from: Option<LitString>,
    }
}

ast_struct! {
    pub struct ExportNamespace {
        pub span: Span,
        pub alias: Option<Ident>,
        pub from: LitString,
    }
}

ast_struct! {
    /// The `name` is the name of the export, `alias_of` is the local name inside the module the
    /// export statement resides if there is an alias (i.e `alias_of as name`).
    ///
    /// The reason for this is so that there always is a straight mapping between `NamedImport.name`
    /// and `NamedExport.name` no matter if aliases are used, the `alias` and `alias_of` are only
    /// used for mapping within each module.
    pub struct NamedExport {
        pub span: Span,
        pub name: Ident,
        pub alias_of: Option<Ident>,
    }
}
