use fajt_lexer::token::Span;
use fajt_macros::FromString;

use crate::ast::class::ExprClass;
use crate::ast::literal::*;
use crate::ast::{FormalParameters, Ident, Stmt};

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Expr {
    ArrowFunction(Box<ExprArrowFunction>),
    Assignment(Box<ExprAssignment>),
    Await(Box<ExprAwait>),
    Binary(Box<ExprBinary>),
    Call(Box<ExprCall>),
    Class(Box<ExprClass>),
    Conditional(Box<ExprConditional>),
    Function(Box<ExprFunction>),
    Identifier(Box<ExprIdentifier>),
    Literal(Box<ExprLiteral>),
    Logical(Box<ExprLogical>),
    Member(Box<ExprMember>),
    MetaProperty(Box<ExprMetaProperty>),
    New(Box<ExprNew>),
    OptionalCall(Box<ExprOptionalCall>),
    OptionalMember(Box<ExprOptionalMember>),
    Parenthesized(Box<ExprParenthesized>),
    Sequence(Box<ExprSequence>),
    This(Box<ExprThis>),
    Unary(Box<ExprUnary>),
    Update(Box<ExprUpdate>),
    Yield(Box<ExprYield>),
}

impl Expr {
    pub fn is_nested_new(&self) -> bool {
        if let Expr::New(expr) = &self {
            matches!(expr.callee, Expr::New(_))
        } else {
            false
        }
    }
}

impl From<ExprThis> for Expr {
    fn from(expr: ExprThis) -> Self {
        Self::This(Box::new(expr))
    }
}

impl From<ExprLiteral> for Expr {
    fn from(expr: ExprLiteral) -> Self {
        Self::Literal(Box::new(expr))
    }
}

impl From<Ident> for Expr {
    fn from(ident: Ident) -> Self {
        Self::Identifier(Box::new(ident.into()))
    }
}

impl From<ExprIdentifier> for Expr {
    fn from(expr: ExprIdentifier) -> Self {
        Self::Identifier(Box::new(expr))
    }
}

impl From<ExprBinary> for Expr {
    fn from(expr: ExprBinary) -> Self {
        Self::Binary(Box::new(expr))
    }
}

impl From<ExprLogical> for Expr {
    fn from(expr: ExprLogical) -> Self {
        Self::Logical(Box::new(expr))
    }
}

impl From<ExprUnary> for Expr {
    fn from(expr: ExprUnary) -> Self {
        Self::Unary(Box::new(expr))
    }
}

impl From<ExprUpdate> for Expr {
    fn from(expr: ExprUpdate) -> Self {
        Self::Update(Box::new(expr))
    }
}

impl From<ExprConditional> for Expr {
    fn from(expr: ExprConditional) -> Self {
        Self::Conditional(Box::new(expr))
    }
}

impl From<ExprYield> for Expr {
    fn from(expr: ExprYield) -> Self {
        Self::Yield(Box::new(expr))
    }
}

impl From<ExprAwait> for Expr {
    fn from(expr: ExprAwait) -> Self {
        Self::Await(Box::new(expr))
    }
}

impl From<ExprSequence> for Expr {
    fn from(expr: ExprSequence) -> Self {
        Self::Sequence(Box::new(expr))
    }
}

impl From<ExprClass> for Expr {
    fn from(expr: ExprClass) -> Self {
        Self::Class(Box::new(expr))
    }
}

impl From<ExprFunction> for Expr {
    fn from(expr: ExprFunction) -> Self {
        Self::Function(Box::new(expr))
    }
}

impl From<ExprArrowFunction> for Expr {
    fn from(expr: ExprArrowFunction) -> Self {
        Self::ArrowFunction(Box::new(expr))
    }
}

impl From<ExprParenthesized> for Expr {
    fn from(expr: ExprParenthesized) -> Self {
        Self::Parenthesized(Box::new(expr))
    }
}

impl From<ExprMember> for Expr {
    fn from(expr: ExprMember) -> Self {
        Self::Member(Box::new(expr))
    }
}

impl From<ExprOptionalMember> for Expr {
    fn from(expr: ExprOptionalMember) -> Self {
        Self::OptionalMember(Box::new(expr))
    }
}

impl From<ExprNew> for Expr {
    fn from(expr: ExprNew) -> Self {
        Self::New(Box::new(expr))
    }
}

impl From<ExprAssignment> for Expr {
    fn from(expr: ExprAssignment) -> Self {
        Self::Assignment(Box::new(expr))
    }
}

impl From<ExprCall> for Expr {
    fn from(expr: ExprCall) -> Self {
        Self::Call(Box::new(expr))
    }
}

impl From<ExprOptionalCall> for Expr {
    fn from(expr: ExprOptionalCall) -> Self {
        Self::OptionalCall(Box::new(expr))
    }
}

impl From<ExprMetaProperty> for Expr {
    fn from(expr: ExprMetaProperty) -> Self {
        Self::MetaProperty(Box::new(expr))
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ExprLiteral {
    pub span: Span,
    pub literal: Literal,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ExprThis {
    pub span: Span,
}

impl ExprThis {
    pub fn new<S>(span: S) -> Self
    where
        S: Into<Span>,
    {
        Self { span: span.into() }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum ExprIdentifier {
    Ident(Ident),
    Yield,
    Await,
}

impl From<Ident> for ExprIdentifier {
    fn from(ident: Ident) -> Self {
        Self::Ident(ident)
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ExprBinary {
    pub span: Span,
    pub operator: BinaryOperator,
    pub left: Expr,
    pub right: Expr,
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
pub struct ExprLogical {
    pub span: Span,
    pub operator: LogicalOperator,
    pub left: Expr,
    pub right: Expr,
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
pub struct ExprUnary {
    pub span: Span,
    pub operator: UnaryOperator,
    pub argument: Expr,
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
pub struct ExprUpdate {
    pub span: Span,
    pub operator: UpdateOperator,
    pub prefix: bool,
    pub argument: Expr,
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
pub struct ExprYield {
    pub span: Span,
    pub argument: Option<Expr>,
    pub delegate: bool,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ExprConditional {
    pub span: Span,
    pub condition: Expr,
    pub consequent: Expr,
    pub alternate: Expr,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ExprAwait {
    pub span: Span,
    pub argument: Expr,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ExprSequence {
    pub span: Span,
    pub expressions: Vec<Expr>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ExprFunction {
    pub span: Span,
    pub asynchronous: bool,
    pub generator: bool,
    pub identifier: Option<Ident>,
    pub parameters: FormalParameters,
    pub body: Vec<Stmt>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ExprArrowFunction {
    pub span: Span,
    pub asynchronous: bool,
    pub binding_parameter: bool,
    pub parameters: FormalParameters,
    pub body: ArrowFunctionBody,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum ArrowFunctionBody {
    Expression(Expr),
    Block(Vec<Stmt>),
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ExprParenthesized {
    pub span: Span,
    pub expression: Expr,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ExprMember {
    pub span: Span,
    pub object: MemberObject,
    pub property: MemberProperty,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ExprOptionalMember {
    pub span: Span,
    pub object: Expr,
    pub property: MemberProperty,
    pub optional: bool,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum MemberObject {
    Expression(Expr),
    Super(Super),
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Super {
    pub span: Span,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum MemberProperty {
    Ident(Ident),
    Expression(Expr),
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ExprMetaProperty {
    pub span: Span,
    pub meta: Ident,
    pub property: Ident,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ExprNew {
    pub span: Span,
    pub callee: Expr,
    pub arguments_span: Option<Span>,
    pub arguments: Vec<Argument>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Argument {
    Expression(Expr),
    Spread(Expr),
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ExprAssignment {
    pub span: Span,
    pub operator: AssignmentOperator,
    pub left: Expr,
    pub right: Expr,
}

#[derive(Debug, PartialOrd, PartialEq, FromString)]
#[from_string_macro("assignment_op")]
#[from_string_macro_rules(
    ($variant:ident) => {
        $crate::ast::AssignmentOperator::$variant
    };
)]
pub enum AssignmentOperator {
    #[from_string("=")]
    Assign,
    #[from_string("*=")]
    Multiply,
    #[from_string("**=")]
    Power,
    #[from_string("/=")]
    Divide,
    #[from_string("%=")]
    Modulus,
    #[from_string("+=")]
    Add,
    #[from_string("-=")]
    Sub,
    #[from_string("<<=")]
    LeftShift,
    #[from_string(">>=")]
    RightShift,
    #[from_string(">>>=")]
    UnsignedRightShift,
    #[from_string("&=")]
    BitwiseAnd,
    #[from_string("^=")]
    BitwiseXOr,
    #[from_string("|=")]
    BitwiseOr,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ExprCall {
    pub span: Span,
    pub callee: Callee,
    pub arguments_span: Span,
    pub arguments: Vec<Argument>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ExprOptionalCall {
    pub span: Span,
    pub callee: Expr,
    pub arguments_span: Span,
    pub arguments: Vec<Argument>,
    pub optional: bool,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Callee {
    Super,
    Import,
    Expression(Expr),
}
