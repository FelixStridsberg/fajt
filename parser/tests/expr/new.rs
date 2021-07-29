use fajt_lexer::token::Span;
use fajt_parser::ast::*;

#[test]
fn new() {
    parser_test!(
        input: "new a",
        expr_output: [
            ExprNew {
                span: Span::new(0, 5),
                callee: ExprIdentifier::Ident(Ident::new("a", (4, 5))).into(),
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
            ExprNew {
                span: Span::new(0, 9),
                callee: ExprNew {
                    span: Span::new(4, 9),
                    callee: ExprIdentifier::Ident(Ident::new("a", (8, 9))).into(),
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
            ExprNew {
                span: Span::new(0, 7),
                callee: ExprIdentifier::Ident(Ident::new("a", (4, 5))).into(),
                arguments_span: Some(Span::new(5, 7)),
                arguments: vec![],
            }.into()
        ]
    );
}

#[test]
fn new_empty_arguments_member() {
    parser_test!(
        input: "new a.b()",
        expr_output: [
            ExprNew {
                span: Span::new(0, 9),
                callee: ExprMember {
                    span: Span::new(4, 7),
                    object: MemberObject::Expr(
                        ExprIdentifier::Ident(Ident::new("a", (4, 5))).into()
                    ),
                    property: MemberProperty::Ident(Ident::new("b", (6, 7))),
                }.into(),
                arguments_span: Some(Span::new(7, 9)),
                arguments: vec![],
            }.into()
        ]
    );
}

#[test]
fn new_with_arguments() {
    parser_test!(
        input: "new a(b, !null)",
        expr_output: [
            ExprNew {
                span: Span::new(0, 15),
                callee: ExprIdentifier::Ident(Ident::new("a", (4, 5))).into(),
                arguments_span: Some(Span::new(5, 15)),
                arguments: vec![
                    Argument::Expr(
                        ExprIdentifier::Ident(Ident::new("b", (6, 7))).into()
                    ),
                    Argument::Expr(
                        ExprUnary {
                            span: Span::new(9, 14),
                            operator: UnaryOperator::Not,
                            argument: ExprLiteral {
                                span: Span::new(10, 14),
                                literal: Literal::Null,
                            }.into(),
                        }.into()
                    ),
                ],
            }.into()
        ]
    );
}

#[test]
fn new_with_spread_arguments() {
    parser_test!(
        input: "new a(...b, c, ...[])",
        expr_output: [
            ExprNew {
                span: Span::new(0, 21),
                callee: ExprIdentifier::Ident(Ident::new("a", (4, 5))).into(),
                arguments_span: Some(Span::new(5, 21)),
                arguments: vec![
                    Argument::Spread(ExprIdentifier::Ident(Ident::new("b", (9, 10))).into()),
                    Argument::Expr(ExprIdentifier::Ident(Ident::new("c", (12, 13))).into()),
                    Argument::Spread(
                        ExprLiteral {
                            span: Span::new(18, 20),
                            literal: Literal::Array(Array {
                                elements: vec![]
                            })
                        }.into()
                    ),
                ],
            }.into()
        ]
    );
}
