mod r#macro;

use fajt_lexer::token::{Span, Token, TokenValue};
use fajt_parser::ast::{Ident, StmtBlock, StmtExpr, StmtDoWhile, StmtEmpty};
use fajt_parser::error::ErrorKind::UnexpectedToken;

#[test]
fn expr_stmt_with_semicolon() {
    parser_test!(
        input: "a;",
        program_span: Span::new(0, 2)
    );
}

#[test]
fn expr_stmt_without_semicolon() {
    parser_test!(
        input: "a",
        program_span: Span::new(0, 1)
    );
}

#[test]
fn break_stmt_with_semicolon() {
    parser_test!(
        input: "break;",
        program_span: Span::new(0, 6)
    );
}

#[test]
fn break_stmt_without_semicolon() {
    parser_test!(
        input: "break",
        program_span: Span::new(0, 5)
    );
}

#[test]
fn continue_stmt_with_semicolon() {
    parser_test!(
        input: "continue;",
        program_span: Span::new(0, 9)
    );
}

#[test]
fn continue_stmt_without_semicolon() {
    parser_test!(
        input: "continue",
        program_span: Span::new(0, 8)
    );
}

#[test]
fn return_stmt_with_semicolon() {
    parser_test!(
        input: "return;",
        program_span: Span::new(0, 7)
    );
}

#[test]
fn return_stmt_without_semicolon() {
    parser_test!(
        input: "return",
        program_span: Span::new(0, 6)
    );
}

#[test]
fn throw_stmt_with_semicolon() {
    parser_test!(
        input: "throw a;",
        program_span: Span::new(0, 8)
    );
}

#[test]
fn throw_stmt_without_semicolon() {
    parser_test!(
        input: "throw a",
        program_span: Span::new(0, 7)
    );
}

#[test]
fn debugger_stmt_with_semicolon() {
    parser_test!(
        input: "debugger;",
        program_span: Span::new(0, 9)
    );
}

#[test]
fn debugger_stmt_without_semicolon() {
    parser_test!(
        input: "debugger",
        program_span: Span::new(0, 8)
    );
}

#[test]
fn var_stmt_with_semicolon() {
    parser_test!(
        input: "var a;",
        program_span: Span::new(0, 6)
    );

    parser_test!(
        input: "var a, b;",
        program_span: Span::new(0, 9)
    );

    parser_test!(
        input: "var a = 1;",
        program_span: Span::new(0, 10)
    );
}

#[test]
fn var_stmt_without_semicolon() {
    parser_test!(
        input: "var a",
        program_span: Span::new(0, 5)
    );

    parser_test!(
        input: "var a, b",
        program_span: Span::new(0, 8)
    );

    parser_test!(
        input: "var a = 1",
        program_span: Span::new(0, 9)
    );
}

#[test]
fn do_while_stmt_with_semicolon() {
    parser_test!(
        input: "do {} while (true);",
        program_span: Span::new(0, 19)
    );
}

#[test]
fn do_while_stmt_without_semicolon() {
    parser_test!(
        input: "do {} while (true)",
        program_span: Span::new(0, 18)
    );
}

#[test]
fn missing_semicolon_between_expr_stmt() {
    parser_test!(
        input: "a b",
        error: UnexpectedToken(Token::new(TokenValue::Identifier("b".to_owned()), false, (2, 3)))
    );
}

#[test]
fn auto_semicolon_separated_by_line() {
    parser_test!(
        input: "a\nb",
        output: [
            StmtExpr {
                span: Span::new(0, 1),
                expr: Ident::new("a", (0, 1)).into(),
            }.into(),
            StmtExpr {
                span: Span::new(2, 3),
                expr: Ident::new("b", (2, 3)).into(),
            }.into()
        ]
    );
}

#[test]
fn auto_semicolon_separated_by_close_brace() {
    parser_test!(
        input: "{ a } b",
        output: [
            StmtBlock {
                span: Span::new(0, 5),
                statements: vec![
                    StmtExpr {
                        span: Span::new(2, 3),
                        expr: Ident::new("a", (2, 3)).into(),
                    }.into(),
                ]
            }.into(),
            StmtExpr {
                span: Span::new(6, 7),
                expr: Ident::new("b", (6, 7)).into(),
            }.into()
        ]
    );
}

#[test]
fn auto_semicolon_after_do_while() {
    parser_test!(
        input: "do ; while(a) b",
        output: [
            StmtDoWhile {
                span: Span::new(0, 13),
                body: StmtEmpty {
                    span: Span::new(3, 4),
                }.into(),
                test: Ident::new("a", (11, 12)).into(),
            }.into(),
            StmtExpr {
                span: Span::new(14, 15),
                expr:  Ident::new("b", (14, 15)).into(),
            }.into()
        ]
    );
}