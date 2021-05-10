mod lib;

use fajt_lexer::punct;
use fajt_lexer::token;
use fajt_lexer::token::{Span, Token};
use fajt_parser::ast::Expr::IdentifierReference;
use fajt_parser::ast::VariableKind::*;
use fajt_parser::ast::*;
use fajt_parser::error::ErrorKind::{SyntaxError, UnexpectedToken};

#[test]
fn parse_var_statement_no_init() {
    parser_test!(
        input: "var foo;",
        output: [
            VariableStmt {
                span: Span::new(0, 8),
                kind: Var,
                declarations: vec![
                    VariableDeclaration {
                        span: Span::new(4, 8),
                        identifier: Ident::new("foo", (4, 7)).into(),
                        initializer: None,
                    },
                ]
            }.into()
        ]
    );
}

#[test]
fn parse_var_statement() {
    parser_test!(
        input: "var foo = a;",
        output: [
            VariableStmt {
                span: Span::new(0, 12),
                kind: Var,
                declarations: vec![
                    VariableDeclaration {
                        span: Span::new(4, 12),
                        identifier: Ident::new("foo", (4, 7)).into(),
                        initializer: Some(IdentifierReference(Ident::new("a", (10, 11)).into()))
                    }
                ]
            }.into()
        ]
    );
}

#[test]
fn fail_var_invalid_identifier() {
    parser_test!(
        input: "var * = c;",
        error: UnexpectedToken(Token::new(punct!("*"), (4, 5)))
    );
}

#[test]
fn parse_var_stmt_empty_obj_binding() {
    parser_test!(
        input: "var {} = a;",
        output: [
            VariableStmt {
                span: Span::new(0, 11),
                kind: Var,
                declarations: vec![
                    VariableDeclaration {
                        span: Span::new(4, 11),
                        identifier: ObjectBinding {
                            span: Span::new(4, 6),
                            props: vec![],
                            rest: None,
                        }.into(),
                        initializer: Some(IdentifierReference(Ident::new("a", (9, 10)).into())),
                    }
                ]
            }.into()
        ]
    );
}

#[test]
fn parse_var_stmt_single_obj_binding() {
    parser_test!(
        input: "var { a } = b;",
        output: [
            VariableStmt {
                span: Span::new(0, 14),
                kind: Var,
                declarations: vec![
                    VariableDeclaration {
                        span: Span::new(4, 14),
                        identifier: ObjectBinding {
                            span: Span::new(4, 9),
                            props: vec![Ident::new("a", (6, 7)).into()],
                            rest: None,
                        }.into(),
                        initializer: Some(IdentifierReference(Ident::new("b", (12, 13)).into())),
                    }
                ]
            }.into()
        ]
    );
}

#[test]
fn parse_var_stmt_multiple_obj_binding() {
    parser_test!(
        input: "var { a, b } = c;",
        output: [
            VariableStmt {
                span: Span::new(0, 17),
                kind: Var,
                declarations: vec![
                    VariableDeclaration {
                        span: Span::new(4, 17),
                        identifier: ObjectBinding {
                            span: Span::new(4, 12),
                            props: vec![
                                Ident::new("a", (6, 7)).into(),
                                Ident::new("b", (9, 10)).into(),
                            ],
                            rest: None,
                        }.into(),
                        initializer: Some(IdentifierReference(Ident::new("c", (15, 16)).into())),
                    }
                ],
            }.into()
        ]
    );
}

#[test]
fn parse_var_stmt_obj_binding_rest() {
    parser_test!(
        input: "var { ...rest } = c;",
        output: [
            VariableStmt {
                span: Span::new(0, 20),
                kind: Var,
                declarations: vec![
                    VariableDeclaration {
                       span: Span::new(4, 20),
                        identifier: ObjectBinding {
                            span: Span::new(4, 15),
                            props: vec![],
                            rest: Some(Ident::new("rest", (9, 13))),
                        }.into(),
                        initializer: Some(IdentifierReference(Ident::new("c", (18, 19)).into())),
                    }
                ],
            }.into()
        ]
    );
}

/// `await` is a valid identifier in the parser context.
/// See the goal symbol `BindingIdentifier` specification.
#[test]
fn parse_var_stmt_obj_binding_await() {
    parser_test!(
        input: "var { await, ...await } = c;",
        output: [
            VariableStmt {
                span: Span::new(0, 28),
                kind: Var,
                declarations: vec![
                    VariableDeclaration {
                        span: Span::new(4, 28),
                        identifier: ObjectBinding {
                            span: Span::new(4, 23),
                            props: vec![
                                Ident::new("await", (6, 11)).into(),
                            ],
                            rest: Some(Ident::new("await", (16, 21))),
                        }.into(),
                        initializer: Some(IdentifierReference(Ident::new("c", (26, 27)).into())),
                    }
                ],
            }.into()
        ]
    );
}

/// `yield` is a valid identifier in the parser context.
/// See the goal symbol `BindingIdentifier` specification.
#[test]
fn parse_var_stmt_obj_binding_yield() {
    parser_test!(
        input: "var { yield, ...yield } = c;",
        output: [
            VariableStmt {
                span: Span::new(0, 28),
                kind: Var,
                declarations: vec![
                    VariableDeclaration {
                        span: Span::new(4, 28),
                        identifier: ObjectBinding {
                            span: Span::new(4, 23),
                            props: vec![
                                Ident::new("yield", (6, 11)).into(),
                            ],
                            rest: Some(Ident::new("yield", (16, 21))),
                        }.into(),
                        initializer: Some(IdentifierReference(Ident::new("c", (26, 27)).into())),
                    }
                ],
            }.into()
        ]
    );
}

#[test]
fn fail_var_statement_prefix_comma() {
    parser_test!(
        input: "var { , a, b } = c;",
        error: UnexpectedToken(Token::new(punct!(","), (6, 7)))
    );
}

#[test]
fn fail_var_statement_double_comma() {
    parser_test!(
        input: "var { a,, b } = c;",
        error: UnexpectedToken(Token::new(punct!(","), (8, 9)))
    );
}

#[test]
fn parse_var_stmt_empty_array_binding() {
    parser_test!(
        input: "var [] = a;",
        output: [
            VariableStmt {
                span: Span::new(0, 11),
                kind: Var,
                declarations: vec![
                    VariableDeclaration {
                        span: Span::new(4, 11),
                        identifier: BindingPattern::Array(
                            ArrayBinding {
                                span: Span::new(4, 6),
                                elements: vec![],
                                rest: None,
                            }
                        ).into(),
                        initializer: Some(IdentifierReference(Ident::new("a", (9, 10)).into())),
                    },
                ],
            }.into()
        ]
    );
}

#[test]
fn parse_var_stmt_single_elem_array_binding() {
    parser_test!(
        input: "var [ a ] = b;",
        output: [
            VariableStmt {
                span: Span::new(0, 14),
                kind: Var,
                declarations: vec![
                    VariableDeclaration {
                        span: Span::new(4, 14),
                        identifier: BindingPattern::Array(
                            ArrayBinding {
                                span: Span::new(4, 9),
                                elements: vec![
                                    Some(BindingPattern::Ident(Ident::new("a", (6, 7)).into()))
                                ],
                                rest: None,
                            }
                        ).into(),
                        initializer: Some(IdentifierReference(Ident::new("b", (12, 13)).into())),
                    },
                ],
            }.into()
        ]
    );
}

#[test]
fn parse_var_stmt_single_elem_array_binding_trailing_comma() {
    parser_test!(
        input: "var [ a, ] = b;",
        output: [
            VariableStmt {
                span: Span::new(0, 15),
                kind: Var,
                declarations: vec![
                    VariableDeclaration {
                        span: Span::new(4, 15),
                        identifier: BindingPattern::Array(
                            ArrayBinding {
                                span: Span::new(4, 10),
                                elements: vec![
                                    Some(BindingPattern::Ident(Ident::new("a", (6, 7)).into()))
                                ],
                                rest: None,
                            }
                        ).into(),
                        initializer: Some(IdentifierReference(Ident::new("b", (13, 14)).into())),
                    },
                ],
            }.into()
        ]
    );
}

#[test]
fn parse_var_stmt_single_elision_array_binding() {
    parser_test!(
        input: "var [ , ] = b;",
        output: [
            VariableStmt {
                span: Span::new(0, 14),
                kind: Var,
                declarations: vec![
                    VariableDeclaration {
                        span: Span::new(4, 14),
                        identifier: BindingPattern::Array(
                            ArrayBinding {
                                span: Span::new(4, 9),
                                elements: vec![ None ],
                                rest: None,
                            }
                        ).into(),
                        initializer: Some(IdentifierReference(Ident::new("b", (12, 13)).into())),
                    },
                ],
            }.into()
        ]
    );
}

#[test]
fn parse_var_stmt_single_rest_array_binding() {
    parser_test!(
        input: "var [ ...a ] = b;",
        output: [
            VariableStmt {
                span: Span::new(0, 17),
                kind: Var,
                declarations: vec![
                    VariableDeclaration {
                        span: Span::new(4, 17),
                        identifier: BindingPattern::Array(
                            ArrayBinding {
                                span: Span::new(4, 12),
                                elements: vec![],
                                rest: Some(Ident::new("a", (9, 10))),
                            }
                        ).into(),
                        initializer: Some(IdentifierReference(Ident::new("b", (15, 16)).into())),
                    },
                ],
            }.into()
        ]
    );
}

#[test]
fn parse_var_stmt_array_binding_mixed() {
    parser_test!(
        input: "var [ , a,,b ] = c;",
        output: [
            VariableStmt {
                span: Span::new(0, 19),
                kind: Var,
                declarations: vec![
                    VariableDeclaration {
                        span: Span::new(4, 19),
                        identifier: BindingPattern::Array(
                            ArrayBinding {
                                span: Span::new(4, 14),
                                elements: vec![
                                    None,
                                    Some(BindingPattern::Ident(Ident::new("a", (8, 9)).into())),
                                    None,
                                    Some(BindingPattern::Ident(Ident::new("b", (11, 12)).into())),
                                ],
                                rest: None,
                            }
                        ).into(),
                        initializer: Some(IdentifierReference(Ident::new("c", (17, 18)).into())),
                    },
                ],
            }.into()
        ]
    );
}

/// `await` is a valid identifier in the parser context.
/// See the goal symbol `BindingIdentifier` specification.
#[test]
fn parse_var_stmt_array_binding_await() {
    parser_test!(
        input: "var [ await, ...await ] = c;",
        output: [
            VariableStmt {
                span: Span::new(0, 28),
                kind: Var,
                declarations: vec![
                    VariableDeclaration {
                        span: Span::new(4, 28),
                        identifier: BindingPattern::Array(
                            ArrayBinding {
                                span: Span::new(4, 23),
                                elements: vec![
                                    Some(BindingPattern::Ident(Ident::new("await", (6, 11)).into())),
                                ],
                                rest: Some(Ident::new("await", (16, 21)).into()),
                            }
                        ).into(),
                        initializer: Some(IdentifierReference(Ident::new("c", (26, 27)).into())),
                    },
                ],
            }.into()
        ]
    );
}

/// `yield` is a valid identifier in the parser context.
/// See the goal symbol `BindingIdentifier` specification.
#[test]
fn parse_var_stmt_array_binding_yield() {
    parser_test!(
        input: "var [ yield, ...yield ] = c;",
        output: [
            VariableStmt {
                span: Span::new(0, 28),
                kind: Var,
                declarations: vec![
                    VariableDeclaration {
                        span: Span::new(4, 28),
                        identifier: BindingPattern::Array(
                            ArrayBinding {
                                span: Span::new(4, 23),
                                elements: vec![
                                    Some(BindingPattern::Ident(Ident::new("yield", (6, 11)).into())),
                                ],
                                rest: Some(Ident::new("yield", (16, 21)).into()),
                            }
                        ).into(),
                        initializer: Some(IdentifierReference(Ident::new("c", (26, 27)).into())),
                    },
                ],
            }.into()
        ]
    );
}

#[test]
fn fail_var_stmt_array_biding_rest_not_last() {
    parser_test!(
        input: "var [ ...rest, b ] = c;",
        error: SyntaxError("Rest element must be last element".to_owned(), (9, 13).into())
    );
}

#[test]
fn parse_var_stmt_array_binding_with_obj_binding() {
    parser_test!(
        input: "var [ { a } ] = b;",
        output: [
            VariableStmt {
                span: Span::new(0, 18),
                kind: Var,
                declarations: vec![
                    VariableDeclaration {
                        span: Span::new(4, 18),
                        identifier: BindingPattern::Array(
                            ArrayBinding {
                                span: Span::new(4, 13),
                                elements: vec![
                                    Some(BindingPattern::Object(
                                        ObjectBinding {
                                            span: Span::new(6, 11),
                                            props: vec![Ident::new("a", (8, 9)).into()],
                                            rest: None,
                                        }
                                    ))
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
