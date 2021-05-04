use crate::error::Error;
use crate::error::ErrorKind::EndOfFile;
use std::mem;
use std::str::CharIndices;

type Result<T> = std::result::Result<T, Error>;

pub struct Reader<'a> {
    iter: CharIndices<'a>,
    current: Option<(usize, char)>,
    next: Option<(usize, char)>,
    position: usize,
    end_of_file: bool,
}

impl<'a> Reader<'a> {
    pub fn new(input: &'a str) -> Result<Self> {
        let mut iter = input.char_indices();
        let current = iter.next();
        let next = iter.next();

        Ok(Reader {
            iter,
            current,
            next,
            position: 0,
            end_of_file: false,
        })
    }

    /// Current code point position.
    pub fn position(&self) -> usize {
        self.position
    }

    /// Returns current char.
    /// TODO Calling this function after end of file results in EndOfFile error.
    pub fn current(&mut self) -> Result<char> {
        if let Some((_, current)) = self.current {
            Ok(current)
        } else {
            Err(Error::of(EndOfFile))
        }
    }

    /// Peek at the code point that will become current after next consume.
    pub fn peek(&self) -> Option<char> {
        self.next.map(|(_, c)| c)
    }

    /// Consumes the current character.
    pub fn consume(&mut self) -> Result<char> {
        let mut next = self.iter.next();
        mem::swap(&mut next, &mut self.next);

        let mut current = next;
        mem::swap(&mut current, &mut self.current);

        if let Some((pos, a)) = current {
            self.position = pos + a.len_utf16();
        }

        if let Some((_, c)) = current {
            Ok(c) // TODO use Into trait to (usize, char) and harmonize with other reader?
        } else {
            Err(Error::of(EndOfFile))
        }
    }

    pub fn read_until(&mut self, check: fn(char) -> bool) -> Result<String> {
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
                Err(e) => {
                    assert_eq!(*e.kind(), EndOfFile);
                    break;
                }
            }
        }

        Ok(result)
    }
}
