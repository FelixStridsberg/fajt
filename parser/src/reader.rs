use crate::error::Result;
use fajt_lexer::token::Token;
use fajt_lexer::Lexer;

pub struct Reader<'a> {
    lexer: Lexer<'a>,
    current: Token,
    next: Option<Token>,
    end_of_file: bool,
}

impl<'a> Reader<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Result<Self> {
        let current = lexer.read().unwrap();

        // TODO handle errors, only end of file should result in None
        let next = lexer.read().map(|t| Some(t)).unwrap_or(None);

        Ok(Reader {
            lexer,
            current,
            next,
            end_of_file: false,
        })
    }

    pub fn eof(&self) -> bool {
        self.end_of_file
    }

    pub fn current(&self) -> &Token {
        &self.current
    }

    pub fn next(&mut self) -> Result<&Token> {
        self.current = self.next.clone().unwrap(); // TODO
        self.next = self.lexer.read().map(|v| Some(v)).unwrap_or(None); // TODO
        Ok(&self.current)
    }
}
