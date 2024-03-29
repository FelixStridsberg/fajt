use crate::{Expr, Ident, LitNumber, LitString, Span};

ast_mapping! {
    pub enum BindingPattern {
        Ident(Ident),
        Array(ArrayBinding),
        Object(ObjectBinding),
    }
}

ast_struct! {
    pub struct ArrayBinding {
        pub span: Span,
        pub elements: Vec<Option<BindingElement>>,
        pub rest: Option<Box<BindingPattern>>,
    }
}

ast_struct! {
    pub struct ObjectBinding {
        pub span: Span,
        pub props: Vec<ObjectBindingProp>,
        // TODO This should be identifier when the assignment pattern is separated.
        pub rest: Option<Box<BindingPattern>>,
    }
}

ast_node! {
    pub enum ObjectBindingProp {
        Single(SingleNameBinding),
        Named(NamedBinding),
    }
}

ast_struct! {
    pub struct SingleNameBinding {
        pub span: Span,
        pub ident: Ident,
        pub initializer: Option<Box<Expr>>,
    }
}

ast_struct! {
    pub struct NamedBinding {
        pub span: Span,
        pub property: PropertyName,
        pub binding: BindingElement,
    }
}

ast_node! {
    pub enum PropertyName {
        Ident(Ident),
        String(LitString),
        Number(LitNumber),
        Computed(Box<Expr>),
    }
}

ast_struct! {
    pub struct BindingElement {
        pub span: Span,
        pub pattern: BindingPattern,
        pub initializer: Option<Box<Expr>>,
    }
}
