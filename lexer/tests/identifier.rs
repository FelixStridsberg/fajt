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
