use fajt_lexer::token::Span;
use fajt_parser::ast::{
    DoWhileStatement, EmptyStatement, ForStatement, Ident, Literal, LiteralExpression, Statement,
    WhileStatement,
};

#[test]
fn do_while() {
    parser_test!(
        input: "do a; while (true)",
        output: [
            DoWhileStatement {
                span: Span::new(0, 18),
                body: Statement::Expression(Ident::new("a", (3, 4)).into()),
                test: LiteralExpression {
                    span: Span::new(13, 17),
                    literal: Literal::Boolean(true),
                }.into()
            }.into()
        ]
    );
}

#[test]
fn r#while() {
    parser_test!(
        input: "while (true) a",
        output: [
            WhileStatement {
                span: Span::new(0, 14),
                test: LiteralExpression {
                    span: Span::new(7, 11),
                    literal: Literal::Boolean(true),
                }.into(),
                body: Statement::Expression(Ident::new("a", (13, 14)).into()),
            }.into()
        ]
    );
}

#[test]
fn empty_for() {
    parser_test!(
        input: "for (;;) ;",
        output: [
            ForStatement {
                span: Span::new(0, 10),
                init: None,
                test: None,
                update: None,
                body: EmptyStatement {
                   span: Span::new(9, 10),
                }.into(),
            }.into()
        ]
    );
}

#[test]
fn for_with_init() {
    parser_test!(
        input: "for (a;;) ;",
        output: [
            ForStatement {
                span: Span::new(0, 11),
                init: Some(Ident::new("a", (5, 6)).into()),
                test: None,
                update: None,
                body: EmptyStatement {
                   span: Span::new(10, 11),
                }.into(),
            }.into()
        ]
    );
}

#[test]
fn for_with_test() {
    parser_test!(
        input: "for (;a;) ;",
        output: [
            ForStatement {
                span: Span::new(0, 11),
                init: None,
                test: Some(Ident::new("a", (6, 7)).into()),
                update: None,
                body: EmptyStatement {
                   span: Span::new(10, 11),
                }.into(),
            }.into()
        ]
    );
}

#[test]
fn for_with_update() {
    parser_test!(
        input: "for (;;a) ;",
        output: [
            ForStatement {
                span: Span::new(0, 11),
                init: None,
                test: None,
                update: Some(Ident::new("a", (7, 8)).into()),
                body: EmptyStatement {
                   span: Span::new(10, 11),
                }.into(),
            }.into()
        ]
    );
}
