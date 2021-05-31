use fajt_lexer::token::Span;
use fajt_macros::FromString;

use crate::ast::literal::*;
use crate::ast::Ident;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Expression {
    ThisExpression(Box<ThisExpression>),
    IdentifierReference(Box<IdentifierReference>),
    Literal(Box<LiteralExpression>),
    BinaryExpression(Box<BinaryExpression>),
}

impl Expression {
    pub fn this(expr: ThisExpression) -> Self {
        Self::ThisExpression(Box::new(expr))
    }

    pub fn reference(expr: IdentifierReference) -> Self {
        Self::IdentifierReference(Box::new(expr))
    }

    pub fn literal(expr: LiteralExpression) -> Self {
        Self::Literal(Box::new(expr))
    }
}

impl From<ThisExpression> for Expression {
    fn from(expr: ThisExpression) -> Self {
        Self::ThisExpression(Box::new(expr))
    }
}

impl From<LiteralExpression> for Expression {
    fn from(expr: LiteralExpression) -> Self {
        Self::Literal(Box::new(expr))
    }
}

impl From<IdentifierReference> for Expression {
    fn from(expr: IdentifierReference) -> Self {
        Self::IdentifierReference(Box::new(expr))
    }
}

impl From<BinaryExpression> for Expression {
    fn from(expr: BinaryExpression) -> Self {
        Self::BinaryExpression(Box::new(expr))
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct LiteralExpression {
    pub span: Span,
    pub literal: Literal,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ThisExpression {
    pub span: Span,
}

impl ThisExpression {
    pub fn new<S>(span: S) -> Self
    where
        S: Into<Span>,
    {
        Self { span: span.into() }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum IdentifierReference {
    Ident(Ident),
    Yield,
    Await,
}

impl From<Ident> for IdentifierReference {
    fn from(ident: Ident) -> Self {
        Self::Ident(ident)
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct BinaryExpression {
    pub span: Span,
    pub operator: BinaryOperator,
    pub left: Expression,
    pub right: Expression,
}

#[derive(Debug, PartialOrd, PartialEq, FromString)]
#[from_string_macro("binary_op")]
#[from_string_macro_rules(
    ($variant:ident) => {
        $crate::ast::BinaryOperator::$variant
    };
)]
pub enum BinaryOperator {
    #[from_string("+")]
    Plus,
    #[from_string("-")]
    Minus,
    #[from_string("*")]
    Multiplication,
    #[from_string("/")]
    Division,
    #[from_string("%")]
    Modulus,
    #[from_string("**")]
    Exponent,
    #[from_string("<<")]
    ShiftLeft,
    #[from_string(">>")]
    ShiftRight,
    #[from_string(">>>")]
    ShiftRightUnsigned,
    #[from_string("<")]
    LessThan,
    #[from_string(">")]
    MoreThan,
    #[from_string("<=")]
    LessThanEquals,
    #[from_string(">=")]
    MoreThanEquals,
    #[from_string("instanceof")]
    InstanceOf,
    #[from_string("in")]
    In,
    #[from_string("==")]
    Equal,
    #[from_string("!=")]
    NotEqual,
    #[from_string("===")]
    StrictEqual,
    #[from_string("!==")]
    StrictNotEqual,
    #[from_string("&")]
    BitwiseAnd,
    #[from_string("^")]
    BitwiseXOR,
    #[from_string("|")]
    BitwiseOR,
}
