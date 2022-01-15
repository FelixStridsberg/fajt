use fajt_ast::Span;
use fajt_lexer::token::{Comment, Token, TokenValue};

fn lex(input: &str) -> Vec<Token> {
    let mut lexer = fajt_lexer::Lexer::new(input).expect("Could not create lexer, empty input?");
    lexer.read_all().unwrap()
}

#[test]
fn single_line_comment() {
    let tokens = lex("// Hello, I am comment.");

    assert_eq!(
        tokens,
        vec![Token {
            value: TokenValue::Comment(Comment {
                multi_line: false,
                content: " Hello, I am comment.".to_string(),
            }),
            first_on_line: true,
            span: Span::new(0, 23)
        }]
    );
}

#[test]
fn single_line_comment_leaves_new_line() {
    let tokens = lex("// Hello, I am comment.\na");
    assert!(tokens[1].first_on_line);
}

#[test]
fn multiline_line_comment() {
    let tokens = lex("/* Hello, I am comment. */");
    println!("TOKENS: {:?}", tokens);

    assert_eq!(
        tokens,
        vec![Token {
            value: TokenValue::Comment(Comment {
                multi_line: true,
                content: " Hello, I am comment. ".to_string(),
            }),
            first_on_line: true,
            span: Span::new(0, 26)
        }]
    );
}
