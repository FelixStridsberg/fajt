use crate::ast::{Expression, Ident};
use fajt_lexer::token::Span;

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ClassExpression {
    pub span: Span,
    pub identifier: Option<Ident>,
    pub super_class: Option<Expression>,
    pub body: Vec<ClassElement>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ClassElement {}
