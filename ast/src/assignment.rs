use crate::{ Ident, Span, Expr, PropertyName};

ast_mapping! {
    pub enum AssignmentPattern {
        Array(ArrayAssignmentPattern),
        Object(ObjectAssignmentPattern),
    }
}

ast_struct! {
    pub struct ArrayAssignmentPattern {
        pub span: Span,
        pub elements: Vec<Option<AssignmentElement>>,
        pub rest: Option<Box<Expr>>,
    }
}

ast_struct! {
    pub struct AssignmentElement {
        pub span: Span,
        pub target: Box<Expr>,
        pub initializer: Option<Box<Expr>>,
    }
}

ast_struct! {
    pub struct ObjectAssignmentPattern {
        pub span: Span,
        pub props: Vec<AssignmentProp>,
        pub rest: Option<Box<Expr>>,
    }
}

ast_node! {
    pub enum AssignmentProp {
        Single(SingleNameProp),
        Named(NamedProp)
    }
}

ast_struct! {
    pub struct SingleNameProp {
        pub span: Span,
        pub ident: Ident,
        pub initializer: Option<Box<Expr>>,
    }
}

ast_struct! {
    pub struct NamedProp {
        pub span: Span,
        pub name: PropertyName,
        pub value: Box<Expr>,
    }
}
