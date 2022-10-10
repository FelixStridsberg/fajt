mod lib;

use fajt_lexer::token::TokenValue::Identifier;

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
fn unicode_escaped_hex() {
    assert_lexer!(
        input: r#"\u0069dent"#,
        output: [
            (Identifier("ident".to_owned()), (0, 10)),
        ]
    );
}

#[test]
fn unicode_escaped_codepoint() {
    assert_lexer!(
        input: r#"\u{0069}dent"#,
        output: [
            (Identifier("ident".to_owned()), (0, 12)),
        ]
    );
}
