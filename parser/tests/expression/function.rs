use fajt_lexer::token::Span;
use fajt_parser::ast::*;

#[test]
fn empty_anonymous_function() {
    parser_test!(
        input: "function () {}",
        expr_output: [
            ExprFunction {
                span: Span::new(0, 14),
                asynchronous: false,
                generator: false,
                identifier: None,
                parameters: FormalParameters::empty((9, 11)),
                body: vec![],
            }.into()
        ]
    );
}

#[test]
fn anonymous_function() {
    parser_test!(
        input: "function (param) { ; }",
        expr_output: [
            ExprFunction {
                span: Span::new(0, 22),
                asynchronous: false,
                generator: false,
                identifier: None,
                parameters: FormalParameters {
                    span: Span::new(9, 16),
                    bindings: vec![
                        BindingElement {
                            span: Span::new(10, 15),
                            pattern: BindingPattern::Ident(Ident::new("param", (10, 15))),
                            initializer: None,
                        }
                    ],
                    rest: None,
                },
                body: vec![
                    StmtEmpty { span: Span::new(19, 20) }.into()
                ],
            }.into()
        ]
    );
}

#[test]
fn empty_function() {
    parser_test!(
        input: "function fn() {}",
        expr_output: [
            ExprFunction {
                span: Span::new(0, 16),
                asynchronous: false,
                generator: false,
                identifier: Some(Ident::new("fn", (9, 11))),
                parameters: FormalParameters::empty((11, 13)),
                body: vec![],
            }.into()
        ]
    );
}

#[test]
fn function() {
    parser_test!(
        input: "function fn(param) { ; }",
        expr_output: [
            ExprFunction {
                span: Span::new(0, 24),
                asynchronous: false,
                generator: false,
                identifier: Some(Ident::new("fn", (9, 11))),
                parameters: FormalParameters {
                    span: Span::new(11, 18),
                    bindings: vec![
                        BindingElement {
                            span: Span::new(12, 17),
                            pattern: BindingPattern::Ident(Ident::new("param", (12, 17))),
                            initializer: None,
                        }
                    ],
                    rest: None,
                },
                body: vec![
                    StmtEmpty { span: Span::new(21, 22) }.into()
                ],
            }.into()
        ]
    );
}

#[test]
fn empty_anonymous_generator() {
    parser_test!(
        input: "function *() {}",
        expr_output: [
            ExprFunction {
                span: Span::new(0, 15),
                asynchronous: false,
                generator: true,
                identifier: None,
                parameters: FormalParameters::empty((10, 12)),
                body: vec![],
            }.into()
        ]
    );
}

#[test]
fn empty_generator() {
    parser_test!(
        input: "function *fn() {}",
        expr_output: [
            ExprFunction {
                span: Span::new(0, 17),
                asynchronous: false,
                generator: true,
                identifier: Some(Ident::new("fn", (10, 12))),
                parameters: FormalParameters::empty((12, 14)),
                body: vec![],
            }.into()
        ]
    );
}

#[test]
fn generator() {
    parser_test!(
        input: "function *(param) { ; }",
        expr_output: [
            ExprFunction {
                span: Span::new(0, 23),
                asynchronous: false,
                generator: true,
                identifier: None,
                parameters: FormalParameters {
                    span: Span::new(10, 17),
                    bindings: vec![
                        BindingElement {
                            span: Span::new(11, 16),
                            pattern: BindingPattern::Ident(Ident::new("param", (11, 16))),
                            initializer: None,
                        }
                    ],
                    rest: None,
                },
                body: vec![
                    StmtEmpty { span: Span::new(20, 21) }.into()
                ],
            }.into()
        ]
    );
}

#[test]
fn empty_anonymous_async_function() {
    parser_test!(
        input: "async function () {}",
        expr_output: [
            ExprFunction {
                span: Span::new(0, 20),
                asynchronous: true,
                generator: false,
                identifier: None,
                parameters: FormalParameters::empty((15, 17)),
                body: vec![],
            }.into()
        ]
    );
}

#[test]
fn anonymous_async_function() {
    parser_test!(
        input: "async function (param) { ; }",
        expr_output: [
            ExprFunction {
                span: Span::new(0, 28),
                asynchronous: true,
                generator: false,
                identifier: None,
                parameters: FormalParameters {
                    span: Span::new(15, 22),
                    bindings: vec![
                        BindingElement {
                            span: Span::new(16, 21),
                            pattern: BindingPattern::Ident(Ident::new("param", (16, 21))),
                            initializer: None,
                        }
                    ],
                    rest: None,
                },
                body: vec![
                    StmtEmpty { span: Span::new(25, 26) }.into()
                ],
            }.into()
        ]
    );
}

#[test]
fn empty_async_function() {
    parser_test!(
        input: "async function fn() {}",
        expr_output: [
            ExprFunction {
                span: Span::new(0, 22),
                asynchronous: true,
                generator: false,
                identifier: Some(Ident::new("fn", (15, 17))),
                parameters: FormalParameters::empty((17, 19)),
                body: vec![],
            }.into()
        ]
    );
}

#[test]
fn async_function() {
    parser_test!(
        input: "async function fn(param) { ; }",
        expr_output: [
            ExprFunction {
                span: Span::new(0, 30),
                asynchronous: true,
                generator: false,
                identifier: Some(Ident::new("fn", (15, 17))),
                parameters: FormalParameters {
                    span: Span::new(17, 24),
                    bindings: vec![
                        BindingElement {
                            span: Span::new(18, 23),
                            pattern: BindingPattern::Ident(Ident::new("param", (18, 23))),
                            initializer: None,
                        }
                    ],
                    rest: None,
                },
                body: vec![
                    StmtEmpty { span: Span::new(27, 28) }.into()
                ],
            }.into()
        ]
    );
}

#[test]
fn empty_anonymous_async_generator() {
    parser_test!(
        input: "async function *() {}",
        expr_output: [
            ExprFunction {
                span: Span::new(0, 21),
                asynchronous: true,
                generator: true,
                identifier: None,
                parameters: FormalParameters::empty((16, 18)),
                body: vec![],
            }.into()
        ]
    );
}

#[test]
fn anonymous_async_generator() {
    parser_test!(
        input: "async function *(param) { ; }",
        expr_output: [
            ExprFunction {
                span: Span::new(0, 29),
                asynchronous: true,
                generator: true,
                identifier: None,
                parameters: FormalParameters {
                    span: Span::new(16, 23),
                    bindings: vec![
                        BindingElement {
                            span: Span::new(17, 22),
                            pattern: BindingPattern::Ident(Ident::new("param", (17, 22))),
                            initializer: None,
                        }
                    ],
                    rest: None,
                },
                body: vec![
                    StmtEmpty { span: Span::new(26, 27) }.into()
                ],
            }.into()
        ]
    );
}

#[test]
fn empty_async_generator() {
    parser_test!(
        input: "async function *fn() {}",
        expr_output: [
            ExprFunction {
                span: Span::new(0, 23),
                asynchronous: true,
                generator: true,
                identifier: Some(Ident::new("fn", (16, 18))),
                parameters: FormalParameters::empty((18, 20)),
                body: vec![],
            }.into()
        ]
    );
}

#[test]
fn async_generator() {
    parser_test!(
        input: "async function* fn(param) { ; }",
        expr_output: [
            ExprFunction {
                span: Span::new(0, 31),
                asynchronous: true,
                generator: true,
                identifier: Some(Ident::new("fn", (16, 18))),
                parameters: FormalParameters {
                    span: Span::new(18, 25),
                    bindings: vec![
                        BindingElement {
                            span: Span::new(19, 24),
                            pattern: BindingPattern::Ident(Ident::new("param", (19, 24))),
                            initializer: None,
                        }
                    ],
                    rest: None,
                },
                body: vec![
                    StmtEmpty { span: Span::new(28, 29) }.into()
                ],
            }.into()
        ]
    );
}
