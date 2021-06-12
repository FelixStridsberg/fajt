use fajt_lexer::token::Span;
use fajt_parser::ast::*;

#[test]
fn identifier() {
    parser_test!(
        input: "a.b",
        expr_output: [
            MemberExpression {
                span: Span::new(0, 3),
                object: IdentifierReference::Ident(Ident::new("a", (0, 1))).into(),
                member: Member::Ident(Ident::new("b", (2, 3))),
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
                object: MemberExpression {
                    span: Span::new(0, 3),
                    object: IdentifierReference::Ident(Ident::new("a", (0, 1))).into(),
                    member: Member::Ident(Ident::new("b", (2, 3))),
                }.into(),
                member: Member::Ident(Ident::new("c", (4, 5))),
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
                object: IdentifierReference::Ident(Ident::new("a", (0, 1))).into(),
                member: Member::Expression(
                    IdentifierReference::Ident(Ident::new("b", (2, 3))).into()
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
                object: MemberExpression {
                    span: Span::new(0, 4),
                    object: IdentifierReference::Ident(Ident::new("a", (0, 1))).into(),
                    member: Member::Expression(
                        IdentifierReference::Ident(Ident::new("b", (2, 3))).into()
                    ),
                }.into(),
                member: Member::Expression(
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
                object: MemberExpression {
                    span: Span::new(0, 6),
                    object: MemberExpression {
                        span: Span::new(0, 4),
                        object: IdentifierReference::Ident(Ident::new("a", (0, 1))).into(),
                        member: Member::Expression(
                            IdentifierReference::Ident(Ident::new("b", (2, 3))).into()
                        )
                    }.into(),
                    member: Member::Ident(Ident::new("c", (5, 6))),
                }.into(),
                member: Member::Expression(
                    IdentifierReference::Ident(Ident::new("d", (7, 8))).into()
                ),
            }.into()
        ]
    );
}