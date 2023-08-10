use crate::assignment::AssignmentPattern;
use crate::class::ExprClass;
use crate::literal::*;
use crate::{Body, FormalParameters, Ident, Span};
use fajt_macros::FromString;

ast_mapping! {
    pub enum Expr {
        ArrowFunction(ExprArrowFunction),
        Assignment(ExprAssignment),
        AssignmentPattern(AssignmentPattern),
        Await(ExprAwait),
        Binary(ExprBinary),
        Call(ExprCall),
        Class(ExprClass),
        Conditional(ExprConditional),
        Function(ExprFunction),
        IdentRef(Ident),
        Literal(ExprLiteral),
        Logical(ExprLogical),
        Member(ExprMember),
        MetaProperty(ExprMetaProperty),
        New(ExprNew),
        OptionalCall(ExprOptionalCall),
        OptionalMember(ExprOptionalMember),
        Parenthesized(ExprParenthesized),
        Sequence(ExprSequence),
        TaggedTemplate(ExprTaggedTemplate),
        This(ExprThis),
        Unary(ExprUnary),
        Update(ExprUpdate),
        Yield(ExprYield),
    }
}

#[test]
fn size_of_expr() {
    // To avoid unexpected increase in node size.
    assert_eq!(std::mem::size_of::<Expr>(), 184);
}

impl Expr {
    pub fn is_nested_new(&self) -> bool {
        if let Expr::New(expr) = &self {
            matches!(*expr.callee, Expr::New(_))
        } else {
            false
        }
    }

    pub fn unwrap_ident_ref(self) -> Ident {
        if let Expr::IdentRef(ident) = self {
            ident
        } else {
            panic!("Tried to unwrap {self:?} as a ident ref");
        }
    }

    pub fn unwrap_literal(self) -> ExprLiteral {
        if let Expr::Literal(literal) = self {
            literal
        } else {
            panic!("Tried to unwrap {self:?} as a literal expression");
        }
    }
}

ast_struct! {
    pub struct ExprLiteral {
        pub span: Span,
        pub literal: Literal,
    }
}

ast_struct! {
    pub struct ExprThis {
        pub span: Span,
    }
}

impl ExprThis {
    pub fn new<S>(span: S) -> Self
    where
        S: Into<Span>,
    {
        Self { span: span.into() }
    }
}

ast_struct! {
    pub struct ExprBinary {
        pub span: Span,
        pub operator: BinaryOperator,
        pub left: Box<Expr>,
        pub right: Box<Expr>,
    }
}

ast_node! {
    #[derive(FromString)]
    #[from_string_macro("binary_op")]
    #[from_string_macro_rules(
        ($variant:ident) => {
            $crate::BinaryOperator::$variant
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
}

ast_struct! {
    pub struct ExprLogical {
        pub span: Span,
        pub operator: LogicalOperator,
        pub left: Box<Expr>,
        pub right: Box<Expr>,
    }
}

ast_node! {
    #[derive(FromString)]
    #[from_string_macro("logical_op")]
    #[from_string_macro_rules(
        ($variant:ident) => {
            $crate::LogicalOperator::$variant
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
}

ast_struct! {
    pub struct ExprUnary {
        pub span: Span,
        pub operator: UnaryOperator,
        pub argument: Box<Expr>,
    }
}

ast_node! {
    #[derive(FromString)]
    #[from_string_macro("unary_op")]
    #[from_string_macro_rules(
        ($variant:ident) => {
            $crate::UnaryOperator::$variant
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
}

ast_struct! {
    pub struct ExprUpdate {
        pub span: Span,
        pub operator: UpdateOperator,
        pub prefix: bool,
        pub argument: Box<Expr>,
    }
}

ast_node! {
    #[derive(FromString)]
    #[from_string_macro("update_op")]
    #[from_string_macro_rules(
        ($variant:ident) => {
            $crate::UpdateOperator::$variant
        };
    )]
    pub enum UpdateOperator {
        #[from_string("++")]
        Increase,
        #[from_string("--")]
        Decrease,
    }
}

ast_struct! {
    pub struct ExprYield {
        pub span: Span,
        pub argument: Option<Box<Expr>>,
        pub delegate: bool,
    }
}

ast_struct! {
    pub struct ExprConditional {
        pub span: Span,
        pub condition: Box<Expr>,
        pub consequent: Box<Expr>,
        pub alternate: Box<Expr>,
    }
}

ast_struct! {
    pub struct ExprAwait {
        pub span: Span,
        pub argument: Box<Expr>,
    }
}

ast_struct! {
    pub struct ExprSequence {
        pub span: Span,
        pub expr: Vec<Expr>,
    }
}

ast_struct! {
    pub struct ExprFunction {
        pub span: Span,
        pub asynchronous: bool,
        pub generator: bool,
        pub identifier: Option<Ident>,
        pub parameters: FormalParameters,
        pub body: Body,
    }
}

ast_struct! {
    pub struct ExprArrowFunction {
        pub span: Span,
        pub asynchronous: bool,
        pub binding_parameter: bool,
        pub parameters: FormalParameters,
        pub body: ArrowFunctionBody,
    }
}

ast_node! {
    pub enum ArrowFunctionBody {
        Expr(Box<Expr>),
        Body(Body),
    }
}

ast_struct! {
    pub struct ExprParenthesized {
        pub span: Span,
        pub expression: Box<Expr>,
    }
}

ast_struct! {
    pub struct ExprMember {
        pub span: Span,
        pub object: MemberObject,
        pub property: MemberProperty,
    }
}

ast_struct! {
    pub struct ExprOptionalMember {
        pub span: Span,
        pub object: Box<Expr>,
        pub property: MemberProperty,
        pub optional: bool,
    }
}

ast_node! {
    pub enum MemberObject {
        Expr(Box<Expr>),
        Super(Super),
    }
}

ast_struct! {
    pub struct Super {
        pub span: Span,
    }
}

ast_node! {
    pub enum MemberProperty {
        Ident(Ident),
        Expr(Box<Expr>),
    }
}

ast_struct! {
    pub struct ExprMetaProperty {
        pub span: Span,
        pub meta: Ident,
        pub property: Ident,
    }
}

ast_struct! {
    pub struct ExprNew {
        pub span: Span,
        pub callee: Box<Expr>,
        pub arguments_span: Option<Span>,
        pub arguments: Vec<Argument>,
    }
}

ast_node! {
    pub enum Argument {
        Expr(Expr),
        Spread(Expr),
    }
}

ast_struct! {
    pub struct ExprAssignment {
        pub span: Span,
        pub operator: AssignmentOperator,
        pub left: Box<Expr>,
        pub right: Box<Expr>,
    }
}

ast_node! {
    #[derive(FromString)]
    #[from_string_macro("assignment_op")]
    #[from_string_macro_rules(
        ($variant:ident) => {
            $crate::AssignmentOperator::$variant
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
}

ast_struct! {
    pub struct ExprCall {
        pub span: Span,
        pub callee: Callee,
        pub arguments_span: Span,
        pub arguments: Vec<Argument>,
    }
}

ast_struct! {
    pub struct ExprOptionalCall {
        pub span: Span,
        pub callee: Box<Expr>,
        pub arguments_span: Span,
        pub arguments: Vec<Argument>,
        pub optional: bool,
    }
}

ast_node! {
    pub enum Callee {
        Super,
        Import,
        Expr(Box<Expr>),
    }
}

ast_struct! {
    pub struct ExprTaggedTemplate {
        pub span: Span,
        pub callee: Box<Expr>,
        pub template: LitTemplate,
    }
}
