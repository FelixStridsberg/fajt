mod lib;

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
        input: "1234.5",
        output: [
            (literal!(decimal, 1234.5), (0, 6)),
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
fn number_octal() {
    assert_lexer!(
        input: "0o347",
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
