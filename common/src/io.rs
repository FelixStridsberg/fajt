use std::fmt::Debug;
use std::mem;
use std::str::CharIndices;

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

    /// Returns next item if exists, otherwise `None`.
    /// The item is returned in a tuple with the end position as first element.
    fn next(&mut self) -> std::result::Result<Option<(usize, T)>, Self::Error>;
}

pub trait ReRead<T> {
    type Error;
    type State;

    /// Re-read the last 2 items with another state. If no change the same two items will be
    /// returned. If result change the first returned item is the new item and second is either
    /// the second parameter of input or None if both in input were consumed.
    fn reread_with_state(
        &mut self,
        last: [Option<(usize, T)>; 2],
        state: Self::State,
    ) -> std::result::Result<[Option<(usize, T)>; 2], Self::Error>;
}

/// The peek reader is always one step ahead to enable peeking.
pub struct PeekReader<T, I> {
    inner: I,
    current: Option<(usize, T)>,
    next: Option<(usize, T)>,
    position: usize,
}

impl<T, I, E: Debug> PeekReader<T, I>
where
    I: ReRead<T, Error = E>,
    I: PeekRead<T, Error = E>,
{
    pub fn reread_with_state(&mut self, state: <I as ReRead<T>>::State) -> Result<(), E> {
        let had_next = self.next.is_some();

        let current = mem::take(&mut self.current);
        let next = mem::take(&mut self.next);

        let [new_current, new_next] = self.inner.reread_with_state([current, next], state)?;

        self.current = new_current;

        // If the next was consumed, re-populate next position.
        if new_next.is_none() && had_next {
            self.next = self.inner.next()?;
        } else {
            self.next = new_next;
        }

        Ok(())
    }
}

impl<T, I> PeekReader<T, I>
where
    I: PeekRead<T>,
    I::Error: Debug,
{
    /// Returns an instance of a PeekReader.
    /// Returns error if inner reader returns error when reading first 2 items.
    pub fn new(mut inner: I) -> Result<Self, I::Error> {
        let current = inner.next()?;
        let next = inner.next()?;

        Ok(PeekReader {
            inner,
            current,
            next,
            position: 0,
        })
    }

    /// Current position of reader, i.e. end position of last consumed item.
    pub fn position(&self) -> usize {
        self.position
    }

    /// Returns reference to the current item.
    /// Calling this function after the stream has been fully consumed results in EndOfStream error.
    pub fn current(&self) -> Result<&T, I::Error> {
        if let Some((_, current)) = self.current.as_ref() {
            Ok(current)
        } else {
            Err(Error::EndOfStream)
        }
    }

    /// Returns true if the next token would result in `Error::EndOfStream`.
    pub fn is_end(&self) -> bool {
        self.current.is_none()
    }

    /// Peek at the item that will become current after next consume.
    pub fn peek(&self) -> Option<&T> {
        self.next.as_ref().map(|(_, item)| item)
    }

    /// Returns the current item and reads a new one from the inner reader.
    /// Consuming passed the end of stream results in EndOfStream error.
    /// Any errors from the inner reader while reading will also result in an error.
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

impl<I> PeekReader<char, I>
where
    I: PeekRead<char>,
    I::Error: Debug,
{
    /// Read a string until `check` callback returns false or the end of the stream is reached.
    pub fn read_while(&mut self, check: fn(&char) -> bool) -> Result<String, I::Error> {
        let mut result = String::new();

        loop {
            match self.current() {
                Ok(c) => {
                    if check(c) {
                        result.push(self.consume()?);
                    } else {
                        break;
                    }
                }
                Err(Error::EndOfStream) => {
                    break;
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        Ok(result)
    }
}

impl<'a> PeekRead<char> for CharIndices<'a> {
    type Error = ();

    fn next(&mut self) -> std::result::Result<Option<(usize, char)>, Self::Error> {
        Ok(Iterator::next(self).map(|(pos, c)| (pos + c.len_utf16(), c)))
    }
}
