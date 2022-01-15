use crate::{Body, Expr, FormalParameters, Ident, PropertyName, Span};

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
        Method(ClassMethod),
    }
}

ast_struct! {
    pub struct ClassMethod { // TODO rename, MethodDefinition, this is not class specific
        pub span: Span,
        pub name: PropertyName,
        pub kind: ClassMethodKind,
        pub parameters: FormalParameters,
        pub body: Body,
        pub generator: bool,
        pub asynchronous: bool,
    }
}

ast_node! {
    pub enum ClassMethodKind {
        Method,
        Get,
        Set,
    }
}
