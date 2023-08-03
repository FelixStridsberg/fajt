mod utils;

use fajt_lexer::literal;

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
