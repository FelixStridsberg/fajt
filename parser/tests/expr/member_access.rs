use fajt_lexer::token::Span;
use fajt_parser::ast::*;
use fajt_parser::error::ErrorKind::SyntaxError;

#[test]
fn identifier() {
    parser_test!(
        input: "a.b",
        expr_output: [
            ExprMember {
                span: Span::new(0, 3),
                object: MemberObject::Expr(
                    Ident::new("a", (0, 1)).into()
                ),
                property: MemberProperty::Ident(Ident::new("b", (2, 3))),
            }.into()
        ]
    );
}

#[test]
fn super_identifier() {
    parser_test!(
        input: "super.b",
        expr_output: [
            ExprMember {
                span: Span::new(0, 7),
                object: MemberObject::Super(
                    Super {
                        span: Span::new(0, 5)
                    }
                ),
                property: MemberProperty::Ident(Ident::new("b", (6, 7))),
            }.into()
        ]
    );
}

#[test]
fn identifier_nested() {
    parser_test!(
        input: "a.b.c",
        expr_output: [
            ExprMember {
                span: Span::new(0, 5),
                object: MemberObject::Expr(
                    ExprMember {
                        span: Span::new(0, 3),
                        object: MemberObject::Expr(
                            Ident::new("a", (0, 1)).into()
                        ),
                        property: MemberProperty::Ident(Ident::new("b", (2, 3))),
                    }.into(),
                ),
                property: MemberProperty::Ident(Ident::new("c", (4, 5))),
            }.into()
        ]
    );
}

#[test]
fn computed() {
    parser_test!(
        input: "a[b]",
        expr_output: [
            ExprMember {
                span: Span::new(0, 4),
                object: MemberObject::Expr(
                    Ident::new("a", (0, 1)).into()
                ),
                property: MemberProperty::Expr(
                    Ident::new("b", (2, 3)).into()
                ),
            }.into()
        ]
    );
}

#[test]
fn super_computed() {
    parser_test!(
        input: "super[b]",
        expr_output: [
            ExprMember {
                span: Span::new(0, 8),
                object: MemberObject::Super(Super {
                    span: Span::new(0, 5)
                }),
                property: MemberProperty::Expr(
                    Ident::new("b", (6, 7)).into()
                ),
            }.into()
        ]
    );
}

#[test]
fn computed_nested() {
    parser_test!(
        input: "a[b][c]",
        expr_output: [
            ExprMember {
                span: Span::new(0, 7),
                object: MemberObject::Expr(
                    ExprMember {
                        span: Span::new(0, 4),
                        object: MemberObject::Expr(
                            Ident::new("a", (0, 1)).into()
                        ),
                        property: MemberProperty::Expr(
                            Ident::new("b", (2, 3)).into()
                        ),
                    }.into(),
                ),
                property: MemberProperty::Expr(
                    Ident::new("c", (5, 6)).into()
                ),
            }.into()
        ]
    );
}

#[test]
fn nested_mixed() {
    parser_test!(
        input: "a[b].c[d]",
        expr_output: [
            ExprMember {
                span: Span::new(0, 9),
                object: MemberObject::Expr(
                    ExprMember {
                        span: Span::new(0, 6),
                        object: MemberObject::Expr(
                            ExprMember {
                                span: Span::new(0, 4),
                                object: MemberObject::Expr(
                                    Ident::new("a", (0, 1)).into()
                                ),
                                property: MemberProperty::Expr(
                                    Ident::new("b", (2, 3)).into()
                                )
                            }.into(),
                        ),
                        property: MemberProperty::Ident(Ident::new("c", (5, 6))),
                    }.into(),
                ),
                property: MemberProperty::Expr(
                    Ident::new("d", (7, 8)).into()
                ),
            }.into()
        ]
    );
}

#[test]
fn new_target_meta_property() {
    parser_test!(
        input: "new.target",
        expr_output: [
            ExprMetaProperty {
                span: Span::new(0, 10),
                meta: Ident::new("new", (0, 3)),
                property: Ident::new("target", (4, 10)),
            }.into()
        ]
    );
}

#[test]
fn import_meta_property() {
    parser_test!(
        input: "import.meta",
        expr_output: [
            ExprMetaProperty {
                span: Span::new(0, 11),
                meta: Ident::new("import", (0, 6)),
                property: Ident::new("meta", (7, 11)),
            }.into()
        ]
    );
}

#[test]
fn optional_member() {
    parser_test!(
        input: "a?.b",
        expr_output: [
            ExprOptionalMember {
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
            ExprOptionalMember {
                span: Span::new(0, 6),
                object: Ident::new("a", (0, 1)).into(),
                property: MemberProperty::Expr(Ident::new("b", (4, 5)).into()),
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
            ExprOptionalMember {
                span: Span::new(0, 10),
                object: ExprOptionalMember {
                    span: Span::new(0, 7),
                    object: ExprOptionalMember {
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
            ExprOptionalMember {
                span: Span::new(0, 9),
                object: ExprOptionalMember {
                    span: Span::new(0, 6),
                    object: ExprMember {
                        span: Span::new(0, 3),
                        object: MemberObject::Expr(Ident::new("a", (0, 1)).into()),
                        property: MemberProperty::Ident(Ident::new("b", (2, 3))),
                    }.into(),
                    property: MemberProperty::Ident(Ident::new("c", (5, 6))),
                    optional: true,
                }.into(),
                property: MemberProperty::Expr(Ident::new("d", (7, 8)).into()),
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
            ExprOptionalCall {
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
            ExprOptionalCall {
                span: Span::new(0, 13),
                callee: ExprOptionalCall {
                    span: Span::new(0, 9),
                    callee: ExprOptionalCall {
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
            ExprOptionalCall {
                span: Span::new(0, 11),
                callee: ExprOptionalCall {
                    span: Span::new(0, 9),
                    callee: ExprOptionalCall {
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

#[test]
fn optional_chain_mixed_call_and_member() {
    parser_test!(
        input: "a.b?.c()?.d?.()",
        expr_output: [
            ExprOptionalCall {
                span: Span::new(0, 15),
                callee: ExprOptionalMember {
                    span: Span::new(0, 11),
                    object: ExprOptionalCall {
                        span: Span::new(0, 8),
                        callee: ExprOptionalMember {
                            span: Span::new(0, 6),
                            object: ExprMember {
                                span: Span::new(0, 3),
                                object: MemberObject::Expr(Ident::new("a", (0, 1)).into()),
                                property: MemberProperty::Ident(Ident::new("b", (2, 3))),
                            }.into(),
                            property: MemberProperty::Ident(Ident::new("c", (5, 6))),
                            optional: true,
                        }.into(),
                        arguments_span: Span::new(6, 8),
                        arguments: vec![],
                        optional: false,
                    }.into(),
                    property: MemberProperty::Ident(Ident::new("d", (10, 11))),
                    optional: true,
                }.into(),
                arguments_span: Span::new(13, 15),
                arguments: vec![],
                optional: true,
            }.into()
        ]
    );
}
