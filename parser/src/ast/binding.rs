use crate::ast::{Expr, Ident, Number};
use fajt_lexer::token::Span;

ast_enum! {
    #[derive(Debug, PartialOrd, PartialEq)]
    pub enum BindingPattern {
        Ident(Ident),
        Object(ObjectBinding),
        Array(ArrayBinding),
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ArrayBinding {
    pub span: Span,
    pub elements: Vec<Option<BindingElement>>,
    pub rest: Option<Ident>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ObjectBinding {
    pub span: Span,
    pub props: Vec<ObjectBindingProp>,
    pub rest: Option<Ident>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum ObjectBindingProp {
    Single(Ident, Option<Expr>),
    KeyValue(PropertyName, BindingElement),
}

impl From<Ident> for ObjectBindingProp {
    fn from(ident: Ident) -> Self {
        Self::Single(ident, None)
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum PropertyName {
    Ident(Ident),
    String(String, char),
    Number(Number),
    Computed(Expr),
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct BindingElement {
    pub span: Span,
    pub pattern: BindingPattern,
    pub initializer: Option<Expr>,
}
