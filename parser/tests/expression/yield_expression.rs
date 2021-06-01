use fajt_lexer::token::Span;
use fajt_parser::ast::*;

#[test]
fn empty_yield() {
    parser_test!(
        input: "yield",
        expr_output: [
            YieldExpression {
                span: Span::new(0, 5),
                argument: None,
                delegate: false,
            }.into()
        ]
    );
}

#[test]
fn yield_a() {
    parser_test!(
        input: "yield a",
        expr_output: [
            YieldExpression {
                span: Span::new(0, 7),
                argument: Some(IdentifierReference::Ident(Ident::new("a", (6, 7))).into()),
                delegate: false,
            }.into()
        ]
    );
}

#[test]
fn delegated_yield() {
    parser_test!(
        input: "yield* a",
        expr_output: [
            YieldExpression {
                span: Span::new(0, 8),
                argument: Some(IdentifierReference::Ident(Ident::new("a", (7, 8))).into()),
                delegate: true,
            }.into()
        ]
    );
}
