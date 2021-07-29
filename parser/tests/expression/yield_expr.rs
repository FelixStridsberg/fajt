use fajt_lexer::token::Span;
use fajt_parser::ast::*;

#[test]
fn empty_yield() {
    parser_test!(
        input: "yield",
        expr_output: [
            ExprYield {
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
            ExprYield {
                span: Span::new(0, 7),
                argument: Some(ExprIdentifier::Ident(Ident::new("a", (6, 7))).into()),
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
            ExprYield {
                span: Span::new(0, 8),
                argument: Some(ExprIdentifier::Ident(Ident::new("a", (7, 8))).into()),
                delegate: true,
            }.into()
        ]
    );
}

#[test]
fn yield_new_line() {
    parser_test!(
        input: "yield\na",
        expr_output: [
            ExprYield {
                span: Span::new(0, 5),
                argument: None,
                delegate: false,
            }.into()
        ]
    );
}

#[test]
fn yield_new_line_delegate() {
    parser_test!(
        input: "yield\n* a",
        expr_output: [
            ExprYield {
                span: Span::new(0, 5),
                argument: None,
                delegate: false,
            }.into()
        ]
    );
}
