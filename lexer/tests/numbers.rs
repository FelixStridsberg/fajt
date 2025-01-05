mod utils;

use fajt_lexer::error::Error;
use fajt_lexer::literal;

#[test]
fn number_integer_decimal() {
    assert_lexer!(
        input: "1234",
        output: [
            (literal!(number, "1234"), (0, 4)),
        ]
    );
}

#[test]
fn number_decimal() {
    assert_lexer!(
        input: "1234.5678",
        output: [
            (literal!(number, "1234.5678"), (0, 9)),
        ]
    );
}

#[test]
fn number_decimal_no_integral() {
    assert_lexer!(
        input: ".5",
        output: [
            (literal!(number, ".5"), (0, 2)),
        ]
    );
}

#[test]
fn number_decimal_no_decimals() {
    assert_lexer!(
        input: "1.",
        output: [
            (literal!(number, "1."), (0, 2)),
        ]
    );
}

#[test]
fn number_hex() {
    assert_lexer!(
        input: "0xff08",
        output: [
            (literal!(number, "0xff08"), (0, 6)),
        ]
    );
}

#[test]
fn number_hex_uppercase() {
    assert_lexer!(
        input: "0XFF08",
        output: [
            (literal!(number, "0XFF08"), (0, 6)),
        ]
    );
}

#[test]
fn number_hex_missing_number() {
    assert_lexer!(
        input: "0x",
        error: Error::syntax_error("expected number".to_owned(), (2, 2))
    );
}

#[test]
fn number_hex_uppercase_missing_number() {
    assert_lexer!(
        input: "0X",
        error: Error::syntax_error("expected number".to_owned(), (2, 2))
    );
}

#[test]
fn number_octal() {
    assert_lexer!(
        input: "0o347",
        output: [
            (literal!(number, "0o347"), (0, 5)),
        ]
    );
}

#[test]
fn number_octal_uppercase() {
    assert_lexer!(
        input: "0O347",
        output: [
            (literal!(number, "0O347"), (0, 5)),
        ]
    );
}

#[test]
fn number_octal_missing_number() {
    assert_lexer!(
        input: "0o",
        error: Error::syntax_error("expected number".to_owned(), (2, 2))
    );
}

#[test]
fn number_octal_uppercase_missing_number() {
    assert_lexer!(
        input: "0O",
        error: Error::syntax_error("expected number".to_owned(), (2, 2))
    );
}

#[test]
fn number_binary() {
    assert_lexer!(
        input: "0b10111100",
        output: [
            (literal!(number, "0b10111100"), (0, 10)),
        ]
    );
}

#[test]
fn number_binary_uppercase() {
    assert_lexer!(
        input: "0B10111100",
        output: [
            (literal!(number, "0B10111100"), (0, 10)),
        ]
    );
}

#[test]
fn number_binary_missing_number() {
    assert_lexer!(
        input: "0b",
        error: Error::syntax_error("expected number".to_owned(), (2, 2))
    );
}

#[test]
fn number_binary_uppercase_missing_number() {
    assert_lexer!(
        input: "0B",
        error: Error::syntax_error("expected number".to_owned(), (2, 2))
    );
}

#[test]
fn number_scientific() {
    assert_lexer!(
        input: "123e10",
        output: [
            (literal!(number, "123e10"), (0, 6)),
        ]
    );
}

#[test]
fn number_scientific_uppercase() {
    assert_lexer!(
        input: "123E10",
        output: [
            (literal!(number, "123E10"), (0, 6)),
        ]
    );
}

#[test]
fn number_scientific_positive_exponent() {
    assert_lexer!(
        input: "123e+10",
        output: [
            (literal!(number, "123e+10"), (0, 7)),
        ]
    );
}

#[test]
fn number_scientific_negative_exponent() {
    assert_lexer!(
        input: "123e-10",
        output: [
            (literal!(number, "123e-10"), (0, 7)),
        ]
    );
}

#[test]
fn number_scientific_exponent_decimal() {
    assert_lexer!(
        input: "123.11e+10",
        output: [
            (literal!(number, "123.11e+10"), (0, 10)),
        ]
    );
}

#[test]
fn number_scientific_with_missing_exponent() {
    assert_lexer!(
        input: "1e",
        error: Error::syntax_error("expected number".to_owned(), (2, 2))
    );
}

#[test]
fn number_scientific_with_invalid_exponent() {
    assert_lexer!(
        input: "1ea",
        error: Error::syntax_error("expected number".to_owned(), (2, 2))
    );
}

#[test]
fn legacy_ocal_number_is_not_supported() {
    assert_lexer!(
        input: "01",
        error: Error::syntax_error("Zero prefixed numbers are deprecated and not supported".to_owned(), (0, 1))
    );
    assert_lexer!(
        input: "09",
        error: Error::syntax_error("Zero prefixed numbers are deprecated and not supported".to_owned(), (0, 1))
    );
}
