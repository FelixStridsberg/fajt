use fajt_lexer::token::Span;
use fajt_parser::ast::*;

#[test]
fn new() {
    parser_test!(
        input: "new a",
        expr_output: [
            NewExpression {
                span: Span::new(0, 5),
                callee: IdentifierReference::Ident(Ident::new("a", (4, 5))).into(),
                arguments_span: None,
                arguments: vec![],
            }.into()
        ]
    );
}

#[test]
fn new_nested() {
    parser_test!(
        input: "new new a",
        expr_output: [
            NewExpression {
                span: Span::new(0, 9),
                callee: NewExpression {
                    span: Span::new(4, 9),
                    callee: IdentifierReference::Ident(Ident::new("a", (8, 9))).into(),
                    arguments_span: None,
                    arguments: vec![],
                }.into(),
                arguments_span: None,
                arguments: vec![],
            }.into()
        ]
    );
}

#[test]
fn new_empty_arguments() {
    parser_test!(
        input: "new a()",
        expr_output: [
            NewExpression {
                span: Span::new(0, 7),
                callee: IdentifierReference::Ident(Ident::new("a", (4, 5))).into(),
                arguments_span: Some(Span::new(5, 7)),
                arguments: vec![],
            }.into()
        ]
    );
}
