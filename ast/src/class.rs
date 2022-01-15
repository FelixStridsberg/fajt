use crate::{Expr, Ident, MethodDefinition, Span};

ast_struct! {
    pub struct DeclClass {
        pub span: Span,
        pub identifier: Ident,
        pub super_class: Option<Box<Expr>>,
        pub body: Vec<ClassElement>,
    }
}

ast_struct! {
    pub struct ExprClass {
        pub span: Span,
        pub identifier: Option<Ident>,
        pub super_class: Option<Box<Expr>>,
        pub body: Vec<ClassElement>,
    }
}

ast_mapping! {
    pub enum ClassElement {
        Method(MethodDefinition),
    }
}
