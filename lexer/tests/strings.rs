mod utils;

use fajt_lexer::{error::Error, literal};

#[test]
fn empty_string_single_quote() {
    assert_lexer!(
        input: "''",
        output: [
            (literal!(string, '\'', ""), (0, 2)),
        ]
    );
}

#[test]
fn empty_string_double_quote() {
    assert_lexer!(
        input: r#""""#,
        output: [
            (literal!(string, '"', ""), (0, 2)),
        ]
    );
}

#[test]
fn single_quote() {
    assert_lexer!(
        input: r#"'a string literal'"#,
        output: [
            (literal!(string, '\'', "a string literal"), (0, 18)),
        ]
    );
}

#[test]
fn double_quote() {
    assert_lexer!(
        input: r#""a string literal""#,
        output: [
            (literal!(string, '"', "a string literal"), (0, 18)),
        ]
    );
}

#[test]
fn single_quote_escaped() {
    assert_lexer!(
        input: r#"'a string \' \\ literal'"#,
        output: [
            (literal!(string, '\'', r#"a string ' \ literal"#), (0, 24)),
        ]
    );
}

#[test]
fn double_quote_escaped() {
    assert_lexer!(
        input: r#""a string \" \\ literal""#,
        output: [
            (literal!(string, '"', r#"a string " \ literal"#), (0, 24)),
        ]
    );
}

#[test]
fn escaped_new_line_in_string() {
    assert_lexer!(
        input: "\"Hello\\\nworld\"",
        output: [
            (literal!(string, '"', "Hello\nworld"), (0, 14)),
        ]
    );
}

#[test]
fn escaped_carriage_return_new_line_in_string() {
    assert_lexer!(
        input: "\"Hello\\\r\nworld\"",
        output: [
            (literal!(string, '"', "Hello\r\nworld"), (0, 15)),
        ]
    );
}

#[test]
fn line_separator_in_string() {
    assert_lexer!(
        input: "\"Hello\u{2028}world\"",
        output: [
            (literal!(string, '"', "Hello\u{2028}world"), (0, 15)),
        ]
    );
}

#[test]
fn paragraph_separator_in_string() {
    assert_lexer!(
        input: "\"Hello\u{2029}world\"",
        output: [
            (literal!(string, '"', "Hello\u{2029}world"), (0, 15)),
        ]
    );
}

#[test]
fn unescaped_new_line_in_string() {
    assert_lexer!(
        input: "\"Hello\nworld\"",
        error: Error::syntax_error("String contained unescaped new line".to_owned(), (6, 6))
    );
}

#[test]
fn unescaped_carriage_return_in_string() {
    assert_lexer!(
        input: "\"Hello\rworld\"",
        error: Error::syntax_error("String contained unescaped new line".to_owned(), (6, 6))
    );
}
