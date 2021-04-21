use crate::error::Error;
use crate::error::ErrorKind::EndOfFile;
use crate::token::FilePosition;
use std::str::CharIndices;

type Result<T> = std::result::Result<T, Error>;

pub struct Reader<'a> {
    input: &'a str,
    iter: CharIndices<'a>,
    current: (usize, char),
    next: Option<(usize, char)>,
    line: usize,
    column: usize,
    end_of_file: bool,
}

impl<'a> Reader<'a> {
    pub fn new(input: &'a str) -> Result<Self> {
        let mut iter = input.char_indices();
        let current = iter.next().ok_or(Error::of(EndOfFile))?;
        let next = iter.next();

        Ok(Reader {
            input,
            iter,
            current,
            next,
            line: 0,
            column: 0,
            end_of_file: false,
        })
    }

    pub fn eof(&self) -> bool {
        return self.end_of_file;
    }

    pub fn position(&self) -> FilePosition {
        FilePosition {
            line: self.line,
            column: self.column,
        }
    }

    pub fn current(&mut self) -> char {
        self.current.1
    }

    pub fn peek(&self) -> Option<char> {
        self.next.map(|(_, c)| c)
    }

    pub fn next(&mut self) -> Result<char> {
        // TODO self.line
        if !self.end_of_file {
            self.column += 1;
        }

        if let Some(next) = self.next {
            self.current = next;
            self.next = self.iter.next();

            Ok(self.current.1)
        } else {
            self.end_of_file = true;
            return Err(Error::of(EndOfFile));
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
}
