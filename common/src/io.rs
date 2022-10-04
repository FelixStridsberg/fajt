use std::fmt::Debug;
use std::io::Seek;
use std::mem;

pub mod char_reader;

pub trait PeekRead<T> {
    type Error;

    /// Returns next item if exists, otherwise `None`.
    /// The item is returned in a tuple with the end position as first element.
    fn next(&mut self) -> Result<(usize, T), Self::Error>;
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
    fn read_with_state(&mut self, state: Self::State) -> Result<(usize, T), Self::Error>;
}

/// The peek reader is always one step ahead to enable peeking.
pub struct PeekReader<T, I>
where
    I: PeekRead<T>,
{
    inner: I,
    current: Result<(usize, T), I::Error>,
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
        self.current = self.inner.next();
        self.next = self.inner.next();
        Ok(())
    }

    /// Re-read `current` token with a specific state.
    pub fn reread_with_state(&mut self, state: <I as ReReadWithState<T>>::State) -> Result<(), E> {
        if let Ok((_, token)) = self.current.as_ref() {
            self.inner.rewind_before(token);

            self.current = self.inner.read_with_state(state);
            self.next = self.inner.next();
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
        let current = inner.next();
        let next = inner.next();

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
    pub fn current(&self) -> Result<&T, &I::Error> {
        self.current.as_ref().map(|(_, t)| t)
    }

    /// Peek at the item that will become current after next consume.
    pub fn peek(&self) -> Result<&T, &I::Error> {
        self.next.as_ref().map(|(_, t)| t)
    }

    /// Returns the current item and reads a new one from the inner reader.
    /// Consuming passed the end of stream results in EndOfStream error.
    /// Any errors from the inner reader while reading will also result in an error.
    pub fn consume(&mut self) -> Result<T, I::Error> {
        let mut next = self.inner.next();
        mem::swap(&mut next, &mut self.next);

        let mut current = next;
        mem::swap(&mut current, &mut self.current);

        match current {
            Ok((position, item)) => {
                self.position = position + self.offset;
                Ok(item)
            }
            Err(error) => Err(error),
        }
    }
}
