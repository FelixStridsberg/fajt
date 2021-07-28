use fajt_lexer::token::Span;
use fajt_parser::ast::{
    BreakStatement, ContinueStatement, Ident, IfStatement, ReturnStatement, Statement,
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
                consequent: Statement::Expression(Ident::new("b", (9, 10)).into()),
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
                consequent: Statement::Expression(Ident::new("b", (9, 10)).into()),
                alternate: Some(Statement::Expression(Ident::new("c", (16, 17)).into())),
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
