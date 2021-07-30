use fajt_lexer::punct;
use fajt_lexer::token::{Span, Token, TokenValue};
use fajt_parser::ast::VariableKind::*;
use fajt_parser::ast::*;
use fajt_parser::error::ErrorKind::{SyntaxError, UnexpectedToken};

#[test]
fn empty() {
    parser_test!(
        input: "var {} = a;",
        output: [
            StmtVariable {
                span: Span::new(0, 11),
                kind: Var,
                declarations: vec![
                    VariableDeclaration {
                        span: Span::new(4, 10),
                        pattern: ObjectBinding {
                            span: Span::new(4, 6),
                            props: vec![],
                            rest: None,
                        }.into(),
                        initializer: Some(Ident::new("a", (9, 10)).into()),
                    }
                ]
            }.into()
        ]
    );
}

#[test]
fn identifier_binding() {
    parser_test!(
        input: "var { a } = b;",
        output: [
            StmtVariable {
                span: Span::new(0, 14),
                kind: Var,
                declarations: vec![
                    VariableDeclaration {
                        span: Span::new(4, 13),
                        pattern: ObjectBinding {
                            span: Span::new(4, 9),
                            props: vec![Ident::new("a", (6, 7)).into()],
                            rest: None,
                        }.into(),
                        initializer: Some(Ident::new("b", (12, 13)).into()),
                    }
                ]
            }.into()
        ]
    );
}

#[test]
fn identifier_binding_with_initializer() {
    parser_test!(
        input: "var { a = b } = c;",
        output: [
            StmtVariable {
                span: Span::new(0, 18),
                kind: Var,
                declarations: vec![
                    VariableDeclaration {
                        span: Span::new(4, 17),
                        pattern: ObjectBinding {
                            span: Span::new(4, 13),
                            props: vec![
                                ObjectBindingProp::Single(
                                    Ident::new("a", (6, 7)),
                                    Some(Ident::new("b", (10, 11)).into()),
                                )
                            ],
                            rest: None,
                        }.into(),
                        initializer: Some(Ident::new("c", (16, 17)).into()),
                    }
                ]
            }.into()
        ]
    );
}

#[test]
fn key_value_binding() {
    parser_test!(
        input: "var { a: b } = c;",
        output: [
            StmtVariable {
                span: Span::new(0, 17),
                kind: Var,
                declarations: vec![
                    VariableDeclaration {
                        span: Span::new(4, 16),
                        pattern: ObjectBinding {
                            span: Span::new(4, 12),
                            props: vec![
                                ObjectBindingProp::KeyValue(
                                    PropertyName::Ident(Ident::new("a", (6, 7)).into()),
                                    BindingElement {
                                        span: Span::new(9, 10),
                                        pattern: BindingPattern::Ident(Ident::new("b", (9, 10)).into()),
                                        initializer: None,
                                    }
                                )
                            ],
                            rest: None,
                        }.into(),
                        initializer: Some(Ident::new("c", (15, 16)).into()),
                    }
                ]
            }.into()
        ]
    );
}

#[test]
fn key_value_binding_with_object_binding() {
    parser_test!(
        input: "var { a: { b } } = c;",
        output: [
            StmtVariable {
                span: Span::new(0, 21),
                kind: Var,
                declarations: vec![
                    VariableDeclaration {
                        span: Span::new(4, 20),
                        pattern: ObjectBinding {
                            span: Span::new(4, 16),
                            props: vec![
                                ObjectBindingProp::KeyValue(
                                    PropertyName::Ident(Ident::new("a", (6, 7)).into()),
                                    BindingElement {
                                        span: Span::new(9, 14),
                                        pattern: ObjectBinding {
                                            span: Span::new(9, 14),
                                            props: vec![
                                                Ident::new("b", (11, 12)).into()
                                            ],
                                            rest: None,
                                        }.into(),
                                        initializer: None,
                                    }
                                )
                            ],
                            rest: None,
                        }.into(),
                        initializer: Some(Ident::new("c", (19, 20)).into()),
                    }
                ]
            }.into()
        ]
    );
}

#[test]
fn trailing_comma() {
    parser_test!(
        input: "var { a, } = b;",
        output: [
            StmtVariable {
                span: Span::new(0, 15),
                kind: Var,
                declarations: vec![
                    VariableDeclaration {
                        span: Span::new(4, 14),
                        pattern: ObjectBinding {
                            span: Span::new(4, 10),
                            props: vec![Ident::new("a", (6, 7)).into()],
                            rest: None,
                        }.into(),
                        initializer: Some(Ident::new("b", (13, 14)).into()),
                    }
                ]
            }.into()
        ]
    );
}

#[test]
fn multiple_identifier_bindings() {
    parser_test!(
        input: "var { a, b } = c;",
        output: [
            StmtVariable {
                span: Span::new(0, 17),
                kind: Var,
                declarations: vec![
                    VariableDeclaration {
                        span: Span::new(4, 16),
                        pattern: ObjectBinding {
                            span: Span::new(4, 12),
                            props: vec![
                                Ident::new("a", (6, 7)).into(),
                                Ident::new("b", (9, 10)).into(),
                            ],
                            rest: None,
                        }.into(),
                        initializer: Some(Ident::new("c", (15, 16)).into()),
                    }
                ],
            }.into()
        ]
    );
}

#[test]
fn rest_binding() {
    parser_test!(
        input: "var { ...rest } = c;",
        output: [
            StmtVariable {
                span: Span::new(0, 20),
                kind: Var,
                declarations: vec![
                    VariableDeclaration {
                       span: Span::new(4, 19),
                        pattern: ObjectBinding {
                            span: Span::new(4, 15),
                            props: vec![],
                            rest: Some(Ident::new("rest", (9, 13))),
                        }.into(),
                        initializer: Some(Ident::new("c", (18, 19)).into()),
                    }
                ],
            }.into()
        ]
    );
}

#[test]
fn await_as_identifier() {
    parser_test!(
        input: "var { await, ...await } = c;",
        output: [
            StmtVariable {
                span: Span::new(0, 28),
                kind: Var,
                declarations: vec![
                    VariableDeclaration {
                        span: Span::new(4, 27),
                        pattern: ObjectBinding {
                            span: Span::new(4, 23),
                            props: vec![
                                Ident::new("await", (6, 11)).into(),
                            ],
                            rest: Some(Ident::new("await", (16, 21))),
                        }.into(),
                        initializer: Some(Ident::new("c", (26, 27)).into()),
                    }
                ],
            }.into()
        ]
    );
}

/// `yield` is a valid identifier in the parser context.
/// See the goal symbol `BindingIdentifier` specification.
#[test]
fn yield_as_identifier() {
    parser_test!(
        input: "var { yield, ...yield } = c;",
        output: [
            StmtVariable {
                span: Span::new(0, 28),
                kind: Var,
                declarations: vec![
                    VariableDeclaration {
                        span: Span::new(4, 27),
                        pattern: ObjectBinding {
                            span: Span::new(4, 23),
                            props: vec![
                                Ident::new("yield", (6, 11)).into(),
                            ],
                            rest: Some(Ident::new("yield", (16, 21))),
                        }.into(),
                        initializer: Some(Ident::new("c", (26, 27)).into()),
                    }
                ],
            }.into()
        ]
    );
}

#[test]
fn fail_prefixing_comma() {
    parser_test!(
        input: "var { , a, b } = c;",
        error: UnexpectedToken(Token::new(punct!(","), false, (6, 7)))
    );
}

#[test]
fn fail_double_comma() {
    parser_test!(
        input: "var { a,, b } = c;",
        error: UnexpectedToken(Token::new(punct!(","), false, (8, 9)))
    );
}

#[test]
fn fail_missing_comma() {
    parser_test!(
        input: "var { a b } = c;",
        error: UnexpectedToken(Token::new(TokenValue::Identifier("b".to_owned()), false, (8, 9)))
    );
}

#[test]
fn fail_rest_binding_must_be_last() {
    parser_test!(
        input: "var { ...rest, b } = c;",
        error: SyntaxError("Rest element must be last element".to_owned(), (9, 13).into())
    );
}