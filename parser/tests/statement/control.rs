use fajt_lexer::token::Span;
use fajt_parser::ast::{
    BreakStatement, ContinueStatement, Ident, IfStatement, ReturnStatement, Statement, SwitchCase,
    SwitchStatement,
};

#[test]
fn return_void() {
    parser_test!(
        input: "return",
        output: [
            ReturnStatement {
                span: Span::new(0, 6),
                argument: None,
            }.into()
        ]
    );
}

#[test]
fn return_expression() {
    parser_test!(
        input: "return a",
        output: [
            ReturnStatement {
                span: Span::new(0, 8),
                argument: Some(Ident::new("a", (7, 8)).into()),
            }.into()
        ]
    );
}

#[test]
fn break_() {
    parser_test!(
        input: "break",
        output: [
            BreakStatement {
                span: Span::new(0, 5),
                label: None,
            }.into()
        ]
    );
}

#[test]
fn break_labelled() {
    parser_test!(
        input: "break a",
        output: [
            BreakStatement {
                span: Span::new(0, 7),
                label: Some(Ident::new("a", (6, 7)).into()),
            }.into()
        ]
    );
}

#[test]
fn continue_() {
    parser_test!(
        input: "continue",
        output: [
            ContinueStatement {
                span: Span::new(0, 8),
                label: None,
            }.into()
        ]
    );
}

#[test]
fn continue_labelled() {
    parser_test!(
        input: "continue a",
        output: [
            ContinueStatement {
                span: Span::new(0, 10),
                label: Some(Ident::new("a", (9, 10)).into()),
            }.into()
        ]
    );
}

#[test]
fn if_no_else() {
    parser_test!(
        input: "if ( a ) b",
        output: [
            IfStatement {
                span: Span::new(0, 10),
                condition: Ident::new("a", (5, 6)).into(),
                consequent: Statement::expression(Ident::new("b", (9, 10))),
                alternate: None,
            }.into()
        ]
    );
}

#[test]
fn if_with_else() {
    parser_test!(
        input: "if ( a ) b else c",
        output: [
            IfStatement {
                span: Span::new(0, 17),
                condition: Ident::new("a", (5, 6)).into(),
                consequent: Statement::expression(Ident::new("b", (9, 10))),
                alternate: Some(Statement::expression(Ident::new("c", (16, 17)))),
            }.into()
        ]
    );
}

#[test]
fn empty_switch() {
    parser_test!(
        input: "switch (a) { }",
        output: [
            SwitchStatement {
                span: Span::new(0, 14),
                discriminant: Ident::new("a", (8, 9)).into(),
                cases: vec![],
            }.into()
        ]
    );
}

#[test]
fn switch_default_empty_case() {
    parser_test!(
        input: "switch (a) { default: }",
        output: [
            SwitchStatement {
                span: Span::new(0, 23),
                discriminant: Ident::new("a", (8, 9)).into(),
                cases: vec![
                    SwitchCase {
                        span: Span::new(13, 21),
                        test: None,
                        consequent: vec![],
                    }
                ],
            }.into()
        ]
    );
}

#[test]
fn switch_default() {
    parser_test!(
        input: "switch (a) { default: b }",
        output: [
            SwitchStatement {
                span: Span::new(0, 25),
                discriminant: Ident::new("a", (8, 9)).into(),
                cases: vec![
                    SwitchCase {
                        span: Span::new(13, 23),
                        test: None,
                        consequent: vec![
                            Statement::expression(Ident::new("b", (22, 23)))
                        ],
                    }
                ],
            }.into()
        ]
    );
}

#[test]
fn switch_case_empty() {
    parser_test!(
        input: "switch (a) { case b: }",
        output: [
            SwitchStatement {
                span: Span::new(0, 22),
                discriminant: Ident::new("a", (8, 9)).into(),
                cases: vec![
                    SwitchCase {
                        span: Span::new(13, 20),
                        test: Some(Ident::new("b", (18, 19)).into()),
                        consequent: vec![],
                    }
                ],
            }.into()
        ]
    );
}

#[test]
fn switch_case() {
    parser_test!(
        input: "switch (a) { case b: c; d }",
        output: [
            SwitchStatement {
                span: Span::new(0, 27),
                discriminant: Ident::new("a", (8, 9)).into(),
                cases: vec![
                    SwitchCase {
                        span: Span::new(13, 25),
                        test: Some(Ident::new("b", (18, 19)).into()),
                        consequent: vec![
                            Statement::expression(Ident::new("c", (21, 22))),
                            Statement::expression(Ident::new("d", (24, 25))),
                        ],
                    }
                ],
            }.into()
        ]
    );
}

#[test]
fn switch_mixed() {
    parser_test!(
        input: "switch (a) { case b: case c: d; default: }",
        output: [
            SwitchStatement {
                span: Span::new(0, 42),
                discriminant: Ident::new("a", (8, 9)).into(),
                cases: vec![
                    SwitchCase {
                        span: Span::new(13, 20),
                        test: Some(Ident::new("b", (18, 19)).into()),
                        consequent: vec![],
                    },
                    SwitchCase {
                        span: Span::new(21, 31),
                        test: Some(Ident::new("c", (26, 27)).into()),
                        consequent: vec![
                            Statement::expression(Ident::new("d", (29, 30))),
                        ],
                    },
                    SwitchCase {
                        span: Span::new(32, 40),
                        test: None,
                        consequent: vec![],
                    },
                ],
            }.into()
        ]
    );
}
