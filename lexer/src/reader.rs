use crate::error::Error;
use crate::error::ErrorKind::EndOfFile;
use std::str::CharIndices;

type Result<T> = std::result::Result<T, Error>;

pub struct Reader<'a> {
    iter: CharIndices<'a>,
    current: (usize, char),
    next: Option<(usize, char)>,
    position: usize,
    end_of_file: bool,
}

impl<'a> Reader<'a> {
    pub fn new(input: &'a str) -> Result<Self> {
        let mut iter = input.char_indices();
        let current = iter.next().ok_or_else(|| Error::of(EndOfFile))?;
        let next = iter.next();

        Ok(Reader {
            iter,
            current,
            next,
            position: 0,
            end_of_file: false,
        })
    }

    pub fn eof(&self) -> bool {
        self.end_of_file
    }

    /// Current code point position.
    pub fn position(&self) -> usize {
        self.position
    }

    /// Returns current char.
    /// TODO Calling this function after end of file results in EndOfFile error.
    pub fn current(&mut self) -> char {
        self.current.1
    }

    /// Peek at the code point that will become current after next consume.
    pub fn peek(&self) -> Option<char> {
        self.next.map(|(_, c)| c)
    }

    /// Consumes the current character.
    pub fn consume(&mut self) -> Result<()> {
        if self.end_of_file {
            Err(Error::of(EndOfFile))
        } else {
            let _ = self.next();
            Ok(())
        }
    }

    pub fn read_until(&mut self, check: fn(char) -> bool) -> Result<String> {
        let mut result = String::new();
        result.push(self.current());

        loop {
            match self.next() {
                Ok(c) => {
                    if check(c) {
                        result.push(c);
                    } else {
                        break;
                    }
                }
                Err(e) => {
                    if *e.kind() == EndOfFile {
                        break;
                    } else {
                        return Err(e);
                    }
                }
            }
        }

        Ok(result)
    }

    fn next(&mut self) -> Result<char> {
        if !self.end_of_file {
            self.position += 1;
        }

        if let Some(next) = self.next {
            self.current = next;
            self.next = self.iter.next();

            Ok(self.current.1)
        } else {
            self.end_of_file = true;
            Err(Error::of(EndOfFile))
        }
    }
}
