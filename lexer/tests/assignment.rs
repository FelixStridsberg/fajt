mod lib;

use fajt_lexer::keyword;
use fajt_lexer::literal;
use fajt_lexer::punct;
use fajt_lexer::token;

#[test]
fn assignment_var() {
    assert_lexer!(
        input: "var variable = 1",
        output: [
            (keyword!("var"), (0, 3)),
            (identifier!("variable"), (4, 12)),
            (punct!("="), (13, 14)),
            (literal!(integer, 1), (15, 16)),
        ]
    );
}

#[test]
fn assignment_let() {
    assert_lexer!(
        input: "let variable = 1",
        output: [
            (keyword!("let"), (0, 3)),
            (identifier!("variable"), (4, 12)),
            (punct!("="), (13, 14)),
            (literal!(integer, 1), (15, 16)),
        ]
    );
}

#[test]
fn assignment_const() {
    assert_lexer!(
        input: "const variable = 1;",
        output: [
            (keyword!("const"), (0, 5)),
            (identifier!("variable"), (6, 14)),
            (punct!("="), (15, 16)),
            (literal!(integer, 1), (17, 18)),
            (punct!(";"), (18, 19)),
        ]
    );
}

#[test]
fn assignment_multiply() {
    assert_lexer!(
        input: "const variable *= 1",
        output: [
            (keyword!("const"), (0, 5)),
            (identifier!("variable"), (6, 14)),
            (punct!("*="), (15, 17)),
            (literal!(integer, 1), (18, 19)),
        ]
    );
}

#[test]
fn assignment_exponent() {
    assert_lexer!(
        input: "const variable **= 1",
        output: [
            (keyword!("const"), (0, 5)),
            (identifier!("variable"), (6, 14)),
            (punct!("**="), (15, 18)),
            (literal!(integer, 1), (19, 20)),
        ]
    );
}

#[test]
fn assignment_divide() {
    assert_lexer!(
        input: "const variable /= 1;",
        output: [
            (keyword!("const"), (0, 5)),
            (identifier!("variable"), (6, 14)),
            (punct!("/="), (15, 17)),
            (literal!(integer, 1), (18, 19)),
            (punct!(";"), (19, 20)),
        ]
    );
}

#[test]
fn assignment_mod() {
    assert_lexer!(
        input: "const variable %= 1",
        output: [
            (keyword!("const"), (0, 5)),
            (identifier!("variable"), (6, 14)),
            (punct!("%="), (15, 17)),
            (literal!(integer, 1), (18, 19)),
        ]
    );
}

#[test]
fn assignment_add() {
    assert_lexer!(
        input: "const variable += 1;",
        output: [
            (keyword!("const"), (0, 5)),
            (identifier!("variable"), (6, 14)),
            (punct!("+="), (15, 17)),
            (literal!(integer, 1), (18, 19)),
            (punct!(";"), (19, 20)),
        ]
    );
}

#[test]
fn assignment_subtract() {
    assert_lexer!(
        input: "const variable -= 1",
        output: [
            (keyword!("const"), (0, 5)),
            (identifier!("variable"), (6, 14)),
            (punct!("-="), (15, 17)),
            (literal!(integer, 1), (18, 19)),
        ]
    );
}

#[test]
fn assignment_bitwise_and() {
    assert_lexer!(
        input: "const variable &= 1;",
        output: [
            (keyword!("const"), (0, 5)),
            (identifier!("variable"), (6, 14)),
            (punct!("&="), (15, 17)),
            (literal!(integer, 1), (18, 19)),
            (punct!(";"), (19, 20)),
        ]
    );
}

#[test]
fn assignment_bitwise_xor() {
    assert_lexer!(
        input: "const variable ^= 1;",
        output: [
            (keyword!("const"), (0, 5)),
            (identifier!("variable"), (6, 14)),
            (punct!("^="), (15, 17)),
            (literal!(integer, 1), (18, 19)),
            (punct!(";"), (19, 20)),
        ]
    );
}

#[test]
fn assignment_bitwise_or() {
    assert_lexer!(
        input: "const variable |= 1;",
        output: [
            (keyword!("const"), (0, 5)),
            (identifier!("variable"), (6, 14)),
            (punct!("|="), (15, 17)),
            (literal!(integer, 1), (18, 19)),
            (punct!(";"), (19, 20)),
        ]
    );
}

#[test]
fn assignment_bitwise_left_shift() {
    assert_lexer!(
        input: "const variable <<= 1;",
        output: [
            (keyword!("const"), (0, 5)),
            (identifier!("variable"), (6, 14)),
            (punct!("<<="), (15, 18)),
            (literal!(integer, 1), (19, 20)),
            (punct!(";"), (20, 21)),
        ]
    );
}

#[test]
fn assignment_bitwise_right_shift() {
    assert_lexer!(
        input: "const variable >>= 1;",
        output: [
            (keyword!("const"), (0, 5)),
            (identifier!("variable"), (6, 14)),
            (punct!(">>="), (15, 18)),
            (literal!(integer, 1), (19, 20)),
            (punct!(";"), (20, 21)),
        ]
    );
}

#[test]
fn assignment_bitwise_unsigned_right_shift() {
    assert_lexer!(
        input: "const variable >>>= 1;",
        output: [
            (keyword!("const"), (0, 5)),
            (identifier!("variable"), (6, 14)),
            (punct!(">>>="), (15, 19)),
            (literal!(integer, 1), (20, 21)),
            (punct!(";"), (21, 22)),
        ]
    );
}
