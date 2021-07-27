use fajt_lexer::token::Span;
use fajt_parser::ast::*;
use fajt_parser::error::ErrorKind::SyntaxError;

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

#[test]
fn optional_member_computed() {
    parser_test!(
        input: "a?.[b]",
        expr_output: [
            OptionalMemberExpression {
                span: Span::new(0, 6),
                object: Ident::new("a", (0, 1)).into(),
                property: MemberProperty::Expression(Ident::new("b", (4, 5)).into()),
                optional: true,
            }.into()
        ]
    );
}

#[test]
fn optional_member_nested() {
    parser_test!(
        input: "a?.b?.c?.d",
        expr_output: [
            OptionalMemberExpression {
                span: Span::new(0, 10),
                object: OptionalMemberExpression {
                    span: Span::new(0, 7),
                    object: OptionalMemberExpression {
                        span: Span::new(0, 4),
                        object: Ident::new("a", (0, 1)).into(),
                        property: MemberProperty::Ident(Ident::new("b", (3, 4))),
                        optional: true,
                    }.into(),
                    property: MemberProperty::Ident(Ident::new("c", (6, 7))),
                    optional: true,
                }.into(),
                property: MemberProperty::Ident(Ident::new("d", (9, 10))),
                optional: true,
            }.into()
        ]
    );
}

#[test]
fn optional_member_mixed() {
    parser_test!(
        input: "a.b?.c[d]",
        expr_output: [
            OptionalMemberExpression {
                span: Span::new(0, 9),
                object: OptionalMemberExpression {
                    span: Span::new(0, 6),
                    object: MemberExpression {
                        span: Span::new(0, 3),
                        object: MemberObject::Expression(Ident::new("a", (0, 1)).into()),
                        property: MemberProperty::Ident(Ident::new("b", (2, 3))),
                    }.into(),
                    property: MemberProperty::Ident(Ident::new("c", (5, 6))),
                    optional: true,
                }.into(),
                property: MemberProperty::Expression(Ident::new("d", (7, 8)).into()),
                optional: false,
            }.into()
        ]
    );
}

#[test]
fn fail_invalid_optional_chain_from_new_expression() {
    // This is not a NewExpression but a MemberExpression.
    parser_test!(
        input: "new a()?.b",
        success
    );

    parser_test!(
        input: "new new a()?.b",
        expr_error: SyntaxError(
            "Invalid optional chain from new expression".to_owned(),
            Span::new(11, 13)
        )
    );
}

#[test]
fn optional_call() {
    parser_test!(
        input: "a?.()",
        expr_output: [
            OptionalCallExpression {
                span: Span::new(0, 5),
                callee: Ident::new("a", (0, 1)).into(),
                arguments_span: Span::new(3, 5),
                arguments: vec![],
                optional: true,
            }.into()
        ]
    );
}

#[test]
fn optional_call_nested() {
    parser_test!(
        input: "a?.()?.()?.()",
        expr_output: [
            OptionalCallExpression {
                span: Span::new(0, 13),
                callee: OptionalCallExpression {
                    span: Span::new(0, 9),
                    callee: OptionalCallExpression {
                        span: Span::new(0, 5),
                        callee: Ident::new("a", (0, 1)).into(),
                        arguments_span: Span::new(3, 5),
                        arguments: vec![],
                        optional: true,
                    }.into(),
                    arguments_span: Span::new(7, 9),
                    arguments: vec![],
                    optional: true,
                }.into(),
                arguments_span: Span::new(11, 13),
                arguments: vec![],
                optional: true,
            }.into()
        ]
    );
}

#[test]
fn optional_call_mixed() {
    parser_test!(
        input: "a?.()?.()()",
        expr_output: [
            OptionalCallExpression {
                span: Span::new(0, 11),
                callee: OptionalCallExpression {
                    span: Span::new(0, 9),
                    callee: OptionalCallExpression {
                        span: Span::new(0, 5),
                        callee: Ident::new("a", (0, 1)).into(),
                        arguments_span: Span::new(3, 5),
                        arguments: vec![],
                        optional: true,
                    }.into(),
                    arguments_span: Span::new(7, 9),
                    arguments: vec![],
                    optional: true,
                }.into(),
                arguments_span: Span::new(9, 11),
                arguments: vec![],
                optional: false,
            }.into()
        ]
    );
}
