use fajt_lexer::token::Span;
use fajt_parser::ast::*;

#[test]
fn sequence_expression() {
    parser_test!(
        input: "a, b, c",
        expr_output: [
            SequenceExpression {
                span: Span::new(0, 7),
                expressions: vec![
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
            ParenthesizedExpression {
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
            ParenthesizedExpression {
                span: Span::new(0, 6),
                expression: SequenceExpression {
                    span: Span::new(1, 5),
                    expressions: vec![
                        Ident::new("a", (1, 2)).into(),
                        Ident::new("b", (4, 5)).into(),
                    ]
                }.into()
            }.into()
        ]
    );
}

#[test]
fn optional_member() {
    parser_test!(
        input: "a?.b",
        expr_output: [
            OptionalMemberExpression {
                span: Span::new(0, 4),
                object: Ident::new("a", (0, 1)).into(),
                property: MemberProperty::Ident(Ident::new("b", (3, 4))),
                optional: true,
            }.into()
        ]
    );
}
