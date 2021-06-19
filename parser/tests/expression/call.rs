use fajt_lexer::token::Span;
use fajt_parser::ast::*;

#[test]
fn super_call() {
    parser_test!(
        input: "super()",
        expr_output: [
            CallExpression {
                span: Span::new(0, 7),
                callee: Callee::Super,
                arguments_span: Span::new(5, 7),
                arguments: vec![]
            }.into()
        ]
    );
}

#[test]
fn super_call_with_args() {
    parser_test!(
        input: "super(a, b)",
        expr_output: [
            CallExpression {
                span: Span::new(0, 11),
                callee: Callee::Super,
                arguments_span: Span::new(5, 11),
                arguments: vec![
                    Argument::Expression(Ident::new("a", (6, 7)).into()),
                    Argument::Expression(Ident::new("b", (9, 10)).into()),
                ]
            }.into()
        ]
    );
}

#[test]
fn import_call() {
    parser_test!(
        input: "import(a)",
        expr_output: [
            CallExpression {
                span: Span::new(0, 9),
                callee: Callee::Import,
                arguments_span: Span::new(6, 9),
                arguments: vec![
                    Argument::Expression(Ident::new("a", (7, 8)).into()),
                ]
            }.into()
        ]
    );
}

#[test]
fn empty_call_expression() {
    parser_test!(
        input: "fn()",
        expr_output: [
            CallExpression {
                span: Span::new(0, 4),
                callee: Callee::Expression(Ident::new("fn", (0, 2)).into()),
                arguments_span: Span::new(2, 4),
                arguments: vec![]
            }.into()
        ]
    );
}

#[test]
fn call_expression() {
    parser_test!(
        input: "fn(a, ...b)",
        expr_output: [
            CallExpression {
                span: Span::new(0, 11),
                callee: Callee::Expression(Ident::new("fn", (0, 2)).into()),
                arguments_span: Span::new(2, 11),
                arguments: vec![
                    Argument::Expression(Ident::new("a", (3, 4)).into()),
                    Argument::Spread(Ident::new("b", (9, 10)).into()),
                ]
            }.into()
        ]
    );
}

#[test]
fn empty_call_member_identifier() {
    parser_test!(
        input: "fn().a",
        expr_output: [
            MemberExpression {
                span: Span::new(0, 6),
                object: MemberObject::Expression(
                    CallExpression {
                        span: Span::new(0, 4),
                        callee: Callee::Expression(Ident::new("fn", (0, 2)).into()),
                        arguments_span: Span::new(2, 4),
                        arguments: vec![]
                    }.into()
                ),
                property: MemberProperty::Ident(Ident::new("a", (5, 6))),
            }.into()
        ]
    );
}

#[test]
fn nested_call_member() {
    parser_test!(
        input: "f1().f2()",
        expr_output: [
            CallExpression {
                span: Span::new(0, 9),
                callee: Callee::Expression(
                    MemberExpression {
                        span: Span::new(0, 7),
                        object: MemberObject::Expression(
                            CallExpression {
                                span: Span::new(0, 4),
                                callee: Callee::Expression(Ident::new("f1", (0, 2)).into()),
                                arguments_span: Span::new(2, 4),
                                arguments: vec![],
                            }.into()
                        ),
                        property: MemberProperty::Ident(Ident::new("f2", (5, 7))),
                    }.into()
                ),
                arguments_span: Span::new(7, 9),
                arguments: vec![]
            }.into()
        ]
    );
}

#[test]
fn nested_call_member_computed() {
    parser_test!(
        input: r#"f1()["f2"]()"#,
        expr_output: [
            CallExpression {
                span: Span::new(0, 12),
                callee: Callee::Expression(
                    MemberExpression {
                        span: Span::new(0, 10),
                        object: MemberObject::Expression(
                            CallExpression {
                                span: Span::new(0, 4),
                                callee: Callee::Expression(Ident::new("f1", (0, 2)).into()),
                                arguments_span: Span::new(2, 4),
                                arguments: vec![],
                            }.into()
                        ),
                        property: MemberProperty::Expression(LiteralExpression {
                            span: Span::new(5, 9),
                            literal: Literal::String("f2".to_owned(), '"')
                        }.into()),
                    }.into()
                ),
                arguments_span: Span::new(10, 12),
                arguments: vec![]
            }.into()
        ]
    );
}
