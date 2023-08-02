mod lib;

use fajt_lexer::{error::Error, token::TokenValue::Identifier};

#[test]
fn ascii() {
    assert_lexer!(
        input: "abcdef1234",
        output: [
            (Identifier("abcdef1234".to_owned()), (0, 10)),
        ]
    );
}

#[test]
fn non_ascii() {
    assert_lexer!(
        input: "\u{0636}\u{05DC}\u{0998}",
        output: [
            (Identifier("\u{0636}\u{05DC}\u{0998}".to_owned()), (0, 5)),
        ]
    );
}

#[test]
fn unicode_escaped_hex_ident_start() {
    assert_lexer!(
        input: r#"\u0069dent"#,
        output: [
            (Identifier("ident".to_owned()), (0, 10)),
        ]
    );
}

#[test]
fn unicode_escaped_codepoint_ident_start() {
    assert_lexer!(
        input: r#"\u{0069}dent"#,
        output: [
            (Identifier("ident".to_owned()), (0, 12)),
        ]
    );
}

#[test]
fn unicode_escaped_hex_ident_middle() {
    assert_lexer!(
        input: r#"id\u0065nt"#,
        output: [
            (Identifier("ident".to_owned()), (0, 10)),
        ]
    );
}

#[test]
fn unicode_escaped_codepoint_ident_middle() {
    assert_lexer!(
        input: r#"id\u{0065}nt"#,
        output: [
            (Identifier("ident".to_owned()), (0, 12)),
        ]
    );
}

#[test]
fn unicode_escaped_codepoint_short() {
    assert_lexer!(
        input: r#"\u{65}"#,
        output: [
            (Identifier("i".to_owned()), (0, 12)),
        ]
    );
}

#[test]
fn unicode_escaped_codepoint_just_a_slash() {
    assert_lexer!(
        input: r#"\"#,
        error: Error::unexpected_end_of_stream()
    );
}

#[test]
fn unicode_escaped_codepoint_no_numbers() {
    assert_lexer!(
        input: r#"\u"#,
        error: Error::syntax_error("invalid escape sequence".to_owned(), (0, 2))
    );
}

#[test]
fn unicode_escaped_codepoint_not_a_u() {
    assert_lexer!(
        input: r#"\a0065"#,
        error: Error::syntax_error("invalid escape sequence".to_owned(), (0, 2))
    );
}

#[test]
fn unicode_escaped_hex_invalid_hex_value() {
    assert_lexer!(
        input: r#"id\u00g6nt"#,
        error: Error::syntax_error("invalid escape sequence".to_owned(), (2, 7))
    );
}

#[test]
fn unicode_escaped_codepoint_invalid_hex_value() {
    assert_lexer!(
        input: r#"id\u{00g6}nt"#,
        error: Error::syntax_error("invalid escape sequence".to_owned(), (2, 8))
    );
}
