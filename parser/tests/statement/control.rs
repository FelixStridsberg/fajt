use fajt_lexer::token::Span;
use fajt_parser::ast::{Ident, ReturnStatement};

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
