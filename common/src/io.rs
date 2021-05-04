use std::fmt::Debug;
use std::mem;

#[derive(Debug)]
pub enum Error<E> {
    EndOfStream,
    ReaderError(E),
}

impl<E> From<E> for Error<E> {
    fn from(error: E) -> Self {
        Self::ReaderError(error)
    }
}

type Result<T, E> = std::result::Result<T, Error<E>>;

pub trait PeekRead<T> {
    type Error;

    fn next(&mut self) -> std::result::Result<Option<(usize, T)>, Self::Error>;
}

pub struct PeekReader<T, I> {
    inner: I,
    current: Option<(usize, T)>,
    next: Option<(usize, T)>,
    position: usize,
}

impl<T, I> PeekReader<T, I>
where
    I: PeekRead<T>,
    I::Error: Debug,
{
    /// Returns an instance of a Reader.
    /// Returns error if lexer returns error (other than eof) when reading first 2 tokens.
    pub fn new(mut inner: I) -> Self {
        let current = inner.next().unwrap();
        let next = inner.next().unwrap();

        PeekReader {
            inner,
            current,
            next,
            position: 0,
        }
    }

    /// Code point position of the current token, or position of end of file if there are no tokens
    /// left.
    pub fn position(&self) -> usize {
        self.position
    }

    /// Returns reference to the current token.
    /// Calling this function after the stream has been fully consumed results in EndOfStream error.
    pub fn current(&self) -> Result<&T, I::Error> {
        if let Some((_, current)) = self.current.as_ref() {
            Ok(current)
        } else {
            Err(Error::EndOfStream)
        }
    }

    /// Peek at the token that will become current after next consume.
    pub fn peek(&self) -> Option<&T> {
        self.next.as_ref().map(|(_, item)| item)
    }

    /// Returns the current token and reads a new one from the lexer.
    /// Reading passed the end of lexer stream results in EndOfStream
    /// Any errors in the lexer while reading will also result in an error.
    pub fn consume(&mut self) -> Result<T, I::Error> {
        let mut next = self.inner.next()?;
        mem::swap(&mut next, &mut self.next);

        let mut current = next;
        mem::swap(&mut current, &mut self.current);

        if let Some((position, item)) = current {
            self.position = position;
            Ok(item)
        } else {
            Err(Error::EndOfStream)
        }
    }
}
