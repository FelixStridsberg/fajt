use fajt_lexer::token::Span;
use fajt_parser::ast::*;

#[test]
fn empty_anonymous_function() {
    parser_test!(
        input: "function () {}",
        expr_output: [
            FunctionExpression {
                span: Span::new(0, 14),
                asynchronous: false,
                identifier: None,
            }.into()
        ]
    );
}

#[test]
fn empty_function() {
    parser_test!(
        input: "function fn() {}",
        expr_output: [
            FunctionExpression {
                span: Span::new(0, 16),
                asynchronous: false,
                identifier: Some(Ident::new("fn", (9, 11))),
            }.into()
        ]
    );
}

#[test]
fn empty_anonymous_async_function() {
    parser_test!(
        input: "async function () {}",
        expr_output: [
            FunctionExpression {
                span: Span::new(0, 20),
                asynchronous: true,
                identifier: None,
            }.into()
        ]
    );
}

#[test]
fn empty_async_function() {
    parser_test!(
        input: "async function fn() {}",
        expr_output: [
            FunctionExpression {
                span: Span::new(0, 22),
                asynchronous: true,
                identifier: Some(Ident::new("fn", (15, 17))),
            }.into()
        ]
    );
}
