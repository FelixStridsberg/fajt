use fajt_lexer::token::Token;

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
fn new_line_in_multi_line_comment() {
    let tokens = lex("a/* Hello,\nI am comment. */b");
    assert!(tokens[1].first_on_line);
}
