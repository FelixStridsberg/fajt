mod utils;

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
fn unicode_escape_sequence_hex_ident_start() {
    assert_lexer!(
        input: r#"\u0069dent"#,
        output: [
            (Identifier("ident".to_owned()), (0, 10)),
        ]
    );
}

#[test]
fn unicode_escape_sequence_hex_ident_middle() {
    assert_lexer!(
        input: r#"id\u0065nt"#,
        output: [
            (Identifier("ident".to_owned()), (0, 10)),
        ]
    );
}

#[test]
fn unicode_escape_sequence_hex_mixed_case() {
    assert_lexer!(
        input: r#"f\u004F\u004f"#,
        output: [
            (Identifier("fOO".to_owned()), (0, 13)),
        ]
    );
}

#[test]
fn unicode_escape_sequence_hex_invalid_character() {
    assert_lexer!(
        input: r#"id\u00g6nt"#,
        error: Error::syntax_error("invalid escape sequence".to_owned(), (2, 7))
    );
}

#[test]
fn unicode_escape_sequence_hex_invalid_id_start() {
    assert_lexer!(
        input: r#"\u0030foo"#,
        error: Error::syntax_error("invalid escape sequence".to_owned(), (0, 6))
    );
}

#[test]
fn unicode_escape_sequence_hex_valid_id_continue() {
    assert_lexer!(
        input: r#"fo\u0030"#,
        output: [
            (Identifier("fo0".to_owned()), (0, 8)),
        ]
    );
}

#[test]
fn unicode_escape_sequence_hex_invalid_id_continue() {
    assert_lexer!(
        input: r#"foo\u0020"#,
        error: Error::syntax_error("invalid escape sequence".to_owned(), (3, 9))
    );
}

#[test]
fn unicode_escape_sequence_code_point_ident_start() {
    assert_lexer!(
        input: r#"\u{0069}dent"#,
        output: [
            (Identifier("ident".to_owned()), (0, 12)),
        ]
    );
}

#[test]
fn unicode_escape_sequence_code_point_ident_middle() {
    assert_lexer!(
        input: r#"id\u{0065}nt"#,
        output: [
            (Identifier("ident".to_owned()), (0, 12)),
        ]
    );
}

#[test]
fn unicode_escape_sequence_code_point_mixed_case() {
    assert_lexer!(
        input: r#"f\u{4F}\u{4f}"#,
        output: [
            (Identifier("fOO".to_owned()), (0, 13)),
        ]
    );
}

#[test]
fn unicode_escape_sequence_code_point_short() {
    assert_lexer!(
        input: r#"\u{69}"#,
        output: [
            (Identifier("i".to_owned()), (0, 6)),
        ]
    );
}

#[test]
fn unicode_escape_sequence_code_point_invalid_character() {
    assert_lexer!(
        input: r#"id\u{00g6}nt"#,
        error: Error::syntax_error("invalid escape sequence".to_owned(), (2, 8))
    );
}

#[test]
fn unicode_escape_sequence_code_point_invalid_id_start() {
    assert_lexer!(
        input: r#"\u{30}foo"#,
        error: Error::syntax_error("invalid escape sequence".to_owned(), (0, 6))
    );
}

#[test]
fn unicode_escape_sequence_code_point_valid_id_continue() {
    assert_lexer!(
        input: r#"fo\u{30}"#,
        output: [
            (Identifier("fo0".to_owned()), (0, 8)),
        ]
    );
}

#[test]
fn unicode_escape_sequence_code_point_invalid_id_continue() {
    assert_lexer!(
        input: r#"foo\u{20}"#,
        error: Error::syntax_error("invalid escape sequence".to_owned(), (3, 9))
    );
}

#[test]
fn unicode_escape_sequence_code_point_overflow_int() {
    assert_lexer!(
        input: r#"\u{fffffffffffffffffffff}"#,
        error: Error::syntax_error("invalid escape sequence".to_owned(), (0, 25))
    );
}

#[test]
fn unicode_escape_sequence_code_point_many_zeros() {
    assert_lexer!(
        input: r#"\u{000000000000000065}"#,
        output: [
            (Identifier("e".to_owned()), (0, 22)),
        ]
    );
}

#[test]
fn unicode_escape_sequence_just_a_slash() {
    assert_lexer!(
        input: r#"\"#,
        error: Error::unexpected_end_of_stream()
    );
}

#[test]
fn unicode_escape_sequence_no_numbers() {
    assert_lexer!(
        input: r#"\u"#,
        error: Error::unexpected_end_of_stream()
    );
}

#[test]
fn unicode_escape_sequence_hex_not_a_u() {
    assert_lexer!(
        input: r#"\a0065"#,
        error: Error::syntax_error("invalid escape sequence".to_owned(), (0, 2))
    );
}

#[test]
fn unicode_escape_sequence_code_point_not_a_u() {
    assert_lexer!(
        input: r#"\a0065"#,
        error: Error::syntax_error("invalid escape sequence".to_owned(), (0, 2))
    );
}

#[test]
fn zwnj_ident_continue() {
    assert_lexer!(
        input: "id\u{200c}ent",
        output: [
            (Identifier("id\u{200c}ent".to_owned()), (0, 8)),
        ]
    );
}

#[test]
fn zwnj_ident_start() {
    assert_lexer!(
        input: "\u{200c}ident",
        error: Error::unrecognized_code_point('\u{200c}', (0, 3))
    );
}

#[test]
fn zwj_ident_continue() {
    assert_lexer!(
        input: "id\u{200d}ent",
        output: [
            (Identifier("id\u{200d}ent".to_owned()), (0, 8)),
        ]
    );
}

#[test]
fn zwj_ident_start() {
    assert_lexer!(
        input: "\u{200d}ident",
        error: Error::unrecognized_code_point('\u{200d}', (0, 3))
    );
}
