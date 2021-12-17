use crate::{Expr, Ident, Number, Span};

ast_mapping! {
    pub enum BindingPattern {
        Ident(Ident),
        Object(ObjectBinding),
        Array(ArrayBinding),
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
        Single(Ident, Option<Expr>), // TODO make struct
        KeyValue(PropertyName, BindingElement), // TODO make struct
    }
}

impl From<Ident> for ObjectBindingProp {
    fn from(ident: Ident) -> Self {
        Self::Single(ident, None)
    }
}

ast_node! {
    pub enum PropertyName {
        Ident(Ident),
        String(String, char), // TODO LitString?
        Number(Number),
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
