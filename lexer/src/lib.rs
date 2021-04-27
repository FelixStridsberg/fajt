extern crate macros;

mod code_point;
pub mod error;
mod reader;

#[macro_use]
pub mod token;

use crate::code_point::CodePoint;
use crate::error::Error;
use crate::error::ErrorKind::EndOfFile;
use crate::reader::Reader;
use crate::token::Base::{Binary, Decimal, Hex, Octal};
use crate::token::Token;
use crate::token::TokenValue;

/// Consume code points from lexer to produce data.
///
/// Consume and produce syntax:
/// ```ignore
/// produce!(self, 1, punct!("/"))   // Consumes 1, produces token `/`
/// produce!(self, 2, punct!("/="))  // Consumes 2, produces token `/=`
/// produce!(self, 3, punct!("**=")) // Consumes 3, produces token `**=`
/// ```
///
/// Conditional produce syntax:
/// ```ignore
/// // If a peek at next code point matches '&' consume 2 and produce `&&`
/// // ... otherwise consume 1 and produce `&`
/// produce!(self, peek: '&' ? punct!("&&") ; punct!("&"))
/// ```
macro_rules! produce {
    ($self:ident, 1, $produce:expr) => {{
        $self.reader.consume()?;
        Ok($produce)
    }};
    ($self:ident, 2, $produce:expr) => {{
        $self.reader.consume()?;
        $self.reader.consume()?;
        Ok($produce)
    }};
    ($self:ident, 3, $produce:expr) => {{
        $self.reader.consume()?;
        $self.reader.consume()?;
        $self.reader.consume()?;
        Ok($produce)
    }};
    ($self:ident, peek: $peek:literal ? $product1:expr ; $product2:expr) => {{
        if $self.reader.peek() == Some($peek) {
            produce!($self, 2, $product1)
        } else {
            produce!($self, 1, $product2)
        }
    }};
}

type Result<T> = std::result::Result<T, Error>;

pub struct Lexer<'a> {
    reader: Reader<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(data: &'a str) -> Result<Self> {
        let reader = Reader::new(data)?;
        Ok(Lexer { reader })
    }

    pub fn read_all(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();

        loop {
            match self.read() {
                Ok(token) => tokens.push(token),
                Err(e) => {
                    if *e.kind() != EndOfFile {
                        return Err(e);
                    }
                    break;
                }
            }
        }

        Ok(tokens)
    }

    pub fn read(&mut self) -> Result<Token> {
        if self.reader.eof() {
            return Err(Error::of(EndOfFile));
        }

        self.skip_whitespaces()?;

        let current = self.reader.current();

        let start = self.reader.position();
        let value = match current {
            '=' if self.reader.peek() != Some('=') => produce!(self, 1, punct!("=")),
            // <op>=
            '/' | '*' | '%' | '+' | '-' | '|' | '^' | '&' if self.reader.peek() == Some('=') => {
                match current {
                    '/' => produce!(self, 2, punct!("/=")),
                    '*' => produce!(self, 2, punct!("*=")),
                    '%' => produce!(self, 2, punct!("%=")),
                    '+' => produce!(self, 2, punct!("+=")),
                    '-' => produce!(self, 2, punct!("-=")),
                    '|' => produce!(self, 2, punct!("|=")),
                    '^' => produce!(self, 2, punct!("^=")),
                    '&' => produce!(self, 2, punct!("&=")),
                    _ => unreachable!(),
                }
            }
            '^' => produce!(self, 1, punct!("^")),
            '%' => produce!(self, 1, punct!("%")),
            ';' => produce!(self, 1, punct!(";")),
            '{' => produce!(self, 1, punct!("{")),
            '}' => produce!(self, 1, punct!("}")),
            ',' => produce!(self, 1, punct!(",")),
            '&' => produce!(self, peek: '&' ? punct!("&&") ; punct!("&")),
            '|' => produce!(self, peek: '|' ? punct!("||") ; punct!("|")),
            '?' => produce!(self, peek: '?' ? punct!("??") ; punct!("?")),
            '+' => produce!(self, peek: '+' ? punct!("++") ; punct!("+")),
            '-' => produce!(self, peek: '-' ? punct!("--") ; punct!("-")),
            '*' => {
                if self.reader.peek() == Some('*') {
                    self.reader.consume()?;
                    produce!(self, peek: '=' ? punct!("**=") ; punct!("**"))
                } else {
                    produce!(self, 1, punct!("*"))
                }
            }
            // TODO handle comments (//, /*)
            '/' => produce!(self, 1, punct!("/")),
            '<' if self.reader.peek() == Some('<') => {
                self.reader.consume()?;
                produce!(self, peek: '=' ? punct!("<<=") ; punct!("<<"))
            }
            '>' if self.reader.peek() == Some('>') => {
                self.reader.consume()?;
                match self.reader.peek() {
                    Some('>') => {
                        self.reader.consume()?;
                        produce!(self, peek: '=' ? punct!(">>>=") ; punct!(">>>"))
                    }
                    Some('=') => produce!(self, 2, punct!(">>=")),
                    _ => produce!(self, 1, punct!(">>")),
                }
            }
            '0'..='9' => self.read_number_literal(),
            c if c.is_start_of_identifier() => self.read_identifier_or_keyword(),
            c => unimplemented!("Unimplemented: {}", c),
        }?;
        let end = self.reader.position();

        Ok(Token::new(value, (start, end)))
    }

    fn read_identifier_or_keyword(&mut self) -> Result<TokenValue> {
        let word = self
            .reader
            .read_until(|c| char::is_part_of_identifier(&c))?;
        let value = if let Ok(keyword) = word.parse() {
            TokenValue::Keyword(keyword)
        } else {
            TokenValue::Identifier(word)
        };

        Ok(value)
    }

    fn read_number_literal(&mut self) -> Result<TokenValue> {
        let current = self.reader.current();
        let (base, number) = match self.reader.peek() {
            Some('x') | Some('X') if current == '0' => {
                (Hex, self.read_number(16, |c| c.is_ascii_hexdigit())?)
            }
            Some('o') | Some('O') if current == '0' => {
                (Octal, self.read_number(8, |c| ('0'..='7').contains(&c))?)
            }
            Some('b') | Some('B') if current == '0' => {
                (Binary, self.read_number(2, |c| c == '0' || c == '1')?)
            }
            _ => (Decimal, self.read_number(10, char::is_numeric)?),
        };

        Ok(literal!(number, base, number))
    }

    fn read_number(&mut self, base: u32, check: fn(char) -> bool) -> Result<i64> {
        // All but base 10 have 2 char prefix: 0b, 0o, 0x
        if base != 10 {
            self.reader.consume()?;
            self.reader.consume()?;
        }

        let number_str = self.reader.read_until(check)?;

        // The check must be strict enough for a safe unwrap here
        Ok(i64::from_str_radix(&number_str, base).unwrap())
    }

    fn skip_whitespaces(&mut self) -> Result<usize> {
        let mut count = 0;

        while self.reader.current().is_ecma_whitespace() {
            count += 1;
            self.reader.next()?;
        }

        Ok(count)
    }
}
