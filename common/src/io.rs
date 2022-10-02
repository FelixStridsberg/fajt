use std::fmt::Debug;
use std::io::Seek;
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

    fn read_next(&mut self) -> Result<(usize, T), Self::Error> {
        match self.next() {
            Ok(Some(item)) => Ok(item),
            Ok(None) => Err(Error::EndOfStream),
            Err(error) => Err(Error::ReaderError(error)),
        }
    }
}

// It's not necessarily logic that this trait depends on Seek, it should really be two traits, but
// the implementation signature is growing too much, will fix that when trait aliases exists in
// stable.
pub trait ReReadWithState<T>: Seek {
    type Error;
    type State;

    /// Rewind reader to before a previously read item. `item` must be previously read or a panic is
    /// likely to occur.
    fn rewind_before(&mut self, item: &T);

    /// Re-read the last 2 items with another state. If no change the same two items will be
    /// returned. If result change the first returned item is the new item and second is either
    /// the second parameter of input or None if both in input were consumed.
    fn read_with_state(
        &mut self,
        state: Self::State,
    ) -> std::result::Result<Option<(usize, T)>, Self::Error>;
}

/// The peek reader is always one step ahead to enable peeking.
pub struct PeekReader<T, I>
where
    I: PeekRead<T>,
{
    inner: I,
    current: Option<(usize, T)>,
    next: Result<(usize, T), I::Error>,
    position: usize,
    offset: usize,
}

impl<T: Debug, I, E: Debug> PeekReader<T, I>
where
    I: ReReadWithState<T, Error = E>,
    I: PeekRead<T, Error = E>,
{
    pub fn rewind_to(&mut self, item: &T) -> Result<(), E> {
        self.inner.rewind_before(item);
        self.current = self.inner.next()?;
        self.next = self.inner.read_next();
        Ok(())
    }

    /// Re-read `current` token with a specific state.
    pub fn reread_with_state(&mut self, state: <I as ReReadWithState<T>>::State) -> Result<(), E> {
        if let Some((_, token)) = self.current.as_ref() {
            self.inner.rewind_before(token);

            self.current = self.inner.read_with_state(state)?;
            self.next = self.inner.read_next();
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
    pub fn new(inner: I) -> Result<Self, I::Error> {
        Self::with_offset(inner, 0)
    }

    pub fn with_offset(mut inner: I, offset: usize) -> Result<Self, I::Error> {
        let current = inner.next()?;
        let next = inner.read_next();

        Ok(PeekReader {
            inner,
            current,
            next,
            position: offset,
            offset,
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
    pub fn peek(&self) -> std::result::Result<&T, &Error<I::Error>> {
        self.next.as_ref().map(|(_, t)| t)
    }

    /// Returns the current item and reads a new one from the inner reader.
    /// Consuming passed the end of stream results in EndOfStream error.
    /// Any errors from the inner reader while reading will also result in an error.
    pub fn consume(&mut self) -> Result<T, I::Error> {
        let mut next = self.inner.read_next();
        mem::swap(&mut next, &mut self.next);

        let mut current = next.ok();
        mem::swap(&mut current, &mut self.current);

        if let Some((position, item)) = current {
            self.position = position + self.offset;
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
