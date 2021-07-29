use fajt_lexer::token::Span;
use fajt_parser::ast::{CatchClause, Ident, StmtBlock, StmtThrow, StmtTry};

#[test]
fn return_void() {
    parser_test!(
        input: "throw",
        output: [
            StmtThrow {
                span: Span::new(0, 5),
                argument: None,
            }.into()
        ]
    );
}

#[test]
fn return_expression() {
    parser_test!(
        input: "throw a",
        output: [
            StmtThrow {
                span: Span::new(0, 7),
                argument: Some(Ident::new("a", (6, 7)).into()),
            }.into()
        ]
    );
}

#[test]
fn try_catch_no_param() {
    parser_test!(
        input: "try {} catch {}",
        output: [
            StmtTry {
                span: Span::new(0, 15),
                block: StmtBlock {
                    span: Span::new(4, 6),
                    statements: vec![],
                }.into(),
                handler: Some(CatchClause {
                    span: Span::new(7, 15),
                    parameter: None,
                    body: StmtBlock {
                        span: Span::new(13, 15),
                        statements: vec![],
                    }.into(),
                }),
                finalizer: None,
            }.into()
        ]
    );
}

#[test]
fn try_catch_with_param() {
    parser_test!(
        input: "try {} catch(e) {}",
        output: [
            StmtTry {
                span: Span::new(0, 18),
                block: StmtBlock {
                    span: Span::new(4, 6),
                    statements: vec![],
                }.into(),
                handler: Some(CatchClause {
                    span: Span::new(7, 18),
                    parameter: Some(Ident::new("e", (13, 14)).into()),
                    body: StmtBlock {
                        span: Span::new(16, 18),
                        statements: vec![],
                    }.into(),
                }),
                finalizer: None,
            }.into()
        ]
    );
}

#[test]
fn try_finally() {
    parser_test!(
        input: "try {} finally {}",
        output: [
            StmtTry {
                span: Span::new(0, 17),
                block: StmtBlock {
                    span: Span::new(4, 6),
                    statements: vec![],
                }.into(),
                handler: None,
                finalizer: Some(StmtBlock {
                    span: Span::new(15, 17),
                    statements: vec![],
                }.into()),
            }.into()
        ]
    );
}

#[test]
fn try_catch_finally() {
    parser_test!(
        input: "try {} catch(e) {} finally {}",
        output: [
            StmtTry {
                span: Span::new(0, 29),
                block: StmtBlock {
                    span: Span::new(4, 6),
                    statements: vec![],
                }.into(),
                handler: Some(CatchClause {
                    span: Span::new(7, 18),
                    parameter: Some(Ident::new("e", (13, 14)).into()),
                    body: StmtBlock {
                        span: Span::new(16, 18),
                        statements: vec![],
                    }.into(),
                }),
                finalizer: Some(StmtBlock {
                    span: Span::new(27, 29),
                    statements: vec![],
                }.into()),
            }.into()
        ]
    );
}
