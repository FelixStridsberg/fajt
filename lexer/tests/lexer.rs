use fajt_ast::Span;
use fajt_common::io::PeekRead;
use fajt_lexer::token::{Token, TokenValue};
use fajt_lexer::Lexer;
use std::io::{Seek, SeekFrom};

#[test]
fn first_on_line() {
    let input = "async function\nasync function";
    let mut lexer = Lexer::new(input).unwrap();
    let tokens = lexer.read_all().unwrap();

    assert_eq!(tokens[0].first_on_line, true);
    assert_eq!(tokens[1].first_on_line, false);
    assert_eq!(tokens[2].first_on_line, true);
    assert_eq!(tokens[3].first_on_line, false);
}

#[test]
fn seek_from_start() {
    let input = "ident1; ident2; ident3;";
    let mut lexer = Lexer::new(input).unwrap();
    lexer.seek(SeekFrom::Start(8)).unwrap();

    let ident = lexer.next().unwrap();
    assert_eq!(
        ident,
        (
            14,
            Token {
                span: Span::new(8, 14),
                value: TokenValue::Identifier("ident2".to_string()),
                first_on_line: true,
            }
        )
    );
}

#[test]
fn seek_from_end() {
    let input = "ident1; ident2; ident3;";
    let mut lexer = Lexer::new(input).unwrap();
    lexer.seek(SeekFrom::End(-7)).unwrap();

    let ident = lexer.next().unwrap();
    assert_eq!(
        ident,
        (
            22,
            Token {
                span: Span::new(16, 22),
                value: TokenValue::Identifier("ident3".to_string()),
                first_on_line: true,
            }
        )
    );
}

#[test]
fn seek_from_current() {
    let input = "ident1; ident2; ident3;";
    let mut lexer = Lexer::new(input).unwrap();
    lexer.next().unwrap();

    lexer.seek(SeekFrom::Current(-6)).unwrap();

    let ident = lexer.next().unwrap();
    assert_eq!(
        ident,
        (
            6,
            Token {
                span: Span::new(0, 6),
                value: TokenValue::Identifier("ident1".to_string()),
                first_on_line: true,
            }
        )
    );
}
