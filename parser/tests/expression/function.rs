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
                generator: false,
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
                generator: false,
                identifier: Some(Ident::new("fn", (9, 11))),
            }.into()
        ]
    );
}

#[test]
fn empty_anonymous_generator() {
    parser_test!(
        input: "function *() {}",
        expr_output: [
            FunctionExpression {
                span: Span::new(0, 15),
                asynchronous: false,
                generator: true,
                identifier: None,
            }.into()
        ]
    );
}

#[test]
fn empty_generator() {
    parser_test!(
        input: "function *fn() {}",
        expr_output: [
            FunctionExpression {
                span: Span::new(0, 17),
                asynchronous: false,
                generator: true,
                identifier: Some(Ident::new("fn", (10, 12))),
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
                generator: false,
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
                generator: false,
                identifier: Some(Ident::new("fn", (15, 17))),
            }.into()
        ]
    );
}

#[test]
fn empty_anonymous_async_generator() {
    parser_test!(
        input: "async function *() {}",
        expr_output: [
            FunctionExpression {
                span: Span::new(0, 21),
                asynchronous: true,
                generator: true,
                identifier: None,
            }.into()
        ]
    );
}

#[test]
fn empty_async_generator() {
    parser_test!(
        input: "async function *fn() {}",
        expr_output: [
            FunctionExpression {
                span: Span::new(0, 23),
                asynchronous: true,
                generator: true,
                identifier: Some(Ident::new("fn", (16, 18))),
            }.into()
        ]
    );
}
