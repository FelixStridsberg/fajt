use crate::ast::{Expression, FormalParameters, Ident, PropertyName, Statement};
use fajt_lexer::token::Span;

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ClassExpression {
    pub span: Span,
    pub identifier: Option<Ident>,
    pub super_class: Option<Expression>,
    pub body: Vec<ClassElement>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum ClassElement {
    Method(ClassMethod),
}

impl From<ClassMethod> for ClassElement {
    fn from(method: ClassMethod) -> Self {
        Self::Method(method)
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ClassMethod {
    pub span: Span,
    pub name: PropertyName,
    pub kind: ClassMethodKind,
    pub parameters: Option<FormalParameters>,
    pub body: Vec<Statement>,
    pub generator: bool,
    pub asynchronous: bool,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum ClassMethodKind {
    Method,
    Get,
    Set,
}
