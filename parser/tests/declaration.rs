mod lib;

use fajt_lexer::token::Span;
use fajt_parser::ast::*;

#[test]
fn function_declaration() {
    parser_test!(
        input: "function fn() {}",
        output: [
            FunctionDeclaration {
                span: Span::new(0, 16),
                asynchronous: false,
                ident: Ident::new("fn", (9, 11)),
                parameters: vec![],
                body: vec![],
            }.into()
        ]
    );
}

#[test]
fn async_function_declaration() {
    parser_test!(
        input: "async function fn() {}",
        output: [
            FunctionDeclaration {
                span: Span::new(0, 22),
                asynchronous: true,
                ident: Ident::new("fn", (15, 17)),
                parameters: vec![],
                body: vec![],
            }.into()
        ]
    );
}
