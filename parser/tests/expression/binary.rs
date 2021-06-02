use fajt_lexer::token::Span;
use fajt_parser::ast::*;

#[test]
fn addition() {
    parser_test!(
        input: "a + b",
        expr_output: [
            BinaryExpression {
                span: Span::new(0, 5),
                operator: BinaryOperator::Plus,
                left: IdentifierReference::Ident(Ident::new("a", (0, 1))).into(),
                right: IdentifierReference::Ident(Ident::new("b", (4, 5))).into(),
            }.into()
        ]
    );
}

#[test]
fn addition_nested() {
    parser_test!(
        input: "a + b + c + d",
        expr_output: [
            BinaryExpression {
                span: Span::new(0, 13),
                operator: BinaryOperator::Plus,
                left: BinaryExpression {
                    span: Span::new(0, 9),
                    operator: BinaryOperator::Plus,
                    left: BinaryExpression {
                        span: Span::new(0, 5),
                        operator: BinaryOperator::Plus,
                        left: IdentifierReference::Ident(Ident::new("a", (0, 1))).into(),
                        right: IdentifierReference::Ident(Ident::new("b", (4, 5))).into(),
                    }.into(),
                    right: IdentifierReference::Ident(Ident::new("c", (8, 9))).into(),
                }.into(),
                right: IdentifierReference::Ident(Ident::new("d", (12, 13))).into(),
            }.into()
        ]
    );
}

#[test]
fn subtraction() {
    parser_test!(
        input: "a - b",
        expr_output: [
            BinaryExpression {
                span: Span::new(0, 5),
                operator: BinaryOperator::Minus,
                left: IdentifierReference::Ident(Ident::new("a", (0, 1))).into(),
                right: IdentifierReference::Ident(Ident::new("b", (4, 5))).into(),
            }.into()
        ]
    );
}

#[test]
fn subtraction_nested() {
    parser_test!(
        input: "a - b - c",
        expr_output: [
            BinaryExpression {
                span: Span::new(0, 9),
                operator: BinaryOperator::Minus,
                left: BinaryExpression {
                    span: Span::new(0, 5),
                    operator: BinaryOperator::Minus,
                    left: IdentifierReference::Ident(Ident::new("a", (0, 1))).into(),
                    right: IdentifierReference::Ident(Ident::new("b", (4, 5))).into(),
                }.into(),
                right: IdentifierReference::Ident(Ident::new("c", (8, 9))).into(),
            }.into()
        ]
    );
}

#[test]
fn multiplication() {
    parser_test!(
        input: "a * b * c",
        expr_output: [
            BinaryExpression {
                span: Span::new(0, 9),
                operator: BinaryOperator::Multiplication,
                left: BinaryExpression {
                    span: Span::new(0, 5),
                    operator: BinaryOperator::Multiplication,
                    left: IdentifierReference::Ident(Ident::new("a", (0, 1))).into(),
                    right: IdentifierReference::Ident(Ident::new("b", (4, 5))).into(),
                }.into(),
                right: IdentifierReference::Ident(Ident::new("c", (8, 9))).into(),
            }.into()
        ]
    );
}

#[test]
fn division() {
    parser_test!(
        input: "a / b / c",
        expr_output: [
            BinaryExpression {
                span: Span::new(0, 9),
                operator: BinaryOperator::Division,
                left: BinaryExpression {
                    span: Span::new(0, 5),
                    operator: BinaryOperator::Division,
                    left: IdentifierReference::Ident(Ident::new("a", (0, 1))).into(),
                    right: IdentifierReference::Ident(Ident::new("b", (4, 5))).into(),
                }.into(),
                right: IdentifierReference::Ident(Ident::new("c", (8, 9))).into(),
            }.into()
        ]
    );
}

#[test]
fn modulus() {
    parser_test!(
        input: "a % b % c",
        expr_output: [
            BinaryExpression {
                span: Span::new(0, 9),
                operator: BinaryOperator::Modulus,
                left: BinaryExpression {
                    span: Span::new(0, 5),
                    operator: BinaryOperator::Modulus,
                    left: IdentifierReference::Ident(Ident::new("a", (0, 1))).into(),
                    right: IdentifierReference::Ident(Ident::new("b", (4, 5))).into(),
                }.into(),
                right: IdentifierReference::Ident(Ident::new("c", (8, 9))).into(),
            }.into()
        ]
    );
}

#[test]
fn exponent() {
    parser_test!(
        input: "a ** b ** c",
        expr_output: [
            BinaryExpression {
                span: Span::new(0, 11),
                operator: BinaryOperator::Exponent,
                left: BinaryExpression {
                    span: Span::new(0, 6),
                    operator: BinaryOperator::Exponent,
                    left: IdentifierReference::Ident(Ident::new("a", (0, 1))).into(),
                    right: IdentifierReference::Ident(Ident::new("b", (5, 6))).into(),
                }.into(),
                right: IdentifierReference::Ident(Ident::new("c", (10, 11))).into(),
            }.into()
        ]
    );
}

#[test]
fn shift_left() {
    parser_test!(
        input: "a << b << c",
        expr_output: [
            BinaryExpression {
                span: Span::new(0, 11),
                operator: BinaryOperator::ShiftLeft,
                left: BinaryExpression {
                    span: Span::new(0, 6),
                    operator: BinaryOperator::ShiftLeft,
                    left: IdentifierReference::Ident(Ident::new("a", (0, 1))).into(),
                    right: IdentifierReference::Ident(Ident::new("b", (5, 6))).into(),
                }.into(),
                right: IdentifierReference::Ident(Ident::new("c", (10, 11))).into(),
            }.into()
        ]
    );
}

#[test]
fn shift_right() {
    parser_test!(
        input: "a >> b >> c",
        expr_output: [
            BinaryExpression {
                span: Span::new(0, 11),
                operator: BinaryOperator::ShiftRight,
                left: BinaryExpression {
                    span: Span::new(0, 6),
                    operator: BinaryOperator::ShiftRight,
                    left: IdentifierReference::Ident(Ident::new("a", (0, 1))).into(),
                    right: IdentifierReference::Ident(Ident::new("b", (5, 6))).into(),
                }.into(),
                right: IdentifierReference::Ident(Ident::new("c", (10, 11))).into(),
            }.into()
        ]
    );
}

#[test]
fn shift_right_unsigned() {
    parser_test!(
        input: "a >>> b >>> c",
        expr_output: [
            BinaryExpression {
                span: Span::new(0, 13),
                operator: BinaryOperator::ShiftRightUnsigned,
                left: BinaryExpression {
                    span: Span::new(0, 7),
                    operator: BinaryOperator::ShiftRightUnsigned,
                    left: IdentifierReference::Ident(Ident::new("a", (0, 1))).into(),
                    right: IdentifierReference::Ident(Ident::new("b", (6, 7))).into(),
                }.into(),
                right: IdentifierReference::Ident(Ident::new("c", (12, 13))).into(),
            }.into()
        ]
    );
}

#[test]
fn less_than() {
    parser_test!(
        input: "a < b < c",
        expr_output: [
            BinaryExpression {
                span: Span::new(0, 9),
                operator: BinaryOperator::LessThan,
                left: BinaryExpression {
                    span: Span::new(0, 5),
                    operator: BinaryOperator::LessThan,
                    left: IdentifierReference::Ident(Ident::new("a", (0, 1))).into(),
                    right: IdentifierReference::Ident(Ident::new("b", (4, 5))).into(),
                }.into(),
                right: IdentifierReference::Ident(Ident::new("c", (8, 9))).into(),
            }.into()
        ]
    );
}

#[test]
fn more_than() {
    parser_test!(
        input: "a > b > c",
        expr_output: [
            BinaryExpression {
                span: Span::new(0, 9),
                operator: BinaryOperator::MoreThan,
                left: BinaryExpression {
                    span: Span::new(0, 5),
                    operator: BinaryOperator::MoreThan,
                    left: IdentifierReference::Ident(Ident::new("a", (0, 1))).into(),
                    right: IdentifierReference::Ident(Ident::new("b", (4, 5))).into(),
                }.into(),
                right: IdentifierReference::Ident(Ident::new("c", (8, 9))).into(),
            }.into()
        ]
    );
}

#[test]
fn less_than_equals() {
    parser_test!(
        input: "a <= b <= c",
        expr_output: [
            BinaryExpression {
                span: Span::new(0, 11),
                operator: BinaryOperator::LessThanEquals,
                left: BinaryExpression {
                    span: Span::new(0, 6),
                    operator: BinaryOperator::LessThanEquals,
                    left: IdentifierReference::Ident(Ident::new("a", (0, 1))).into(),
                    right: IdentifierReference::Ident(Ident::new("b", (5, 6))).into(),
                }.into(),
                right: IdentifierReference::Ident(Ident::new("c", (10, 11))).into(),
            }.into()
        ]
    );
}

#[test]
fn more_than_equals() {
    parser_test!(
        input: "a >= b >= c",
        expr_output: [
            BinaryExpression {
                span: Span::new(0, 11),
                operator: BinaryOperator::MoreThanEquals,
                left: BinaryExpression {
                    span: Span::new(0, 6),
                    operator: BinaryOperator::MoreThanEquals,
                    left: IdentifierReference::Ident(Ident::new("a", (0, 1))).into(),
                    right: IdentifierReference::Ident(Ident::new("b", (5, 6))).into(),
                }.into(),
                right: IdentifierReference::Ident(Ident::new("c", (10, 11))).into(),
            }.into()
        ]
    );
}

#[test]
fn instance_of() {
    parser_test!(
        input: "a instanceof b instanceof c",
        expr_output: [
            BinaryExpression {
                span: Span::new(0, 27),
                operator: BinaryOperator::InstanceOf,
                left: BinaryExpression {
                    span: Span::new(0, 14),
                    operator: BinaryOperator::InstanceOf,
                    left: IdentifierReference::Ident(Ident::new("a", (0, 1))).into(),
                    right: IdentifierReference::Ident(Ident::new("b", (13, 14))).into(),
                }.into(),
                right: IdentifierReference::Ident(Ident::new("c", (26, 27))).into(),
            }.into()
        ]
    );
}

#[test]
fn in_() {
    parser_test!(
        input: "a in b in c",
        expr_output: [
            BinaryExpression {
                span: Span::new(0, 11),
                operator: BinaryOperator::In,
                left: BinaryExpression {
                    span: Span::new(0, 6),
                    operator: BinaryOperator::In,
                    left: IdentifierReference::Ident(Ident::new("a", (0, 1))).into(),
                    right: IdentifierReference::Ident(Ident::new("b", (5, 6))).into(),
                }.into(),
                right: IdentifierReference::Ident(Ident::new("c", (10, 11))).into(),
            }.into()
        ]
    );
}

#[test]
fn equal() {
    parser_test!(
        input: "a == b == c",
        expr_output: [
            BinaryExpression {
                span: Span::new(0, 11),
                operator: BinaryOperator::Equal,
                left: BinaryExpression {
                    span: Span::new(0, 6),
                    operator: BinaryOperator::Equal,
                    left: IdentifierReference::Ident(Ident::new("a", (0, 1))).into(),
                    right: IdentifierReference::Ident(Ident::new("b", (5, 6))).into(),
                }.into(),
                right: IdentifierReference::Ident(Ident::new("c", (10, 11))).into(),
            }.into()
        ]
    );
}

#[test]
fn not_equal() {
    parser_test!(
        input: "a != b != c",
        expr_output: [
            BinaryExpression {
                span: Span::new(0, 11),
                operator: BinaryOperator::NotEqual,
                left: BinaryExpression {
                    span: Span::new(0, 6),
                    operator: BinaryOperator::NotEqual,
                    left: IdentifierReference::Ident(Ident::new("a", (0, 1))).into(),
                    right: IdentifierReference::Ident(Ident::new("b", (5, 6))).into(),
                }.into(),
                right: IdentifierReference::Ident(Ident::new("c", (10, 11))).into(),
            }.into()
        ]
    );
}

#[test]
fn strict_equal() {
    parser_test!(
        input: "a === b === c",
        expr_output: [
            BinaryExpression {
                span: Span::new(0, 13),
                operator: BinaryOperator::StrictEqual,
                left: BinaryExpression {
                    span: Span::new(0, 7),
                    operator: BinaryOperator::StrictEqual,
                    left: IdentifierReference::Ident(Ident::new("a", (0, 1))).into(),
                    right: IdentifierReference::Ident(Ident::new("b", (6, 7))).into(),
                }.into(),
                right: IdentifierReference::Ident(Ident::new("c", (12, 13))).into(),
            }.into()
        ]
    );
}

#[test]
fn strict_not_equal() {
    parser_test!(
        input: "a !== b !== c",
        expr_output: [
            BinaryExpression {
                span: Span::new(0, 13),
                operator: BinaryOperator::StrictNotEqual,
                left: BinaryExpression {
                    span: Span::new(0, 7),
                    operator: BinaryOperator::StrictNotEqual,
                    left: IdentifierReference::Ident(Ident::new("a", (0, 1))).into(),
                    right: IdentifierReference::Ident(Ident::new("b", (6, 7))).into(),
                }.into(),
                right: IdentifierReference::Ident(Ident::new("c", (12, 13))).into(),
            }.into()
        ]
    );
}

#[test]
fn bitwise_and() {
    parser_test!(
        input: "a & b & c",
        expr_output: [
            BinaryExpression {
                span: Span::new(0, 9),
                operator: BinaryOperator::BitwiseAnd,
                left: BinaryExpression {
                    span: Span::new(0, 5),
                    operator: BinaryOperator::BitwiseAnd,
                    left: IdentifierReference::Ident(Ident::new("a", (0, 1))).into(),
                    right: IdentifierReference::Ident(Ident::new("b", (4, 5))).into(),
                }.into(),
                right: IdentifierReference::Ident(Ident::new("c", (8, 9))).into(),
            }.into()
        ]
    );
}

#[test]
fn bitwise_xor() {
    parser_test!(
        input: "a ^ b ^ c",
        expr_output: [
            BinaryExpression {
                span: Span::new(0, 9),
                operator: BinaryOperator::BitwiseXOR,
                left: BinaryExpression {
                    span: Span::new(0, 5),
                    operator: BinaryOperator::BitwiseXOR,
                    left: IdentifierReference::Ident(Ident::new("a", (0, 1))).into(),
                    right: IdentifierReference::Ident(Ident::new("b", (4, 5))).into(),
                }.into(),
                right: IdentifierReference::Ident(Ident::new("c", (8, 9))).into(),
            }.into()
        ]
    );
}

#[test]
fn bitwise_or() {
    parser_test!(
        input: "a | b | c",
        expr_output: [
            BinaryExpression {
                span: Span::new(0, 9),
                operator: BinaryOperator::BitwiseOR,
                left: BinaryExpression {
                    span: Span::new(0, 5),
                    operator: BinaryOperator::BitwiseOR,
                    left: IdentifierReference::Ident(Ident::new("a", (0, 1))).into(),
                    right: IdentifierReference::Ident(Ident::new("b", (4, 5))).into(),
                }.into(),
                right: IdentifierReference::Ident(Ident::new("c", (8, 9))).into(),
            }.into()
        ]
    );
}

#[test]
fn logical_or() {
    parser_test!(
        input: "a || b || c",
        expr_output: [
            LogicalExpression {
                span: Span::new(0, 11),
                operator: LogicalOperator::Or,
                left: LogicalExpression {
                    span: Span::new(0, 6),
                    operator: LogicalOperator::Or,
                    left: IdentifierReference::Ident(Ident::new("a", (0, 1))).into(),
                    right: IdentifierReference::Ident(Ident::new("b", (5, 6))).into(),
                }.into(),
                right: IdentifierReference::Ident(Ident::new("c", (10, 11))).into(),
            }.into()
        ]
    );
}

#[test]
fn logical_and() {
    parser_test!(
        input: "a && b && c",
        expr_output: [
            LogicalExpression {
                span: Span::new(0, 11),
                operator: LogicalOperator::And,
                left: LogicalExpression {
                    span: Span::new(0, 6),
                    operator: LogicalOperator::And,
                    left: IdentifierReference::Ident(Ident::new("a", (0, 1))).into(),
                    right: IdentifierReference::Ident(Ident::new("b", (5, 6))).into(),
                }.into(),
                right: IdentifierReference::Ident(Ident::new("c", (10, 11))).into(),
            }.into()
        ]
    );
}

#[test]
fn coalesce() {
    parser_test!(
        input: "a ?? b ?? c",
        expr_output: [
            LogicalExpression {
                span: Span::new(0, 11),
                operator: LogicalOperator::Coalesce,
                left: LogicalExpression {
                    span: Span::new(0, 6),
                    operator: LogicalOperator::Coalesce,
                    left: IdentifierReference::Ident(Ident::new("a", (0, 1))).into(),
                    right: IdentifierReference::Ident(Ident::new("b", (5, 6))).into(),
                }.into(),
                right: IdentifierReference::Ident(Ident::new("c", (10, 11))).into(),
            }.into()
        ]
    );
}
