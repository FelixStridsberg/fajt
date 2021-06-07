use fajt_lexer::token::Span;
use fajt_parser::ast::*;

#[test]
fn empty_anonymous_function() {
    parser_test!(
        input: "function () {}",
        expr_output: [
            FunctionExpression {
                span: Span::new(0, 14),
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
                identifier: Some(Ident::new("fn", (9, 11))),
            }.into()
        ]
    );
}
