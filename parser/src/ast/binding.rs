use crate::ast::{Expression, Ident, Number};
use fajt_lexer::token::Span;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum BindingPattern {
    Ident(Ident),
    Object(ObjectBinding),
    Array(ArrayBinding),
}

impl From<Ident> for BindingPattern {
    fn from(ident: Ident) -> Self {
        Self::Ident(ident)
    }
}

impl From<ObjectBinding> for BindingPattern {
    fn from(binding: ObjectBinding) -> Self {
        Self::Object(binding)
    }
}

impl From<ArrayBinding> for BindingPattern {
    fn from(binding: ArrayBinding) -> Self {
        Self::Array(binding)
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
    Single(Ident, Option<Expression>),
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
    Computed(Expression),
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct BindingElement {
    pub span: Span,
    pub pattern: BindingPattern,
    pub initializer: Option<Expression>,
}
