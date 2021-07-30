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
