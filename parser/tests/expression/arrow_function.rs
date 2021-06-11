use fajt_lexer::token::Span;
use fajt_parser::ast::*;

#[test]
fn identifier_argument() {
    parser_test!(
        input: "a => {}",
        expr_output: [
            ArrowFunctionExpression {
                span: Span::new(0, 7),
                binding_parameter: true,
                parameters: Some(FormalParameters {
                    span: Span::new(0, 1),
                    parameters: vec![
                        BindingElement {
                            span: Span::new(0, 1),
                            pattern: Ident::new("a", (0, 1)).into(),
                            initializer: None,
                        }
                    ],
                    rest: None,
                }),
                body: ArrowFunctionBody::Block(vec![])
            }.into()
        ]
    );
}

#[test]
fn async_identifier_argument() {
    parser_test!(
        input: "async a => {}",
        expr_output: [
            ArrowFunctionExpression {
                span: Span::new(0, 13),
                binding_parameter: true,
                parameters: Some(FormalParameters {
                    span: Span::new(6, 7),
                    parameters: vec![
                        BindingElement {
                            span: Span::new(6, 7),
                            pattern: Ident::new("a", (6, 7)).into(),
                            initializer: None,
                        }
                    ],
                    rest: None,
                }),
                body: ArrowFunctionBody::Block(vec![])
            }.into()
        ]
    );
}

#[test]
fn identifier_argument_expression_body() {
    parser_test!(
        input: "a => b",
        expr_output: [
            ArrowFunctionExpression {
                span: Span::new(0, 6),
                binding_parameter: true,
                parameters: Some(FormalParameters {
                    span: Span::new(0, 1),
                    parameters: vec![
                        BindingElement {
                            span: Span::new(0, 1),
                            pattern: Ident::new("a", (0, 1)).into(),
                            initializer: None,
                        }
                    ],
                    rest: None,
                }),
                body: ArrowFunctionBody::Expression(
                    IdentifierReference::Ident(Ident::new("b", (5, 6))).into()
                )
            }.into()
        ]
    );
}

#[test]
fn async_identifier_argument_expression_body() {
    parser_test!(
        input: "async a => b",
        expr_output: [
            ArrowFunctionExpression {
                span: Span::new(0, 12),
                binding_parameter: true,
                parameters: Some(FormalParameters {
                    span: Span::new(6, 7),
                    parameters: vec![
                        BindingElement {
                            span: Span::new(6, 7),
                            pattern: Ident::new("a", (6, 7)).into(),
                            initializer: None,
                        }
                    ],
                    rest: None,
                }),
                body: ArrowFunctionBody::Expression(
                    IdentifierReference::Ident(Ident::new("b", (11, 12))).into()
                )
            }.into()
        ]
    );
}

#[test]
fn no_parameters() {
    parser_test!(
        input: "() => {}",
        expr_output: [
            ArrowFunctionExpression {
                span: Span::new(0, 8),
                binding_parameter: false,
                parameters: None,
                body: ArrowFunctionBody::Block(vec![])
            }.into()
        ]
    );
}

#[test]
fn async_no_parameters() {
    parser_test!(
        input: "async () => {}",
        expr_output: [
            ArrowFunctionExpression {
                span: Span::new(0, 14),
                binding_parameter: false,
                parameters: None,
                body: ArrowFunctionBody::Block(vec![])
            }.into()
        ]
    );
}

#[test]
fn parameters_and_body() {
    parser_test!(
        input: "(a) => { ; }",
        expr_output: [
            ArrowFunctionExpression {
                span: Span::new(0, 12),
                binding_parameter: false,
                parameters: Some(FormalParameters {
                    span: Span::new(0, 3),
                    parameters: vec![
                        BindingElement {
                            span: Span::new(1, 2),
                            pattern: Ident::new("a", (1, 2)).into(),
                            initializer: None,
                        }
                    ],
                    rest: None,
                }),
                body: ArrowFunctionBody::Block(vec![
                    EmptyStatement {
                        span: Span::new(9, 10),
                    }.into()
                ])
            }.into()
        ]
    );
}

#[test]
fn multiple_parameters() {
    parser_test!(
        input: "(a, b, ...rest) => { ; }",
        expr_output: [
            ArrowFunctionExpression {
                span: Span::new(0, 24),
                binding_parameter: false,
                parameters: Some(FormalParameters {
                    span: Span::new(0, 15),
                    parameters: vec![
                        BindingElement {
                            span: Span::new(1, 2),
                            pattern: Ident::new("a", (1, 2)).into(),
                            initializer: None,
                        },
                        BindingElement {
                            span: Span::new(4, 5),
                            pattern: Ident::new("b", (4, 5)).into(),
                            initializer: None,
                        }
                    ],
                    rest: Some(BindingPattern::Ident(Ident::new("rest", (10, 14)))),
                }),
                body: ArrowFunctionBody::Block(vec![
                    EmptyStatement {
                        span: Span::new(21, 22),
                    }.into()
                ])
            }.into()
        ]
    );
}

#[test]
fn async_multiple_parameters() {
    parser_test!(
        input: "async (a, b, ...rest) => { ; }",
        expr_output: [
            ArrowFunctionExpression {
                span: Span::new(0, 30),
                binding_parameter: false,
                parameters: Some(FormalParameters {
                    span: Span::new(6, 21),
                    parameters: vec![
                        BindingElement {
                            span: Span::new(7, 8),
                            pattern: Ident::new("a", (7, 8)).into(),
                            initializer: None,
                        },
                        BindingElement {
                            span: Span::new(10, 11),
                            pattern: Ident::new("b", (10, 11)).into(),
                            initializer: None,
                        }
                    ],
                    rest: Some(BindingPattern::Ident(Ident::new("rest", (16, 20)))),
                }),
                body: ArrowFunctionBody::Block(vec![
                    EmptyStatement {
                        span: Span::new(27, 28),
                    }.into()
                ])
            }.into()
        ]
    );
}
