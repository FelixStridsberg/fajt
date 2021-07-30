use fajt_lexer::token::Span;
use fajt_parser::ast::*;

#[test]
fn sequence_expression() {
    parser_test!(
        input: "a, b, c",
        expr_output: [
            ExprSequence {
                span: Span::new(0, 7),
                expr: vec![
                    Ident::new("a", (0, 1)).into(),
                    Ident::new("b", (3, 4)).into(),
                    Ident::new("c", (6, 7)).into(),
                ],
            }.into()
        ]
    );
}

#[test]
fn parenthesized_expression() {
    parser_test!(
        input: "(a)",
        expr_output: [
            ExprParenthesized {
                span: Span::new(0, 3),
                expression: Ident::new("a", (1, 2)).into(),
            }.into()
        ]
    );
}

#[test]
fn parenthesized_expression_sequence() {
    parser_test!(
        input: "(a, b)",
        expr_output: [
            ExprParenthesized {
                span: Span::new(0, 6),
                expression: ExprSequence {
                    span: Span::new(1, 5),
                    expr: vec![
                        Ident::new("a", (1, 2)).into(),
                        Ident::new("b", (4, 5)).into(),
                    ]
                }.into()
            }.into()
        ]
    );
}