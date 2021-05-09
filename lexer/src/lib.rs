extern crate fajt_macros;

mod code_point;
pub mod error;

#[macro_use]
pub mod token;

use crate::code_point::CodePoint;
use crate::error::Error;
use crate::error::ErrorKind::EndOfFile;
use crate::token::Base::{Binary, Decimal, Hex, Octal};
use crate::token::Literal;
use crate::token::Token;
use crate::token::TokenValue;
use fajt_common::io::{PeekRead, PeekReader};
use std::str::CharIndices;

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
        if $self.reader.peek() == Some(&$peek) {
            produce!($self, 2, $product1)
        } else {
            produce!($self, 1, $product2)
        }
    }};
}

type Result<T> = std::result::Result<T, Error>;

pub struct Lexer<'a> {
    reader: PeekReader<char, CharIndices<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(data: &'a str) -> Result<Self> {
        let reader = PeekReader::new(data.char_indices())?;
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
        self.skip_whitespaces()?;

        let current = self.reader.current()?;

        let start = self.reader.position();
        let value = match current {
            // <op>=
            '/' | '*' | '%' | '+' | '-' | '|' | '^' | '&' | '<' | '>' | '='
                if self.reader.peek() == Some(&'=') =>
            {
                match current {
                    '/' => produce!(self, 2, punct!("/=")),
                    '*' => produce!(self, 2, punct!("*=")),
                    '%' => produce!(self, 2, punct!("%=")),
                    '+' => produce!(self, 2, punct!("+=")),
                    '-' => produce!(self, 2, punct!("-=")),
                    '|' => produce!(self, 2, punct!("|=")),
                    '^' => produce!(self, 2, punct!("^=")),
                    '&' => produce!(self, 2, punct!("&=")),
                    '<' => produce!(self, 2, punct!("<=")),
                    '>' => produce!(self, 2, punct!(">=")),
                    '=' => {
                        self.reader.consume()?;
                        produce!(self, peek: '=' ? punct!("===") ; punct!("=="))
                    }
                    _ => unreachable!(),
                }
            }
            '<' if self.reader.peek() == Some(&'<') => {
                self.reader.consume()?;
                produce!(self, peek: '=' ? punct!("<<=") ; punct!("<<"))
            }
            '>' if self.reader.peek() == Some(&'>') => {
                self.reader.consume()?;
                match self.reader.peek() {
                    Some(&'>') => {
                        self.reader.consume()?;
                        produce!(self, peek: '=' ? punct!(">>>=") ; punct!(">>>"))
                    }
                    Some(&'=') => produce!(self, 2, punct!(">>=")),
                    _ => produce!(self, 1, punct!(">>")),
                }
            }
            '!' if self.reader.peek() == Some(&'=') => {
                self.reader.consume()?;
                produce!(self, peek: '=' ? punct!("!==") ; punct!("!="))
            }
            '=' => produce!(self, peek: '>' ? punct!("=>") ; punct!("=")),
            '<' => produce!(self, 1, punct!("<")),
            '>' => produce!(self, 1, punct!(">")),
            '^' => produce!(self, 1, punct!("^")),
            '%' => produce!(self, 1, punct!("%")),
            ';' => produce!(self, 1, punct!(";")),
            '{' => produce!(self, 1, punct!("{")),
            '}' => produce!(self, 1, punct!("}")),
            ',' => produce!(self, 1, punct!(",")),
            '[' => produce!(self, 1, punct!("[")),
            ']' => produce!(self, 1, punct!("]")),
            '(' => produce!(self, 1, punct!("(")),
            ')' => produce!(self, 1, punct!(")")),
            '~' => produce!(self, 1, punct!("~")),
            ':' => produce!(self, 1, punct!(":")),
            '!' => produce!(self, 1, punct!("!")),
            '&' => produce!(self, peek: '&' ? punct!("&&") ; punct!("&")),
            '|' => produce!(self, peek: '|' ? punct!("||") ; punct!("|")),
            '?' => produce!(self, peek: '?' ? punct!("??") ; punct!("?")),
            '+' => produce!(self, peek: '+' ? punct!("++") ; punct!("+")),
            '-' => produce!(self, peek: '-' ? punct!("--") ; punct!("-")),
            '*' => {
                if self.reader.peek() == Some(&'*') {
                    self.reader.consume()?;
                    produce!(self, peek: '=' ? punct!("**=") ; punct!("**"))
                } else {
                    produce!(self, 1, punct!("*"))
                }
            }
            '.' => {
                if self.reader.peek() == Some(&'.') {
                    self.reader.consume()?;
                    if self.reader.peek() == Some(&'.') {
                        produce!(self, 2, punct!("..."))
                    } else {
                        todo!("Syntax error, '..' is not a valid punctuator.")
                    }
                } else {
                    produce!(self, 1, punct!("."))
                }
            }
            // TODO handle comments (//, /*)
            '/' => produce!(self, 1, punct!("/")),
            '0'..='9' => self.read_number_literal(),
            '"' | '\'' => self.read_string_literal(),
            c if c.is_start_of_identifier() => self.read_identifier_or_keyword(),
            c => unimplemented!("Lexer did not recognize code point '{}'.", c),
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

    fn read_string_literal(&mut self) -> Result<TokenValue> {
        let delimiter = self.reader.consume()?;
        debug_assert!(delimiter == '"' || delimiter == '\'');

        let mut result = String::new();
        let mut escape = false;
        loop {
            let c = self.reader.consume()?;
            if !escape && c == delimiter {
                break;
            }

            escape = c == '\\' && !escape;
            if !escape {
                result.push(c);
            }
        }

        Ok(TokenValue::Literal(Literal::String(result, delimiter)))
    }

    fn read_number_literal(&mut self) -> Result<TokenValue> {
        let current = self.reader.current()?;
        let (base, number) = match self.reader.peek() {
            Some(&'x') | Some(&'X') if current == &'0' => {
                (Hex, self.read_number(16, char::is_ascii_hexdigit)?)
            }
            Some(&'o') | Some(&'O') if current == &'0' => {
                (Octal, self.read_number(8, |c| ('0'..='7').contains(&c))?)
            }
            Some(&'b') | Some(&'B') if current == &'0' => {
                (Binary, self.read_number(2, |c| c == &'0' || c == &'1')?)
            }
            _ => {
                return self.read_integer_or_decimal();
            }
        };

        Ok(literal!(number, base, number))
    }

    fn read_integer_or_decimal(&mut self) -> Result<TokenValue> {
        let integral = self.read_number(10, |c| c.is_numeric())?;
        if let Ok(&'.') = self.reader.current() {
            self.reader.consume()?;
            let fraction = self.read_number(10, |c| c.is_numeric())?;
            let digits = (fraction as f64).log10().floor() + 1.0;
            let float = integral as f64 + (fraction as f64 / (digits * 10.0));
            Ok(literal!(decimal, float))
        } else {
            Ok(literal!(integer, integral))
        }
    }

    fn read_number(&mut self, base: u32, check: fn(&char) -> bool) -> Result<i64> {
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

        while self.reader.current()?.is_ecma_whitespace() {
            count += 1;
            self.reader.consume()?;
        }

        Ok(count)
    }
}

impl PeekRead<Token> for Lexer<'_> {
    type Error = error::Error;

    fn next(&mut self) -> std::result::Result<Option<(usize, Token)>, Error> {
        match self.read() {
            Ok(t) => Ok(Some((t.span.end, t))),
            Err(e) => {
                if *e.kind() == EndOfFile {
                    return Ok(None);
                }
                Err(e)
            }
        }
    }
}
