mod utils;

use fajt_lexer::error::Error;
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

#[test]
fn new_line_in_regexp() {
    assert_lexer!(
        state: LexerState::regex_allowed(),
        input: "/a\nb/",
        error: Error::syntax_error("Unterminated regular expression literal".to_owned(), (0, 2))
    );
}

#[test]
fn escaped_new_line_in_regexp() {
    assert_lexer!(
        state: LexerState::regex_allowed(),
        input: "/a\\\nb/",
        error: Error::syntax_error("Unterminated regular expression literal".to_owned(), (0, 3))
    );
}
