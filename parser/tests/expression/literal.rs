mod array;
mod object;

use fajt_lexer::token::Span;
use fajt_parser::ast::*;

#[test]
fn this() {
    parser_test!(
        input: "this",
        expr_output: [
            ThisExpression {
                span: Span::new(0, 4),
            }.into()
        ]
    );
}

#[test]
fn null_literal() {
    parser_test!(
        input: "null",
        expr_output: [
            LiteralExpression {
                span: Span::new(0, 4),
                literal: Literal::Null,
            }.into()
        ]
    );
}

#[test]
fn boolean_true_literal() {
    parser_test!(
        input: "true",
        expr_output: [
            LiteralExpression {
                span: Span::new(0, 4),
                literal: Literal::Boolean(true),
            }.into()
        ]
    );
}

#[test]
fn boolean_false_literal() {
    parser_test!(
        input: "false",
        expr_output: [
            LiteralExpression {
                span: Span::new(0, 5),
                literal: Literal::Boolean(false),
            }.into()
        ]
    );
}

#[test]
fn string_literal_double_quote() {
    parser_test!(
        input: r#""this is string""#,
        expr_output: [
            LiteralExpression {
                span: Span::new(0, 16),
                literal: Literal::String("this is string".to_owned(), '"'),
            }.into()
        ]
    );
}

#[test]
fn string_literal_single_quote() {
    parser_test!(
        input: "'this is string'",
        expr_output: [
            LiteralExpression {
                span: Span::new(0, 16),
                literal: Literal::String("this is string".to_owned(), '\''),
            }.into()
        ]
    );
}

#[test]
fn decimal_integer_literal() {
    parser_test!(
        input: "1234",
        expr_output: [
            LiteralExpression {
                span: Span::new(0, 4),
                literal: Literal::Number(
                    Number::Integer(1234, Base::Decimal)
                ),
            }.into()
        ]
    );
}

#[test]
fn decimal_hex_literal() {
    parser_test!(
        input: "0xff",
        expr_output: [
            LiteralExpression {
                span: Span::new(0, 4),
                literal: Literal::Number(
                    Number::Integer(0xff, Base::Hex)
                ),
            }.into()
        ]
    );
}

#[test]
fn decimal_octal_literal() {
    parser_test!(
        input: "0o77",
        expr_output: [
            LiteralExpression {
                span: Span::new(0, 4),
                literal: Literal::Number(
                    Number::Integer(0o77, Base::Octal)
                ),
            }.into()
        ]
    );
}

#[test]
fn decimal_binary_literal() {
    parser_test!(
        input: "0b11110000",
        expr_output: [
            LiteralExpression {
                span: Span::new(0, 10),
                literal: Literal::Number(
                    Number::Integer(0b11110000, Base::Binary)
                ),
            }.into()
        ]
    );
}

#[test]
fn decimal_literal() {
    parser_test!(
        input: "1234.5",
        expr_output: [
            LiteralExpression {
                span: Span::new(0, 6),
                literal: Literal::Number(
                    Number::Decimal(1234.5)
                ),
            }.into()
        ]
    );
}
