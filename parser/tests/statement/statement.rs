use fajt_lexer::token::Span;
use fajt_parser::ast::*;

#[test]
fn debugger() {
    parser_test!(
        input: "debugger",
        output: [
            DebuggerStatement {
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
            EmptyStatement {
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
            BlockStatement {
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
            BlockStatement {
                span: Span::new(0, 6),
                statements: vec![
                    EmptyStatement {
                        span: Span::new(2, 3),
                    }.into(),
                    EmptyStatement {
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
            WithStatement {
                span: Span::new(0, 12),
                object: LiteralExpression {
                    span: Span::new(6, 8),
                    literal: Literal::Object(Object {
                        props: vec![],
                    })
                }.into(),
                body: BlockStatement {
                    span: Span::new(10, 12),
                    statements: vec![],
                }.into(),
            }.into()
        ]
    );
}
