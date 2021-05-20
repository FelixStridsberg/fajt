mod lib;

use fajt_lexer::punct;

#[test]
fn parentheses() {
    assert_lexer!(
        input: "( ) [ ] { }",
        output: [
            (punct!("("), (0, 1)),
            (punct!(")"), (2, 3)),
            (punct!("["), (4, 5)),
            (punct!("]"), (6, 7)),
            (punct!("{"), (8, 9)),
            (punct!("}"), (10, 11)),
        ]
    );
}

#[test]
fn dots() {
    assert_lexer!(
        input: ". ... ; ,",
        output: [
            (punct!("."), (0, 1)),
            (punct!("..."), (2, 5)),
            (punct!(";"), (6, 7)),
            (punct!(","), (8, 9)),
        ]
    );
}

#[test]
fn math() {
    assert_lexer!(
        input: "< > + - * / % ** ++ --",
        output: [
            (punct!("<"), (0, 1)),
            (punct!(">"), (2, 3)),
            (punct!("+"), (4, 5)),
            (punct!("-"), (6, 7)),
            (punct!("*"), (8, 9)),
            (punct!("/"), (10, 11)),
            (punct!("%"), (12, 13)),
            (punct!("**"), (14, 16)),
            (punct!("++"), (17, 19)),
            (punct!("--"), (20, 22)),
        ]
    );
}

#[test]
fn assignments() {
    assert_lexer!(
        input: "<= >= == != === !== = += -= *= %= **= <<= >>= >>>= &= |= ^= => /=",
        output: [
            (punct!("<="), (0, 2)),
            (punct!(">="), (3, 5)),
            (punct!("=="), (6, 8)),
            (punct!("!="), (9, 11)),
            (punct!("==="), (12, 15)),
            (punct!("!=="), (16, 19)),
            (punct!("="), (20, 21)),
            (punct!("+="), (22, 24)),
            (punct!("-="), (25, 27)),
            (punct!("*="), (28, 30)),
            (punct!("%="), (31, 33)),
            (punct!("**="), (34, 37)),
            (punct!("<<="), (38, 41)),
            (punct!(">>="), (42, 45)),
            (punct!(">>>="), (46, 50)),
            (punct!("&="), (51, 53)),
            (punct!("|="), (54, 56)),
            (punct!("^="), (57, 59)),
            (punct!("=>"), (60, 62)),
            (punct!("/="), (63, 65)),
        ]
    );
}

#[test]
fn bitwise() {
    assert_lexer!(
        input: "<< >> >>> & | ^ ~",
        output: [
            (punct!("<<"), (0, 2)),
            (punct!(">>"), (3, 5)),
            (punct!(">>>"), (6, 9)),
            (punct!("&"), (10, 11)),
            (punct!("|"), (12, 13)),
            (punct!("^"), (14, 15)),
            (punct!("~"), (16, 17)),
        ]
    );
}

#[test]
fn others() {
    assert_lexer!(
        input: "&& || ?? ? : !",
        output: [
            (punct!("&&"), (0, 2)),
            (punct!("||"), (3, 5)),
            (punct!("??"), (6, 8)),
            (punct!("?"), (9, 10)),
            (punct!(":"), (11, 12)),
            (punct!("!"), (13, 14)),
        ]
    );
}
