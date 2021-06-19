use fajt_lexer::token::Span;
use fajt_parser::ast::*;

#[test]
fn identifier() {
    parser_test!(
        input: "a.b",
        expr_output: [
            MemberExpression {
                span: Span::new(0, 3),
                object: MemberObject::Expression(
                    IdentifierReference::Ident(Ident::new("a", (0, 1))).into()
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
            MemberExpression {
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
            MemberExpression {
                span: Span::new(0, 5),
                object: MemberObject::Expression(
                    MemberExpression {
                        span: Span::new(0, 3),
                        object: MemberObject::Expression(
                            IdentifierReference::Ident(Ident::new("a", (0, 1))).into()
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
            MemberExpression {
                span: Span::new(0, 4),
                object: MemberObject::Expression(
                    IdentifierReference::Ident(Ident::new("a", (0, 1))).into()
                ),
                property: MemberProperty::Expression(
                    IdentifierReference::Ident(Ident::new("b", (2, 3))).into()
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
            MemberExpression {
                span: Span::new(0, 8),
                object: MemberObject::Super(Super {
                    span: Span::new(0, 5)
                }),
                property: MemberProperty::Expression(
                    IdentifierReference::Ident(Ident::new("b", (6, 7))).into()
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
            MemberExpression {
                span: Span::new(0, 7),
                object: MemberObject::Expression(
                    MemberExpression {
                        span: Span::new(0, 4),
                        object: MemberObject::Expression(
                            IdentifierReference::Ident(Ident::new("a", (0, 1))).into()
                        ),
                        property: MemberProperty::Expression(
                            IdentifierReference::Ident(Ident::new("b", (2, 3))).into()
                        ),
                    }.into(),
                ),
                property: MemberProperty::Expression(
                    IdentifierReference::Ident(Ident::new("c", (5, 6))).into()
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
            MemberExpression {
                span: Span::new(0, 9),
                object: MemberObject::Expression(
                    MemberExpression {
                        span: Span::new(0, 6),
                        object: MemberObject::Expression(
                            MemberExpression {
                                span: Span::new(0, 4),
                                object: MemberObject::Expression(
                                    IdentifierReference::Ident(Ident::new("a", (0, 1))).into()
                                ),
                                property: MemberProperty::Expression(
                                    IdentifierReference::Ident(Ident::new("b", (2, 3))).into()
                                )
                            }.into(),
                        ),
                        property: MemberProperty::Ident(Ident::new("c", (5, 6))),
                    }.into(),
                ),
                property: MemberProperty::Expression(
                    IdentifierReference::Ident(Ident::new("d", (7, 8))).into()
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
            MetaPropertyExpression {
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
            MetaPropertyExpression {
                span: Span::new(0, 11),
                meta: Ident::new("import", (0, 6)),
                property: Ident::new("meta", (7, 11)),
            }.into()
        ]
    );
}
