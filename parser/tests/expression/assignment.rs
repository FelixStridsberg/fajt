use fajt_lexer::token::Span;
use fajt_parser::ast::*;

#[test]
fn assign() {
    parser_test!(
        input: "a = b",
        expr_output: [
            AssignmentExpression {
                span: Span::new(0, 5),
                operator: AssignmentOperator::Assign,
                left: IdentifierReference::Ident(Ident::new("a", (0, 1))).into(),
                right: IdentifierReference::Ident(Ident::new("b", (4, 5))).into(),
            }.into()
        ]
    );
}

#[test]
fn assign_multiply() {
    parser_test!(
        input: "a *= b",
        expr_output: [
            AssignmentExpression {
                span: Span::new(0, 6),
                operator: AssignmentOperator::Multiply,
                left: IdentifierReference::Ident(Ident::new("a", (0, 1))).into(),
                right: IdentifierReference::Ident(Ident::new("b", (5, 6))).into(),
            }.into()
        ]
    );
}

#[test]
fn assign_power() {
    parser_test!(
        input: "a **= b",
        expr_output: [
            AssignmentExpression {
                span: Span::new(0, 7),
                operator: AssignmentOperator::Power,
                left: IdentifierReference::Ident(Ident::new("a", (0, 1))).into(),
                right: IdentifierReference::Ident(Ident::new("b", (6, 7))).into(),
            }.into()
        ]
    );
}

#[test]
fn assign_divide() {
    parser_test!(
        input: "a /= b",
        expr_output: [
            AssignmentExpression {
                span: Span::new(0, 6),
                operator: AssignmentOperator::Divide,
                left: IdentifierReference::Ident(Ident::new("a", (0, 1))).into(),
                right: IdentifierReference::Ident(Ident::new("b", (5, 6))).into(),
            }.into()
        ]
    );
}

#[test]
fn assign_modulus() {
    parser_test!(
        input: "a %= b",
        expr_output: [
            AssignmentExpression {
                span: Span::new(0, 6),
                operator: AssignmentOperator::Modulus,
                left: IdentifierReference::Ident(Ident::new("a", (0, 1))).into(),
                right: IdentifierReference::Ident(Ident::new("b", (5, 6))).into(),
            }.into()
        ]
    );
}

#[test]
fn assign_add() {
    parser_test!(
        input: "a += b",
        expr_output: [
            AssignmentExpression {
                span: Span::new(0, 6),
                operator: AssignmentOperator::Add,
                left: IdentifierReference::Ident(Ident::new("a", (0, 1))).into(),
                right: IdentifierReference::Ident(Ident::new("b", (5, 6))).into(),
            }.into()
        ]
    );
}

#[test]
fn assign_sub() {
    parser_test!(
        input: "a -= b",
        expr_output: [
            AssignmentExpression {
                span: Span::new(0, 6),
                operator: AssignmentOperator::Sub,
                left: IdentifierReference::Ident(Ident::new("a", (0, 1))).into(),
                right: IdentifierReference::Ident(Ident::new("b", (5, 6))).into(),
            }.into()
        ]
    );
}

#[test]
fn assign_left_shift() {
    parser_test!(
        input: "a <<= b",
        expr_output: [
            AssignmentExpression {
                span: Span::new(0, 7),
                operator: AssignmentOperator::LeftShift,
                left: IdentifierReference::Ident(Ident::new("a", (0, 1))).into(),
                right: IdentifierReference::Ident(Ident::new("b", (6, 7))).into(),
            }.into()
        ]
    );
}

#[test]
fn assign_right_shift() {
    parser_test!(
        input: "a >>= b",
        expr_output: [
            AssignmentExpression {
                span: Span::new(0, 7),
                operator: AssignmentOperator::RightShift,
                left: IdentifierReference::Ident(Ident::new("a", (0, 1))).into(),
                right: IdentifierReference::Ident(Ident::new("b", (6, 7))).into(),
            }.into()
        ]
    );
}

#[test]
fn assign_unsigned_right_shift() {
    parser_test!(
        input: "a >>>= b",
        expr_output: [
            AssignmentExpression {
                span: Span::new(0, 8),
                operator: AssignmentOperator::UnsignedRightShift,
                left: IdentifierReference::Ident(Ident::new("a", (0, 1))).into(),
                right: IdentifierReference::Ident(Ident::new("b", (7, 8))).into(),
            }.into()
        ]
    );
}

#[test]
fn assign_bitwise_and() {
    parser_test!(
        input: "a &= b",
        expr_output: [
            AssignmentExpression {
                span: Span::new(0, 6),
                operator: AssignmentOperator::BitwiseAnd,
                left: IdentifierReference::Ident(Ident::new("a", (0, 1))).into(),
                right: IdentifierReference::Ident(Ident::new("b", (5, 6))).into(),
            }.into()
        ]
    );
}

#[test]
fn assign_bitwise_xor() {
    parser_test!(
        input: "a ^= b",
        expr_output: [
            AssignmentExpression {
                span: Span::new(0, 6),
                operator: AssignmentOperator::BitwiseXOr,
                left: IdentifierReference::Ident(Ident::new("a", (0, 1))).into(),
                right: IdentifierReference::Ident(Ident::new("b", (5, 6))).into(),
            }.into()
        ]
    );
}

#[test]
fn assign_bitwise_or() {
    parser_test!(
        input: "a |= b",
        expr_output: [
            AssignmentExpression {
                span: Span::new(0, 6),
                operator: AssignmentOperator::BitwiseOr,
                left: IdentifierReference::Ident(Ident::new("a", (0, 1))).into(),
                right: IdentifierReference::Ident(Ident::new("b", (5, 6))).into(),
            }.into()
        ]
    );
}
