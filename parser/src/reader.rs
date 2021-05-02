use crate::error::ErrorKind::EndOfStream;
use crate::error::{Error, Result};
use core::mem;
use fajt_lexer::error::ErrorKind::EndOfFile;
use fajt_lexer::token::Token;
use fajt_lexer::Lexer;

pub struct Reader<'a> {
    lexer: Lexer<'a>,
    current: Token,
    next: Option<Token>,
    end: bool,
}

impl<'a> Reader<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Result<Self> {
        let current = lexer.read()?;

        let mut reader = Reader {
            lexer,
            current,
            next: None,
            end: false,
        };

        reader.next = reader.next_if_exists()?;

        Ok(reader)
    }

    pub fn has_next(&self) -> bool {
        !self.end
    }

    pub fn current(&self) -> &Token {
        &self.current
    }

    pub fn peek(&self) -> Option<&Token> {
        self.next.as_ref()
    }

    /// Consumes the current token.
    /// TODO revisit if this is the correct approach. Is only peek and next better?
    /// TODO if this should be used, should it return ownership of Token?
    pub fn consume(&mut self) -> Result<()> {
        let _ = self.next();
        Ok(())
    }

    pub fn next(&mut self) -> Result<&Token> {
        let mut next = self.next_if_exists()?;
        mem::swap(&mut next, &mut self.next);

        if let Some(next) = next {
            self.current = next;
            Ok(&self.current)
        } else {
            Err(Error::of(EndOfStream))
        }
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
