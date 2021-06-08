use fajt_lexer::token::Span;
use fajt_macros::FromString;

use crate::ast::class::ClassExpression;
use crate::ast::literal::*;
use crate::ast::{FormalParameters, Ident, Statement};

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Expression {
    ThisExpression(Box<ThisExpression>),
    IdentifierReference(Box<IdentifierReference>),
    Literal(Box<LiteralExpression>),
    BinaryExpression(Box<BinaryExpression>),
    LogicalExpression(Box<LogicalExpression>),
    UnaryExpression(Box<UnaryExpression>),
    UpdateExpression(Box<UpdateExpression>),
    YieldExpression(Box<YieldExpression>),
    ConditionalExpression(Box<ConditionalExpression>),
    AwaitExpression(Box<AwaitExpression>),
    SequenceExpression(Box<SequenceExpression>),
    ClassExpression(Box<ClassExpression>),
    FunctionExpression(Box<FunctionExpression>),
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

impl From<Ident> for Expression {
    fn from(ident: Ident) -> Self {
        Self::IdentifierReference(Box::new(ident.into()))
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

impl From<LogicalExpression> for Expression {
    fn from(expr: LogicalExpression) -> Self {
        Self::LogicalExpression(Box::new(expr))
    }
}

impl From<UnaryExpression> for Expression {
    fn from(expr: UnaryExpression) -> Self {
        Self::UnaryExpression(Box::new(expr))
    }
}

impl From<UpdateExpression> for Expression {
    fn from(expr: UpdateExpression) -> Self {
        Self::UpdateExpression(Box::new(expr))
    }
}

impl From<ConditionalExpression> for Expression {
    fn from(expr: ConditionalExpression) -> Self {
        Self::ConditionalExpression(Box::new(expr))
    }
}

impl From<YieldExpression> for Expression {
    fn from(expr: YieldExpression) -> Self {
        Self::YieldExpression(Box::new(expr))
    }
}

impl From<AwaitExpression> for Expression {
    fn from(expr: AwaitExpression) -> Self {
        Self::AwaitExpression(Box::new(expr))
    }
}

impl From<SequenceExpression> for Expression {
    fn from(expr: SequenceExpression) -> Self {
        Self::SequenceExpression(Box::new(expr))
    }
}

impl From<ClassExpression> for Expression {
    fn from(expr: ClassExpression) -> Self {
        Self::ClassExpression(Box::new(expr))
    }
}

impl From<FunctionExpression> for Expression {
    fn from(expr: FunctionExpression) -> Self {
        Self::FunctionExpression(Box::new(expr))
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

#[derive(Debug, PartialOrd, PartialEq)]
pub struct LogicalExpression {
    pub span: Span,
    pub operator: LogicalOperator,
    pub left: Expression,
    pub right: Expression,
}

#[derive(Debug, PartialOrd, PartialEq, FromString)]
#[from_string_macro("logical_op")]
#[from_string_macro_rules(
    ($variant:ident) => {
        $crate::ast::LogicalOperator::$variant
    };
)]
pub enum LogicalOperator {
    #[from_string("&&")]
    And,
    #[from_string("||")]
    Or,
    #[from_string("??")]
    Coalesce,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct UnaryExpression {
    pub span: Span,
    pub operator: UnaryOperator,
    pub argument: Expression,
}

#[derive(Debug, PartialOrd, PartialEq, FromString)]
#[from_string_macro("unary_op")]
#[from_string_macro_rules(
    ($variant:ident) => {
        $crate::ast::UnaryOperator::$variant
    };
)]
pub enum UnaryOperator {
    #[from_string("delete")]
    Delete,
    #[from_string("void")]
    Void,
    #[from_string("typeof")]
    Typeof,
    #[from_string("+")]
    Plus,
    #[from_string("-")]
    Minus,
    #[from_string("~")]
    BitwiseNot,
    #[from_string("!")]
    Not,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct UpdateExpression {
    pub span: Span,
    pub operator: UpdateOperator,
    pub prefix: bool,
    pub argument: Expression,
}

#[derive(Debug, PartialOrd, PartialEq, FromString)]
#[from_string_macro("update_op")]
#[from_string_macro_rules(
    ($variant:ident) => {
        $crate::ast::UpdateOperator::$variant
    };
)]
pub enum UpdateOperator {
    #[from_string("++")]
    Increase,
    #[from_string("--")]
    Decrease,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct YieldExpression {
    pub span: Span,
    pub argument: Option<Expression>,
    pub delegate: bool,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ConditionalExpression {
    pub span: Span,
    pub condition: Expression,
    pub consequent: Expression,
    pub alternate: Expression,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct AwaitExpression {
    pub span: Span,
    pub argument: Expression,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct SequenceExpression {
    pub span: Span,
    pub expressions: Vec<Expression>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct FunctionExpression {
    pub span: Span,
    pub asynchronous: bool,
    pub generator: bool,
    pub identifier: Option<Ident>,
    pub parameters: Option<FormalParameters>,
    pub body: Vec<Statement>,
}
