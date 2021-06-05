use crate::ast::Expression;
use fajt_lexer::token::Span;

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ClassExpression {
    pub span: Span,
    pub super_class: Option<Expression>,
    pub body: Vec<ClassElement>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ClassElement {}
