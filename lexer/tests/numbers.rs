mod utils;

use fajt_lexer::error::Error;
use fajt_lexer::literal;

#[test]
fn number_integer_decimal() {
    assert_lexer!(
        input: "1234",
        output: [
            (literal!(integer, 1234), (0, 4)),
        ]
    );
}

#[test]
fn number_decimal() {
    assert_lexer!(
        input: "1234.5678",
        output: [
            (literal!(decimal, 1234.5678), (0, 9)),
        ]
    );
}

#[test]
fn number_decimal_no_integral() {
    assert_lexer!(
        input: ".5",
        output: [
            (literal!(decimal, 0.5), (0, 2)),
        ]
    );
}

#[test]
fn number_decimal_no_decimals() {
    assert_lexer!(
        input: "1.",
        output: [
            (literal!(decimal, 1.0), (0, 2)),
        ]
    );
}

#[test]
fn number_hex() {
    assert_lexer!(
        input: "0xff08",
        output: [
            (literal!(hex, 0xff08), (0, 6)),
        ]
    );
}

#[test]
fn number_hex_uppercase() {
    assert_lexer!(
        input: "0XFF08",
        output: [
            (literal!(hex, 0xff08), (0, 6)),
        ]
    );
}

#[test]
fn number_octal() {
    assert_lexer!(
        input: "0o347",
        output: [
            (literal!(octal, 0o347), (0, 5)),
        ]
    );
}

#[test]
fn number_octal_uppercase() {
    assert_lexer!(
        input: "0O347",
        output: [
            (literal!(octal, 0o347), (0, 5)),
        ]
    );
}

#[test]
fn number_binary() {
    assert_lexer!(
        input: "0b10111100",
        output: [
            (literal!(binary, 0b10111100), (0, 10)),
        ]
    );
}

#[test]
fn number_binary_uppercase() {
    assert_lexer!(
        input: "0B10111100",
        output: [
            (literal!(binary, 0b10111100), (0, 10)),
        ]
    );
}

#[test]
fn number_scientific() {
    assert_lexer!(
        input: "123e10",
        output: [
            (literal!(scientific, 123.0, 10), (0, 6)),
        ]
    );
}

#[test]
fn number_scientific_uppercase() {
    assert_lexer!(
        input: "123E10",
        output: [
            (literal!(scientific, 123.0, 10), (0, 6)),
        ]
    );
}

#[test]
fn number_scientific_positive_exponent() {
    assert_lexer!(
        input: "123e+10",
        output: [
            (literal!(scientific, 123.0, 10), (0, 7)),
        ]
    );
}

#[test]
fn number_scientific_negative_exponent() {
    assert_lexer!(
        input: "123e-10",
        output: [
            (literal!(scientific, 123.0, -10), (0, 7)),
        ]
    );
}

#[test]
fn number_scientific_with_missing_exponent() {
    assert_lexer!(
        input: "1e",
        error: Error::unexpected_end_of_stream()
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
