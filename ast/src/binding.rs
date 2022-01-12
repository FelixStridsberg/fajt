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
        pub rest: Option<Ident>,
    }
}

ast_struct! {
    pub struct ObjectBinding {
        pub span: Span,
        pub props: Vec<ObjectBindingProp>,
        pub rest: Option<Ident>,
    }
}

ast_node! {
    pub enum ObjectBindingProp {
        Single(SingleNameBinding),
        KeyValue(PropertyName, BindingElement), // TODO make struct
    }
}

ast_struct! {
    pub struct SingleNameBinding {
        pub span: Span,
        pub ident: Ident,
        pub initializer: Option<Expr>,
    }
}

ast_node! {
    pub enum PropertyName {
        Ident(Ident),
        String(LitString),
        Number(LitNumber),
        Computed(Expr),
    }
}

ast_struct! {
    pub struct BindingElement {
        pub span: Span,
        pub pattern: BindingPattern,
        pub initializer: Option<Expr>,
    }
}
