use fajt_lexer::token::Span;
use fajt_parser::ast::*;

#[test]
fn addition() {
    parser_test!(
        input: "a + b",
        expr_output: [
            ExprBinary {
                span: Span::new(0, 5),
                operator: BinaryOperator::Plus,
                left: ExprIdentifier::Ident(Ident::new("a", (0, 1))).into(),
                right: ExprIdentifier::Ident(Ident::new("b", (4, 5))).into(),
            }.into()
        ]
    );
}

#[test]
fn addition_nested() {
    parser_test!(
        input: "a + b + c + d",
        expr_output: [
            ExprBinary {
                span: Span::new(0, 13),
                operator: BinaryOperator::Plus,
                left: ExprBinary {
                    span: Span::new(0, 9),
                    operator: BinaryOperator::Plus,
                    left: ExprBinary {
                        span: Span::new(0, 5),
                        operator: BinaryOperator::Plus,
                        left: ExprIdentifier::Ident(Ident::new("a", (0, 1))).into(),
                        right: ExprIdentifier::Ident(Ident::new("b", (4, 5))).into(),
                    }.into(),
                    right: ExprIdentifier::Ident(Ident::new("c", (8, 9))).into(),
                }.into(),
                right: ExprIdentifier::Ident(Ident::new("d", (12, 13))).into(),
            }.into()
        ]
    );
}

#[test]
fn subtraction() {
    parser_test!(
        input: "a - b",
        expr_output: [
            ExprBinary {
                span: Span::new(0, 5),
                operator: BinaryOperator::Minus,
                left: ExprIdentifier::Ident(Ident::new("a", (0, 1))).into(),
                right: ExprIdentifier::Ident(Ident::new("b", (4, 5))).into(),
            }.into()
        ]
    );
}

#[test]
fn subtraction_nested() {
    parser_test!(
        input: "a - b - c",
        expr_output: [
            ExprBinary {
                span: Span::new(0, 9),
                operator: BinaryOperator::Minus,
                left: ExprBinary {
                    span: Span::new(0, 5),
                    operator: BinaryOperator::Minus,
                    left: ExprIdentifier::Ident(Ident::new("a", (0, 1))).into(),
                    right: ExprIdentifier::Ident(Ident::new("b", (4, 5))).into(),
                }.into(),
                right: ExprIdentifier::Ident(Ident::new("c", (8, 9))).into(),
            }.into()
        ]
    );
}

#[test]
fn multiplication() {
    parser_test!(
        input: "a * b * c",
        expr_output: [
            ExprBinary {
                span: Span::new(0, 9),
                operator: BinaryOperator::Multiplication,
                left: ExprBinary {
                    span: Span::new(0, 5),
                    operator: BinaryOperator::Multiplication,
                    left: ExprIdentifier::Ident(Ident::new("a", (0, 1))).into(),
                    right: ExprIdentifier::Ident(Ident::new("b", (4, 5))).into(),
                }.into(),
                right: ExprIdentifier::Ident(Ident::new("c", (8, 9))).into(),
            }.into()
        ]
    );
}

#[test]
fn division() {
    parser_test!(
        input: "a / b / c",
        expr_output: [
            ExprBinary {
                span: Span::new(0, 9),
                operator: BinaryOperator::Division,
                left: ExprBinary {
                    span: Span::new(0, 5),
                    operator: BinaryOperator::Division,
                    left: ExprIdentifier::Ident(Ident::new("a", (0, 1))).into(),
                    right: ExprIdentifier::Ident(Ident::new("b", (4, 5))).into(),
                }.into(),
                right: ExprIdentifier::Ident(Ident::new("c", (8, 9))).into(),
            }.into()
        ]
    );
}

#[test]
fn modulus() {
    parser_test!(
        input: "a % b % c",
        expr_output: [
            ExprBinary {
                span: Span::new(0, 9),
                operator: BinaryOperator::Modulus,
                left: ExprBinary {
                    span: Span::new(0, 5),
                    operator: BinaryOperator::Modulus,
                    left: ExprIdentifier::Ident(Ident::new("a", (0, 1))).into(),
                    right: ExprIdentifier::Ident(Ident::new("b", (4, 5))).into(),
                }.into(),
                right: ExprIdentifier::Ident(Ident::new("c", (8, 9))).into(),
            }.into()
        ]
    );
}

#[test]
fn exponent() {
    parser_test!(
        input: "a ** b ** c",
        expr_output: [
            ExprBinary {
                span: Span::new(0, 11),
                operator: BinaryOperator::Exponent,
                left: ExprBinary {
                    span: Span::new(0, 6),
                    operator: BinaryOperator::Exponent,
                    left: ExprIdentifier::Ident(Ident::new("a", (0, 1))).into(),
                    right: ExprIdentifier::Ident(Ident::new("b", (5, 6))).into(),
                }.into(),
                right: ExprIdentifier::Ident(Ident::new("c", (10, 11))).into(),
            }.into()
        ]
    );
}

#[test]
fn shift_left() {
    parser_test!(
        input: "a << b << c",
        expr_output: [
            ExprBinary {
                span: Span::new(0, 11),
                operator: BinaryOperator::ShiftLeft,
                left: ExprBinary {
                    span: Span::new(0, 6),
                    operator: BinaryOperator::ShiftLeft,
                    left: ExprIdentifier::Ident(Ident::new("a", (0, 1))).into(),
                    right: ExprIdentifier::Ident(Ident::new("b", (5, 6))).into(),
                }.into(),
                right: ExprIdentifier::Ident(Ident::new("c", (10, 11))).into(),
            }.into()
        ]
    );
}

#[test]
fn shift_right() {
    parser_test!(
        input: "a >> b >> c",
        expr_output: [
            ExprBinary {
                span: Span::new(0, 11),
                operator: BinaryOperator::ShiftRight,
                left: ExprBinary {
                    span: Span::new(0, 6),
                    operator: BinaryOperator::ShiftRight,
                    left: ExprIdentifier::Ident(Ident::new("a", (0, 1))).into(),
                    right: ExprIdentifier::Ident(Ident::new("b", (5, 6))).into(),
                }.into(),
                right: ExprIdentifier::Ident(Ident::new("c", (10, 11))).into(),
            }.into()
        ]
    );
}

#[test]
fn shift_right_unsigned() {
    parser_test!(
        input: "a >>> b >>> c",
        expr_output: [
            ExprBinary {
                span: Span::new(0, 13),
                operator: BinaryOperator::ShiftRightUnsigned,
                left: ExprBinary {
                    span: Span::new(0, 7),
                    operator: BinaryOperator::ShiftRightUnsigned,
                    left: ExprIdentifier::Ident(Ident::new("a", (0, 1))).into(),
                    right: ExprIdentifier::Ident(Ident::new("b", (6, 7))).into(),
                }.into(),
                right: ExprIdentifier::Ident(Ident::new("c", (12, 13))).into(),
            }.into()
        ]
    );
}

#[test]
fn less_than() {
    parser_test!(
        input: "a < b < c",
        expr_output: [
            ExprBinary {
                span: Span::new(0, 9),
                operator: BinaryOperator::LessThan,
                left: ExprBinary {
                    span: Span::new(0, 5),
                    operator: BinaryOperator::LessThan,
                    left: ExprIdentifier::Ident(Ident::new("a", (0, 1))).into(),
                    right: ExprIdentifier::Ident(Ident::new("b", (4, 5))).into(),
                }.into(),
                right: ExprIdentifier::Ident(Ident::new("c", (8, 9))).into(),
            }.into()
        ]
    );
}

#[test]
fn more_than() {
    parser_test!(
        input: "a > b > c",
        expr_output: [
            ExprBinary {
                span: Span::new(0, 9),
                operator: BinaryOperator::MoreThan,
                left: ExprBinary {
                    span: Span::new(0, 5),
                    operator: BinaryOperator::MoreThan,
                    left: ExprIdentifier::Ident(Ident::new("a", (0, 1))).into(),
                    right: ExprIdentifier::Ident(Ident::new("b", (4, 5))).into(),
                }.into(),
                right: ExprIdentifier::Ident(Ident::new("c", (8, 9))).into(),
            }.into()
        ]
    );
}

#[test]
fn less_than_equals() {
    parser_test!(
        input: "a <= b <= c",
        expr_output: [
            ExprBinary {
                span: Span::new(0, 11),
                operator: BinaryOperator::LessThanEquals,
                left: ExprBinary {
                    span: Span::new(0, 6),
                    operator: BinaryOperator::LessThanEquals,
                    left: ExprIdentifier::Ident(Ident::new("a", (0, 1))).into(),
                    right: ExprIdentifier::Ident(Ident::new("b", (5, 6))).into(),
                }.into(),
                right: ExprIdentifier::Ident(Ident::new("c", (10, 11))).into(),
            }.into()
        ]
    );
}

#[test]
fn more_than_equals() {
    parser_test!(
        input: "a >= b >= c",
        expr_output: [
            ExprBinary {
                span: Span::new(0, 11),
                operator: BinaryOperator::MoreThanEquals,
                left: ExprBinary {
                    span: Span::new(0, 6),
                    operator: BinaryOperator::MoreThanEquals,
                    left: ExprIdentifier::Ident(Ident::new("a", (0, 1))).into(),
                    right: ExprIdentifier::Ident(Ident::new("b", (5, 6))).into(),
                }.into(),
                right: ExprIdentifier::Ident(Ident::new("c", (10, 11))).into(),
            }.into()
        ]
    );
}

#[test]
fn instance_of() {
    parser_test!(
        input: "a instanceof b instanceof c",
        expr_output: [
            ExprBinary {
                span: Span::new(0, 27),
                operator: BinaryOperator::InstanceOf,
                left: ExprBinary {
                    span: Span::new(0, 14),
                    operator: BinaryOperator::InstanceOf,
                    left: ExprIdentifier::Ident(Ident::new("a", (0, 1))).into(),
                    right: ExprIdentifier::Ident(Ident::new("b", (13, 14))).into(),
                }.into(),
                right: ExprIdentifier::Ident(Ident::new("c", (26, 27))).into(),
            }.into()
        ]
    );
}

#[test]
fn r#in() {
    parser_test!(
        input: "a in b in c",
        expr_output: [
            ExprBinary {
                span: Span::new(0, 11),
                operator: BinaryOperator::In,
                left: ExprBinary {
                    span: Span::new(0, 6),
                    operator: BinaryOperator::In,
                    left: ExprIdentifier::Ident(Ident::new("a", (0, 1))).into(),
                    right: ExprIdentifier::Ident(Ident::new("b", (5, 6))).into(),
                }.into(),
                right: ExprIdentifier::Ident(Ident::new("c", (10, 11))).into(),
            }.into()
        ]
    );
}

#[test]
fn equal() {
    parser_test!(
        input: "a == b == c",
        expr_output: [
            ExprBinary {
                span: Span::new(0, 11),
                operator: BinaryOperator::Equal,
                left: ExprBinary {
                    span: Span::new(0, 6),
                    operator: BinaryOperator::Equal,
                    left: ExprIdentifier::Ident(Ident::new("a", (0, 1))).into(),
                    right: ExprIdentifier::Ident(Ident::new("b", (5, 6))).into(),
                }.into(),
                right: ExprIdentifier::Ident(Ident::new("c", (10, 11))).into(),
            }.into()
        ]
    );
}

#[test]
fn not_equal() {
    parser_test!(
        input: "a != b != c",
        expr_output: [
            ExprBinary {
                span: Span::new(0, 11),
                operator: BinaryOperator::NotEqual,
                left: ExprBinary {
                    span: Span::new(0, 6),
                    operator: BinaryOperator::NotEqual,
                    left: ExprIdentifier::Ident(Ident::new("a", (0, 1))).into(),
                    right: ExprIdentifier::Ident(Ident::new("b", (5, 6))).into(),
                }.into(),
                right: ExprIdentifier::Ident(Ident::new("c", (10, 11))).into(),
            }.into()
        ]
    );
}

#[test]
fn strict_equal() {
    parser_test!(
        input: "a === b === c",
        expr_output: [
            ExprBinary {
                span: Span::new(0, 13),
                operator: BinaryOperator::StrictEqual,
                left: ExprBinary {
                    span: Span::new(0, 7),
                    operator: BinaryOperator::StrictEqual,
                    left: ExprIdentifier::Ident(Ident::new("a", (0, 1))).into(),
                    right: ExprIdentifier::Ident(Ident::new("b", (6, 7))).into(),
                }.into(),
                right: ExprIdentifier::Ident(Ident::new("c", (12, 13))).into(),
            }.into()
        ]
    );
}

#[test]
fn strict_not_equal() {
    parser_test!(
        input: "a !== b !== c",
        expr_output: [
            ExprBinary {
                span: Span::new(0, 13),
                operator: BinaryOperator::StrictNotEqual,
                left: ExprBinary {
                    span: Span::new(0, 7),
                    operator: BinaryOperator::StrictNotEqual,
                    left: ExprIdentifier::Ident(Ident::new("a", (0, 1))).into(),
                    right: ExprIdentifier::Ident(Ident::new("b", (6, 7))).into(),
                }.into(),
                right: ExprIdentifier::Ident(Ident::new("c", (12, 13))).into(),
            }.into()
        ]
    );
}

#[test]
fn bitwise_and() {
    parser_test!(
        input: "a & b & c",
        expr_output: [
            ExprBinary {
                span: Span::new(0, 9),
                operator: BinaryOperator::BitwiseAnd,
                left: ExprBinary {
                    span: Span::new(0, 5),
                    operator: BinaryOperator::BitwiseAnd,
                    left: ExprIdentifier::Ident(Ident::new("a", (0, 1))).into(),
                    right: ExprIdentifier::Ident(Ident::new("b", (4, 5))).into(),
                }.into(),
                right: ExprIdentifier::Ident(Ident::new("c", (8, 9))).into(),
            }.into()
        ]
    );
}

#[test]
fn bitwise_xor() {
    parser_test!(
        input: "a ^ b ^ c",
        expr_output: [
            ExprBinary {
                span: Span::new(0, 9),
                operator: BinaryOperator::BitwiseXOR,
                left: ExprBinary {
                    span: Span::new(0, 5),
                    operator: BinaryOperator::BitwiseXOR,
                    left: ExprIdentifier::Ident(Ident::new("a", (0, 1))).into(),
                    right: ExprIdentifier::Ident(Ident::new("b", (4, 5))).into(),
                }.into(),
                right: ExprIdentifier::Ident(Ident::new("c", (8, 9))).into(),
            }.into()
        ]
    );
}

#[test]
fn bitwise_or() {
    parser_test!(
        input: "a | b | c",
        expr_output: [
            ExprBinary {
                span: Span::new(0, 9),
                operator: BinaryOperator::BitwiseOR,
                left: ExprBinary {
                    span: Span::new(0, 5),
                    operator: BinaryOperator::BitwiseOR,
                    left: ExprIdentifier::Ident(Ident::new("a", (0, 1))).into(),
                    right: ExprIdentifier::Ident(Ident::new("b", (4, 5))).into(),
                }.into(),
                right: ExprIdentifier::Ident(Ident::new("c", (8, 9))).into(),
            }.into()
        ]
    );
}

#[test]
fn logical_or() {
    parser_test!(
        input: "a || b || c",
        expr_output: [
            ExprLogical {
                span: Span::new(0, 11),
                operator: LogicalOperator::Or,
                left: ExprLogical {
                    span: Span::new(0, 6),
                    operator: LogicalOperator::Or,
                    left: ExprIdentifier::Ident(Ident::new("a", (0, 1))).into(),
                    right: ExprIdentifier::Ident(Ident::new("b", (5, 6))).into(),
                }.into(),
                right: ExprIdentifier::Ident(Ident::new("c", (10, 11))).into(),
            }.into()
        ]
    );
}

#[test]
fn logical_and() {
    parser_test!(
        input: "a && b && c",
        expr_output: [
            ExprLogical {
                span: Span::new(0, 11),
                operator: LogicalOperator::And,
                left: ExprLogical {
                    span: Span::new(0, 6),
                    operator: LogicalOperator::And,
                    left: ExprIdentifier::Ident(Ident::new("a", (0, 1))).into(),
                    right: ExprIdentifier::Ident(Ident::new("b", (5, 6))).into(),
                }.into(),
                right: ExprIdentifier::Ident(Ident::new("c", (10, 11))).into(),
            }.into()
        ]
    );
}

#[test]
fn coalesce() {
    parser_test!(
        input: "a ?? b ?? c",
        expr_output: [
            ExprLogical {
                span: Span::new(0, 11),
                operator: LogicalOperator::Coalesce,
                left: ExprLogical {
                    span: Span::new(0, 6),
                    operator: LogicalOperator::Coalesce,
                    left: ExprIdentifier::Ident(Ident::new("a", (0, 1))).into(),
                    right: ExprIdentifier::Ident(Ident::new("b", (5, 6))).into(),
                }.into(),
                right: ExprIdentifier::Ident(Ident::new("c", (10, 11))).into(),
            }.into()
        ]
    );
}
