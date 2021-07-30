mod r#macro;

use fajt_lexer::token::Span;

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