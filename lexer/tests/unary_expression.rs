mod lib;

use fajt_lexer::literal;
use fajt_lexer::punct;

#[test]
fn lex_expression_add() {
    assert_lexer!(
        input: "1 + 1",
        output: [
            (literal!(integer, 1), (0, 1)),
            (punct!("+"), (2, 3)),
            (literal!(integer, 1), (4, 5)),
        ]
    );
}

#[test]
fn lex_expression_subtract() {
    assert_lexer!(
        input: "1 - 1",
        output: [
            (literal!(integer, 1), (0, 1)),
            (punct!("-"), (2, 3)),
            (literal!(integer, 1), (4, 5)),
        ]
    );
}

#[test]
fn lex_expression_multiply() {
    assert_lexer!(
        input: "1 * 1",
        output: [
            (literal!(integer, 1), (0, 1)),
            (punct!("*"), (2, 3)),
            (literal!(integer, 1), (4, 5)),
        ]
    );
}

#[test]
fn lex_expression_exponent() {
    assert_lexer!(
        input: "1 ** 1",
        output: [
            (literal!(integer, 1), (0, 1)),
            (punct!("**"), (2, 4)),
            (literal!(integer, 1), (5, 6)),
        ]
    );
}

#[test]
fn lex_expression_divide() {
    assert_lexer!(
        input: "1 / 1",
        output: [
            (literal!(integer, 1), (0, 1)),
            (punct!("/"), (2, 3)),
            (literal!(integer, 1), (4, 5)),
        ]
    );
}

#[test]
fn lex_expression_modulus() {
    assert_lexer!(
        input: "1 % 1",
        output: [
            (literal!(integer, 1), (0, 1)),
            (punct!("%"), (2, 3)),
            (literal!(integer, 1), (4, 5)),
        ]
    );
}

#[test]
fn lex_expression_bitwise_and() {
    assert_lexer!(
        input: "1 & 1",
        output: [
            (literal!(integer, 1), (0, 1)),
            (punct!("&"), (2, 3)),
            (literal!(integer, 1), (4, 5)),
        ]
    );
}

#[test]
fn lex_expression_bitwise_or() {
    assert_lexer!(
        input: "1 | 1",
        output: [
            (literal!(integer, 1), (0, 1)),
            (punct!("|"), (2, 3)),
            (literal!(integer, 1), (4, 5)),
        ]
    );
}

#[test]
fn lex_expression_bitwise_xor() {
    assert_lexer!(
        input: "1 ^ 1",
        output: [
            (literal!(integer, 1), (0, 1)),
            (punct!("^"), (2, 3)),
            (literal!(integer, 1), (4, 5)),
        ]
    );
}

#[test]
fn lex_expression_bitwise_shift_left() {
    assert_lexer!(
        input: "a << 10",
        output: [
            (identifier!("a"), (0, 1)),
            (punct!("<<"), (2, 4)),
            (literal!(integer, 10), (5, 7)),
        ]
    );
}

#[test]
fn lex_expression_bitwise_shift_right() {
    assert_lexer!(
        input: "a >> 10",
        output: [
            (identifier!("a"), (0, 1)),
            (punct!(">>"), (2, 4)),
            (literal!(integer, 10), (5, 7)),
        ]
    );
}

#[test]
fn lex_expression_bitwise_unsigned_shift_right() {
    assert_lexer!(
        input: "a >>> 10",
        output: [
            (identifier!("a"), (0, 1)),
            (punct!(">>>"), (2, 5)),
            (literal!(integer, 10), (6, 8)),
        ]
    );
}

#[test]
fn lex_expression_and() {
    assert_lexer!(
        input: "a && b",
        output: [
            (identifier!("a"), (0, 1)),
            (punct!("&&"), (2, 4)),
            (identifier!("b"), (5, 6)),
        ]
    );
}

#[test]
fn lex_expression_or() {
    assert_lexer!(
        input: "a || b",
        output: [
            (identifier!("a"), (0, 1)),
            (punct!("||"), (2, 4)),
            (identifier!("b"), (5, 6)),
        ]
    );
}

#[test]
fn lex_expression_coalesce() {
    assert_lexer!(
        input: "a ?? b",
        output: [
            (identifier!("a"), (0, 1)),
            (punct!("??"), (2, 4)),
            (identifier!("b"), (5, 6)),
        ]
    );
}
