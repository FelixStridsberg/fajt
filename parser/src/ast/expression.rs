use fajt_lexer::token::Span;
use fajt_macros::FromString;

use crate::ast::class::ClassExpression;
use crate::ast::literal::*;
use crate::ast::{FormalParameters, Ident, Stmt};

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Expr {
    ArrowFunction(Box<ArrowFunctionExpression>),
    Assignment(Box<AssignmentExpression>),
    Await(Box<AwaitExpression>),
    Binary(Box<BinaryExpression>),
    Call(Box<CallExpression>),
    Class(Box<ClassExpression>),
    Conditional(Box<ConditionalExpression>),
    Function(Box<FunctionExpression>),
    Identifier(Box<IdentifierReference>),
    Literal(Box<LiteralExpression>),
    Logical(Box<LogicalExpression>),
    Member(Box<MemberExpression>),
    MetaProperty(Box<MetaPropertyExpression>),
    New(Box<NewExpression>),
    OptionalCall(Box<OptionalCallExpression>),
    OptionalMember(Box<OptionalMemberExpression>),
    Parenthesized(Box<ParenthesizedExpression>),
    Sequence(Box<SequenceExpression>),
    This(Box<ThisExpression>),
    Unary(Box<UnaryExpression>),
    Update(Box<UpdateExpression>),
    Yield(Box<YieldExpression>),
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

impl From<ThisExpression> for Expr {
    fn from(expr: ThisExpression) -> Self {
        Self::This(Box::new(expr))
    }
}

impl From<LiteralExpression> for Expr {
    fn from(expr: LiteralExpression) -> Self {
        Self::Literal(Box::new(expr))
    }
}

impl From<Ident> for Expr {
    fn from(ident: Ident) -> Self {
        Self::Identifier(Box::new(ident.into()))
    }
}

impl From<IdentifierReference> for Expr {
    fn from(expr: IdentifierReference) -> Self {
        Self::Identifier(Box::new(expr))
    }
}

impl From<BinaryExpression> for Expr {
    fn from(expr: BinaryExpression) -> Self {
        Self::Binary(Box::new(expr))
    }
}

impl From<LogicalExpression> for Expr {
    fn from(expr: LogicalExpression) -> Self {
        Self::Logical(Box::new(expr))
    }
}

impl From<UnaryExpression> for Expr {
    fn from(expr: UnaryExpression) -> Self {
        Self::Unary(Box::new(expr))
    }
}

impl From<UpdateExpression> for Expr {
    fn from(expr: UpdateExpression) -> Self {
        Self::Update(Box::new(expr))
    }
}

impl From<ConditionalExpression> for Expr {
    fn from(expr: ConditionalExpression) -> Self {
        Self::Conditional(Box::new(expr))
    }
}

impl From<YieldExpression> for Expr {
    fn from(expr: YieldExpression) -> Self {
        Self::Yield(Box::new(expr))
    }
}

impl From<AwaitExpression> for Expr {
    fn from(expr: AwaitExpression) -> Self {
        Self::Await(Box::new(expr))
    }
}

impl From<SequenceExpression> for Expr {
    fn from(expr: SequenceExpression) -> Self {
        Self::Sequence(Box::new(expr))
    }
}

impl From<ClassExpression> for Expr {
    fn from(expr: ClassExpression) -> Self {
        Self::Class(Box::new(expr))
    }
}

impl From<FunctionExpression> for Expr {
    fn from(expr: FunctionExpression) -> Self {
        Self::Function(Box::new(expr))
    }
}

impl From<ArrowFunctionExpression> for Expr {
    fn from(expr: ArrowFunctionExpression) -> Self {
        Self::ArrowFunction(Box::new(expr))
    }
}

impl From<ParenthesizedExpression> for Expr {
    fn from(expr: ParenthesizedExpression) -> Self {
        Self::Parenthesized(Box::new(expr))
    }
}

impl From<MemberExpression> for Expr {
    fn from(expr: MemberExpression) -> Self {
        Self::Member(Box::new(expr))
    }
}

impl From<OptionalMemberExpression> for Expr {
    fn from(expr: OptionalMemberExpression) -> Self {
        Self::OptionalMember(Box::new(expr))
    }
}

impl From<NewExpression> for Expr {
    fn from(expr: NewExpression) -> Self {
        Self::New(Box::new(expr))
    }
}

impl From<AssignmentExpression> for Expr {
    fn from(expr: AssignmentExpression) -> Self {
        Self::Assignment(Box::new(expr))
    }
}

impl From<CallExpression> for Expr {
    fn from(expr: CallExpression) -> Self {
        Self::Call(Box::new(expr))
    }
}

impl From<OptionalCallExpression> for Expr {
    fn from(expr: OptionalCallExpression) -> Self {
        Self::OptionalCall(Box::new(expr))
    }
}

impl From<MetaPropertyExpression> for Expr {
    fn from(expr: MetaPropertyExpression) -> Self {
        Self::MetaProperty(Box::new(expr))
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
pub struct LogicalExpression {
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
pub struct UnaryExpression {
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
pub struct UpdateExpression {
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
pub struct YieldExpression {
    pub span: Span,
    pub argument: Option<Expr>,
    pub delegate: bool,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ConditionalExpression {
    pub span: Span,
    pub condition: Expr,
    pub consequent: Expr,
    pub alternate: Expr,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct AwaitExpression {
    pub span: Span,
    pub argument: Expr,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct SequenceExpression {
    pub span: Span,
    pub expressions: Vec<Expr>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct FunctionExpression {
    pub span: Span,
    pub asynchronous: bool,
    pub generator: bool,
    pub identifier: Option<Ident>,
    pub parameters: FormalParameters,
    pub body: Vec<Stmt>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ArrowFunctionExpression {
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
pub struct ParenthesizedExpression {
    pub span: Span,
    pub expression: Expr,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct MemberExpression {
    pub span: Span,
    pub object: MemberObject,
    pub property: MemberProperty,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct OptionalMemberExpression {
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
pub struct MetaPropertyExpression {
    pub span: Span,
    pub meta: Ident,
    pub property: Ident,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct NewExpression {
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
pub struct AssignmentExpression {
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
pub struct CallExpression {
    pub span: Span,
    pub callee: Callee,
    pub arguments_span: Span,
    pub arguments: Vec<Argument>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct OptionalCallExpression {
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
