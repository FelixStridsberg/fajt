use crate::error::Result;
use core::mem;
use fajt_lexer::error::ErrorKind::EndOfFile;
use fajt_lexer::token::Token;
use fajt_lexer::Lexer;

pub struct Reader<'a> {
    lexer: Lexer<'a>,
    current: Option<Token>,
    next: Option<Token>,
    location: usize,
}

impl<'a> Reader<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Result<Self> {
        let current = lexer.read()?;

        let mut reader = Reader {
            lexer,
            current: Some(current),
            next: None,
            location: 0,
        };

        reader.next = reader.next_if_exists()?;

        Ok(reader)
    }

    pub fn location(&self) -> usize {
        self.location
    }

    pub fn current(&self) -> Result<&Token> {
        Ok(self.current.as_ref().unwrap())
    }

    /// Consumes the current token.
    /// TODO error handling
    pub fn consume(&mut self) -> Result<Token> {
        let mut next = self.next_if_exists()?;
        mem::swap(&mut next, &mut self.next);

        let mut current = next;
        mem::swap(&mut current, &mut self.current);

        if let Some(t) = &current {
            self.location = t.location.end;
        }

        Ok(current.unwrap()) // TODO end of stream error
    }

    fn next_if_exists(&mut self) -> Result<Option<Token>> {
        match self.lexer.read() {
            Ok(token) => Ok(Some(token)),
            Err(err) => {
                if err.kind() == &EndOfFile {
                    Ok(None)
                } else {
                    Err(err)?
                }
            }
        }
    }
}
