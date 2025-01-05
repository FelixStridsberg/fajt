mod utils;

use fajt_lexer::error::Error;
use fajt_lexer::literal;
use fajt_lexer::punct;
use fajt_lexer::token::Token;
use fajt_lexer::LexerState;

fn lex(input: &str) -> Vec<Token> {
    let mut lexer = fajt_lexer::Lexer::new(input).expect("Could not create lexer, empty input?");
    lexer.read_all().unwrap()
}

#[test]
fn single_line_comment_leaves_new_line() {
    let tokens = lex("// Hello, I am comment.\na");
    assert!(tokens[0].first_on_line);
}

#[test]
fn no_new_line_in_multi_line_comment() {
    let tokens = lex("a/* Hello, I am comment. */b");
    assert!(!tokens[1].first_on_line);
}

#[test]
fn empty_multiline_comment() {
    let tokens = lex("/**/b");
    assert!(tokens[0].first_on_line);
}

#[test]
fn empty_single_multiline_comment() {
    let tokens = lex("/**/");
    assert_eq!(tokens.len(), 0);
}

#[test]
fn unterminated_multiline_comment() {
    assert_lexer!(
        input: "/* *",
        error: Error::syntax_error("Unterminated comment".to_owned(), (0, 2))
    );
}

#[test]
fn empty_single_comment() {
    let tokens = lex("//");
    assert_eq!(tokens.len(), 0);
}

#[test]
fn no_space_between_comments() {
    let tokens = lex("/* Hello, I am comment. *//**/b");
    assert!(tokens[0].first_on_line);
}

#[test]
fn new_line_in_multi_line_comment() {
    let tokens = lex("a/* Hello,\nI am comment. */b");
    assert!(tokens[1].first_on_line);
}

#[test]
fn single_line_html_open_comment() {
    let tokens = lex("<!-- Hello, I am comment.\na");
    assert_eq!(tokens.len(), 1);
}

#[test]
fn single_line_html_open_comment_when_not_allowed() {
    let mut lexer = fajt_lexer::Lexer::new("<!-- Hello, I am comment.").unwrap();
    lexer.set_state(LexerState::default().with_html_comments_allowed(false));

    assert_eq!(
        lexer.read_all(),
        Err(Error::syntax_error(
            "HTML comments are not allowed in this context".to_owned(),
            (0, 2)
        ))
    )
}

#[test]
fn empty_single_line_html_open_comment() {
    let tokens = lex("<!--");
    assert_eq!(tokens.len(), 0);
}

#[test]
fn single_line_html_close_comment() {
    let tokens = lex("--> Hello, I am comment.\na");
    assert_eq!(tokens.len(), 1);
}

#[test]
fn empty_single_line_html_close_comment() {
    let tokens = lex("-->");
    assert_eq!(tokens.len(), 0);
}

#[test]
fn single_line_html_close_comment_when_not_allowed() {
    let mut lexer = fajt_lexer::Lexer::new("-->").unwrap();
    lexer.set_state(LexerState::default().with_html_comments_allowed(false));

    assert_eq!(
        lexer.read_all(),
        Err(Error::syntax_error(
            "HTML comments are not allowed in this context".to_owned(),
            (0, 3)
        ))
    )
}

#[test]
fn not_single_line_html_close_comment_if_not_first() {
    assert_lexer!(
        input: "a-->1",
        output: [
            (identifier!("a"), (0, 1)),
            (punct!("--"), (1, 3)),
            (punct!(">"), (3, 4)),
            (literal!(number, "1"), (4, 5)),
        ]
    );
}

#[test]
fn single_line_html_close_comment_comment_before() {
    let tokens = lex("/* hello */ /* hello */ --> Hello, I am comment.\na");
    assert_eq!(tokens.len(), 1);
}
