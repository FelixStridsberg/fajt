mod utils;

use fajt_lexer::{literal, LexerState};

#[test]
fn single_char() {
    assert_lexer!(
        state: LexerState::regex_allowed(),
        input: "/a/",
        output: [
            (literal!(regexp, "/a/"), (0, 3)),
        ]
    );
}

#[test]
fn multiple_chars() {
    assert_lexer!(
        state: LexerState::regex_allowed(),
        input: "/abcd/",
        output: [
            (literal!(regexp, "/abcd/"), (0, 6)),
        ]
    );
}

#[test]
fn escaped_backslash() {
    assert_lexer!(
        state: LexerState::regex_allowed(),
        input: r#"/a\\b/"#,
        output: [
            (literal!(regexp, r#"/a\\b/"#), (0, 6)),
        ]
    );
}

#[test]
fn forward_slash_in_group() {
    assert_lexer!(
        state: LexerState::regex_allowed(),
        input: r#"/a[/]b/"#,
        output: [
            (literal!(regexp, r#"/a[/]b/"#), (0, 7)),
        ]
    );
}
