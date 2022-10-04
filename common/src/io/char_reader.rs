use crate::io::char_reader::Error::EndOfStream;
use crate::io::{PeekRead, PeekReader};
use std::fmt::Debug;
use std::str::CharIndices;

#[derive(Debug)]
pub enum Error {
    EndOfStream,
}

impl<I> PeekReader<char, I>
where
    I: PeekRead<char, Error = Error>,
{
    /// Read a string until `check` callback returns false or the end of the stream is reached.
    pub fn read_while(&mut self, check: fn(&char) -> bool) -> Result<String, I::Error> {
        let mut result = String::new();

        while let Ok(c) = self.current() {
            if check(c) {
                result.push(self.consume()?);
            } else {
                break;
            }
        }

        Ok(result)
    }
}

impl<'a> PeekRead<char> for CharIndices<'a> {
    type Error = Error;

    fn next(&mut self) -> Result<(usize, char), Self::Error> {
        Iterator::next(self)
            .map(|(pos, c)| (pos + c.len_utf16(), c))
            .ok_or(EndOfStream)
    }
}
