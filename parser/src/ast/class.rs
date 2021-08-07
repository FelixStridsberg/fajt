use crate::ast::{Expr, FormalParameters, Ident, PropertyName, Stmt};
use fajt_lexer::token::Span;

ast_struct! {
    pub struct ExprClass {
        pub span: Span,
        pub identifier: Option<Ident>,
        pub super_class: Option<Box<Expr>>,
        pub body: Vec<ClassElement>,
    }
}

ast_struct! {
    pub enum ClassElement {
        Method(ClassMethod),
    }
}

impl From<ClassMethod> for ClassElement {
    fn from(method: ClassMethod) -> Self {
        Self::Method(method)
    }
}

ast_struct! {
    pub struct ClassMethod {
        pub span: Span,
        pub name: PropertyName,
        pub kind: ClassMethodKind,
        pub parameters: FormalParameters,
        pub body: Vec<Stmt>,
        pub generator: bool,
        pub asynchronous: bool,
    }
}

ast_struct! {
    pub enum ClassMethodKind {
        Method,
        Get,
        Set,
    }
}
