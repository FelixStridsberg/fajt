use fajt_lexer::token::Span;
use fajt_parser::ast::{Ident, ThrowStatement};

#[test]
fn return_void() {
    parser_test!(
        input: "throw",
        output: [
            ThrowStatement {
                span: Span::new(0, 5),
                argument: None,
            }.into()
        ]
    );
}

#[test]
fn return_expression() {
    parser_test!(
        input: "throw a",
        output: [
            ThrowStatement {
                span: Span::new(0, 7),
                argument: Some(Ident::new("a", (6, 7)).into()),
            }.into()
        ]
    );
}
