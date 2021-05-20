mod lib;

use fajt_lexer::token::{Span, Token, TokenValue};
use fajt_parser::ast::Expression::IdentifierReference;
use fajt_parser::ast::VariableKind::*;
use fajt_parser::ast::*;
use fajt_parser::error::ErrorKind::{SyntaxError, UnexpectedToken};

#[test]
fn empty() {
    parser_test!(
        input: "var [] = a;",
        output: [
            VariableStatement {
                span: Span::new(0, 11),
                kind: Var,
                declarations: vec![
                    VariableDeclaration {
                        span: Span::new(4, 11),
                        pattern: BindingPattern::Array(
                            ArrayBinding {
                                span: Span::new(4, 6),
                                elements: vec![],
                                rest: None,
                            }
                        ),
                        initializer: Some(IdentifierReference(Ident::new("a", (9, 10)).into())),
                    },
                ],
            }.into()
        ]
    );
}

#[test]
fn identifier_binding() {
    parser_test!(
        input: "var [ a ] = b;",
        output: [
            VariableStatement {
                span: Span::new(0, 14),
                kind: Var,
                declarations: vec![
                    VariableDeclaration {
                        span: Span::new(4, 14),
                        pattern: BindingPattern::Array(
                            ArrayBinding {
                                span: Span::new(4, 9),
                                elements: vec![
                                    Some(BindingElement {
                                        span: Span::new(6, 7),
                                        pattern: BindingPattern::Ident(Ident::new("a", (6, 7))),
                                        initializer: None,
                                    })
                                ],
                                rest: None,
                            }
                        ),
                        initializer: Some(IdentifierReference(Ident::new("b", (12, 13)).into())),
                    },
                ],
            }.into()
        ]
    );
}

#[test]
fn trailing_comma() {
    parser_test!(
        input: "var [ a, ] = b;",
        output: [
            VariableStatement {
                span: Span::new(0, 15),
                kind: Var,
                declarations: vec![
                    VariableDeclaration {
                        span: Span::new(4, 15),
                        pattern: BindingPattern::Array(
                            ArrayBinding {
                                span: Span::new(4, 10),
                                elements: vec![
                                    Some(BindingElement {
                                        span: Span::new(6, 7),
                                        pattern: BindingPattern::Ident(Ident::new("a", (6, 7))),
                                        initializer: None,
                                    })
                                ],
                                rest: None,
                            }
                        ),
                        initializer: Some(IdentifierReference(Ident::new("b", (13, 14)).into())),
                    },
                ],
            }.into()
        ]
    );
}

#[test]
fn single_elision() {
    parser_test!(
        input: "var [ , ] = b;",
        output: [
            VariableStatement {
                span: Span::new(0, 14),
                kind: Var,
                declarations: vec![
                    VariableDeclaration {
                        span: Span::new(4, 14),
                        pattern: BindingPattern::Array(
                            ArrayBinding {
                                span: Span::new(4, 9),
                                elements: vec![ None ],
                                rest: None,
                            }
                        ),
                        initializer: Some(IdentifierReference(Ident::new("b", (12, 13)).into())),
                    },
                ],
            }.into()
        ]
    );
}

#[test]
fn rest_element() {
    parser_test!(
        input: "var [ ...a ] = b;",
        output: [
            VariableStatement {
                span: Span::new(0, 17),
                kind: Var,
                declarations: vec![
                    VariableDeclaration {
                        span: Span::new(4, 17),
                        pattern: BindingPattern::Array(
                            ArrayBinding {
                                span: Span::new(4, 12),
                                elements: vec![],
                                rest: Some(Ident::new("a", (9, 10))),
                            }
                        ),
                        initializer: Some(IdentifierReference(Ident::new("b", (15, 16)).into())),
                    },
                ],
            }.into()
        ]
    );
}

#[test]
fn mixed_elision_and_identifiers() {
    parser_test!(
        input: "var [ , a,,b ] = c;",
        output: [
            VariableStatement {
                span: Span::new(0, 19),
                kind: Var,
                declarations: vec![
                    VariableDeclaration {
                        span: Span::new(4, 19),
                        pattern: BindingPattern::Array(
                            ArrayBinding {
                                span: Span::new(4, 14),
                                elements: vec![
                                    None,
                                    Some(BindingElement {
                                        span: Span::new(8, 9),
                                        pattern: BindingPattern::Ident(Ident::new("a", (8, 9))),
                                        initializer: None,
                                    }),
                                    None,
                                    Some(
                                        BindingElement {
                                            span: Span::new(11, 12),
                                            pattern: BindingPattern::Ident(Ident::new("b", (11, 12))),
                                            initializer: None,
                                        }
                                    ),
                                ],
                                rest: None,
                            }
                        ),
                        initializer: Some(IdentifierReference(Ident::new("c", (17, 18)).into())),
                    },
                ],
            }.into()
        ]
    );
}

#[test]
fn await_as_identifier() {
    parser_test!(
        input: "var [ await, ...await ] = c;",
        output: [
            VariableStatement {
                span: Span::new(0, 28),
                kind: Var,
                declarations: vec![
                    VariableDeclaration {
                        span: Span::new(4, 28),
                        pattern: BindingPattern::Array(
                            ArrayBinding {
                                span: Span::new(4, 23),
                                elements: vec![
                                    Some(
                                        BindingElement {
                                            span: Span::new(6, 11),
                                            pattern: BindingPattern::Ident(Ident::new("await", (6, 11))),
                                            initializer: None,
                                        }
                                    ),
                                ],
                                rest: Some(Ident::new("await", (16, 21))),
                            }
                        ),
                        initializer: Some(IdentifierReference(Ident::new("c", (26, 27)).into())),
                    },
                ],
            }.into()
        ]
    );
}

// `yield` is a valid identifier in the parser context.
// See the goal symbol `BindingIdentifier` specification.
#[test]
fn yield_as_identifier() {
    parser_test!(
        input: "var [ yield, ...yield ] = c;",
        output: [
            VariableStatement {
                span: Span::new(0, 28),
                kind: Var,
                declarations: vec![
                    VariableDeclaration {
                        span: Span::new(4, 28),
                        pattern: BindingPattern::Array(
                            ArrayBinding {
                                span: Span::new(4, 23),
                                elements: vec![
                                    Some(
                                        BindingElement {
                                            span: Span::new(6, 11),
                                            pattern: BindingPattern::Ident(Ident::new("yield", (6, 11))),
                                            initializer: None,
                                        }
                                    ),
                                ],
                                rest: Some(Ident::new("yield", (16, 21))),
                            }
                        ),
                        initializer: Some(IdentifierReference(Ident::new("c", (26, 27)).into())),
                    },
                ],
            }.into()
        ]
    );
}

#[test]
fn fail_rest_element_must_be_last() {
    parser_test!(
        input: "var [ ...rest, b ] = c;",
        error: SyntaxError("Rest element must be last element".to_owned(), (9, 13).into())
    );
}

#[test]
fn fail_missing_delimiter() {
    parser_test!(
        input: "var [ a b ] = c;",
        error: UnexpectedToken(Token::new(TokenValue::Identifier("b".to_owned()), false, (8, 9)))
    );
}

#[test]
fn nested_object_binding() {
    parser_test!(
        input: "var [ { a } ] = b;",
        output: [
            VariableStatement {
                span: Span::new(0, 18),
                kind: Var,
                declarations: vec![
                    VariableDeclaration {
                        span: Span::new(4, 18),
                        pattern: BindingPattern::Array(
                            ArrayBinding {
                                span: Span::new(4, 13),
                                elements: vec![
                                    Some(
                                        BindingElement {
                                            span: Span::new(6, 11),
                                            pattern: BindingPattern::Object(
                                                ObjectBinding {
                                                    span: Span::new(6, 11),
                                                    props: vec![Ident::new("a", (8, 9)).into()],
                                                    rest: None,
                                                }
                                            ),
                                            initializer: None,
                                        }
                                    )
                                ],
                                rest: None,
                            }
                        ),
                        initializer: Some(IdentifierReference(Ident::new("b", (16, 17)).into())),
                    },
                ],
            }.into()
        ]
    );
}
