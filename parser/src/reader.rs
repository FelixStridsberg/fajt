use crate::error::ErrorKind::EndOfStream;
use crate::error::{Error, Result};
use core::mem;
use fajt_lexer::error::ErrorKind::EndOfFile;
use fajt_lexer::token::Token;
use fajt_lexer::Lexer;

pub struct Reader<'a> {
    lexer: Lexer<'a>,
    current: Option<Token>,
    next: Option<Token>,
    position: usize,
}

impl<'a> Reader<'a> {
    /// Returns an instance of a Reader.
    /// Returns error if lexer returns error (other than eof) when reading first 2 tokens.
    pub fn new(mut lexer: Lexer<'a>) -> Result<Self> {
        let mut reader = Reader {
            lexer,
            current: None,
            next: None,
            position: 0,
        };

        reader.current = reader.next_if_exists()?;
        reader.next = reader.next_if_exists()?;

        Ok(reader)
    }

    /// Code point position of the current token, or position of end of file if there are no tokens
    /// left.
    pub fn position(&self) -> usize {
        self.position
    }

    /// Returns reference to the current token.
    /// Calling this function after the stream has been fully consumed results in EndOfStream error.
    pub fn current(&self) -> Result<&Token> {
        if let Some(current) = self.current.as_ref() {
            Ok(current)
        } else {
            Err(Error::of(EndOfStream))
        }
    }

    /// Peek at the token that will become current after next consume.
    pub fn peek(&self) -> Option<&Token> {
        self.next.as_ref()
    }

    /// Returns the current token and reads a new one from the lexer.
    /// Reading passed the end of lexer stream results in EndOfStream
    /// Any errors in the lexer while reading will also result in an error.
    pub fn consume(&mut self) -> Result<Token> {
        let mut next = self.next_if_exists()?;
        mem::swap(&mut next, &mut self.next);

        let mut current = next;
        mem::swap(&mut current, &mut self.current);

        if let Some(t) = &current {
            self.position = t.span.end;
        }

        if matches!(current, Some(_)) {
            Ok(current.unwrap())
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
