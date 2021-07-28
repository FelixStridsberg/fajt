use fajt_lexer::token::Span;
use fajt_parser::ast::{Ident, ReturnStatement, BreakStatement};

#[test]
fn return_void() {
    parser_test!(
        input: "return",
        output: [
            ReturnStatement {
                span: Span::new(0, 6),
                argument: None,
            }.into()
        ]
    );
}

#[test]
fn return_expression() {
    parser_test!(
        input: "return a",
        output: [
            ReturnStatement {
                span: Span::new(0, 8),
                argument: Some(Ident::new("a", (7, 8)).into()),
            }.into()
        ]
    );
}

#[test]
fn break_() {
    parser_test!(
        input: "break",
        output: [
            BreakStatement {
                span: Span::new(0, 5),
                label: None,
            }.into()
        ]
    );
}

#[test]
fn break_labelled() {
    parser_test!(
        input: "break a",
        output: [
            BreakStatement {
                span: Span::new(0, 7),
                label: Some(Ident::new("a", (6, 7)).into()),
            }.into()
        ]
    );
}
