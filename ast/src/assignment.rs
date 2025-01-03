use crate::{Expr, Ident, PropertyName, Span};

ast_mapping! {
    pub enum PatternOrExpr {
        Expr(Expr),
        AssignmentPattern(AssignmentPattern),
    }
}

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
        pub rest: Option<Box<PatternOrExpr>>,
    }
}

ast_struct! {
    pub struct AssignmentElement {
        pub span: Span,
        pub target: Box<PatternOrExpr>,
        pub initializer: Option<Box<Expr>>,
    }
}

ast_struct! {
    pub struct ObjectAssignmentPattern {
        pub span: Span,
        pub props: Vec<AssignmentProp>,
        pub rest: Option<Box<PatternOrExpr>>,
    }
}

ast_node! {
    pub enum AssignmentProp {
        Single(SingleNameAssignmentProp),
        Named(NamedAssignmentProp)
    }
}

ast_struct! {
    pub struct SingleNameAssignmentProp {
        pub span: Span,
        pub ident: Ident,
        pub initializer: Option<Box<Expr>>,
    }
}

ast_struct! {
    pub struct NamedAssignmentProp {
        pub span: Span,
        pub name: PropertyName,
        pub value: Box<PatternOrExpr>,
        pub initializer: Option<Box<Expr>>,
    }
}
