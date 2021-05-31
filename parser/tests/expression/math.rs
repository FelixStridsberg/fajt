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