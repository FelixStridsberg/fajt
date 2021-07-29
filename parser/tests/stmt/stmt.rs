use fajt_lexer::token::Span;
use fajt_parser::ast::*;

#[test]
fn debugger() {
    parser_test!(
        input: "debugger",
        output: [
            StmtDebugger {
                span: Span::new(0, 8),
            }.into()
        ]
    );
}

#[test]
fn empty_statement() {
    parser_test!(
        input: ";",
        output: [
            StmtEmpty {
                span: Span::new(0, 1)
            }.into()
        ]
    );
}

#[test]
fn empty_block_statement() {
    parser_test!(
        input: "{ }",
        output: [
            StmtBlock {
                span: Span::new(0, 3),
                statements: vec![]
            }.into()
        ]
    );
}

#[test]
fn block_statement() {
    parser_test!(
        input: "{ ;; }",
        output: [
            StmtBlock {
                span: Span::new(0, 6),
                statements: vec![
                    StmtEmpty {
                        span: Span::new(2, 3),
                    }.into(),
                    StmtEmpty {
                        span: Span::new(3, 4),
                    }.into()
                ]
            }.into()
        ]
    );
}

#[test]
fn with_statement() {
    parser_test!(
        input: "with ({}) {}",
        output: [
            StmtWith {
                span: Span::new(0, 12),
                object: ExprLiteral {
                    span: Span::new(6, 8),
                    literal: Literal::Object(Object {
                        props: vec![],
                    })
                }.into(),
                body: StmtBlock {
                    span: Span::new(10, 12),
                    statements: vec![],
                }.into(),
            }.into()
        ]
    );
}
