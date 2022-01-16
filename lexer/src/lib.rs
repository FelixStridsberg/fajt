#[macro_use]
extern crate bitflags;

extern crate fajt_macros;

mod code_point;
pub mod error;

#[macro_use]
pub mod token;

use crate::code_point::CodePoint;
use crate::error::Error;
use crate::error::ErrorKind::{EndOfFile, InvalidOrUnexpectedToken};
use crate::token::Token;
use crate::token::TokenValue;
use fajt_ast::Base::{Binary, Hex, Octal};
use fajt_ast::{LitString, Literal, Span};
use fajt_common::io::{PeekRead, PeekReader, ReRead};
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
        let new_line = self.skip_comments_and_white_spaces()?;
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
            '+' => produce!(self, peek: '+' ? punct!("++") ; punct!("+")),
            '-' => produce!(self, peek: '-' ? punct!("--") ; punct!("-")),
            '?' => {
                self.reader.consume()?;
                Ok(match self.reader.current() {
                    Ok(&'.') if !matches!(self.reader.peek(), Some('0'..='9')) => {
                        self.reader.consume()?;
                        punct!("?.")
                    }
                    Ok(&'?') => {
                        self.reader.consume()?;
                        punct!("??")
                    }
                    _ => punct!("?"),
                })
            }
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
                        let end = self.reader.position();
                        let error_token = Token::new(punct!("."), new_line, (start, end));
                        return Err(Error::of(InvalidOrUnexpectedToken(error_token)));
                    }
                } else {
                    produce!(self, 1, punct!("."))
                }
            }
            '/' => produce!(self, 1, punct!("/")),
            '0'..='9' => self.read_number_literal(),
            '"' | '\'' => self.read_string_literal(),
            c if c.is_start_of_identifier() => self.read_identifier_or_keyword(),
            c => unimplemented!("Lexer did not recognize code point '{}'.", c),
        }?;
        let end = self.reader.position();

        Ok(Token::new(value, new_line, (start, end)))
    }

    /// Skips all consecutive comments and white spaces, returns true if a new line was skipped.
    fn skip_comments_and_white_spaces(&mut self) -> Result<bool> {
        let mut skipped_new_line = false;

        loop {
            skipped_new_line = self.skip_whitespaces()? || skipped_new_line;

            match self.reader.current() {
                Ok('/') if self.reader.peek() == Some(&'/') => {
                    self.skip_single_line_comment();
                    // Single line comments never includes the ending new line. If at end of file it
                    // doesn't matter if we lie and say there was a new line.
                    skipped_new_line = true;
                }
                Ok('/') if self.reader.peek() == Some(&'*') => {
                    skipped_new_line = self.skip_multi_line_comment()? || skipped_new_line;
                }
                _ => break,
            }
        }

        skipped_new_line = self.skip_whitespaces()? || skipped_new_line;

        Ok(skipped_new_line)
    }

    fn skip_single_line_comment(&mut self) {
        self.reader.consume().unwrap();
        self.reader.consume().unwrap();

        self.reader
            .read_while(|c| !c.is_ecma_line_terminator())
            .unwrap();

        if self.reader.current().is_ok() {
            self.reader.consume().unwrap(); // Consume trailing new line
        }
    }

    /// Skips multi line comment, returns true if a new line was skipped.
    fn skip_multi_line_comment(&mut self) -> Result<bool> {
        self.reader.consume()?;
        self.reader.consume()?;

        let mut contains_line_break = false;
        let mut content = String::new();
        loop {
            if matches!(self.reader.current(), Ok('*')) && self.reader.peek() == Some(&'/') {
                self.reader.consume()?;
                self.reader.consume()?;
                break;
            }

            let char = self.reader.consume()?;
            if char.is_ecma_line_terminator() {
                contains_line_break = true;
            }
            content.push(char);
        }

        Ok(contains_line_break)
    }

    fn read_identifier_or_keyword(&mut self) -> Result<TokenValue> {
        let word = self.reader.read_while(char::is_part_of_identifier)?;
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

        let mut value = String::new();
        self.read_until_not_escaped(delimiter, &mut value)?;

        Ok(TokenValue::Literal(Literal::String(LitString {
            value,
            delimiter,
        })))
    }

    fn read_rest_of_regex_literal(&mut self, start: &mut String) -> Result<()> {
        let mut result = start;
        self.read_until_not_escaped('/', &mut result)?;
        result.push('/');

        let flags = self.reader.read_while(char::is_part_of_identifier)?;
        result.push_str(&flags);

        Ok(())
    }

    fn read_until_not_escaped(&mut self, delimiter: char, result: &mut String) -> Result<()> {
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

        Ok(())
    }

    fn read_number_literal(&mut self) -> Result<TokenValue> {
        let current = self.reader.current()?;
        let (base, number) = match self.reader.peek() {
            Some(&'x' | &'X') if current == &'0' => {
                (Hex, self.read_number(16, char::is_ascii_hexdigit)?)
            }
            Some(&'o' | &'O') if current == &'0' => {
                (Octal, self.read_number(8, |c| ('0'..='7').contains(c))?)
            }
            Some(&'b' | &'B') if current == &'0' => {
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

        let number_str = self.reader.read_while(check)?;

        // The check must be strict enough for a safe unwrap here
        Ok(i64::from_str_radix(&number_str, base).unwrap())
    }

    fn skip_whitespaces(&mut self) -> Result<bool> {
        let mut line_terminator = self.reader.position() == 0;

        loop {
            if self.reader.current()?.is_ecma_line_terminator() {
                line_terminator = true;
                self.reader.consume()?;
                continue;
            }

            if self.reader.current()?.is_ecma_whitespace() {
                self.reader.consume()?;
                continue;
            }

            break;
        }

        Ok(line_terminator)
    }
}

pub struct LexerState {
    regex_allowed: bool,
}

impl LexerState {
    pub fn regex_allowed() -> Self {
        LexerState {
            regex_allowed: true,
        }
    }
}

impl ReRead<Token> for Lexer<'_> {
    type Error = error::Error;
    type State = LexerState;

    fn reread_with_state(
        &mut self,
        last: [Option<(usize, Token)>; 2],
        state: LexerState,
    ) -> std::result::Result<[Option<(usize, Token)>; 2], Self::Error> {
        if !state.regex_allowed {
            todo!("Reevaluate other than regexps.")
        }

        // TODO cleanup
        if let Some((_, first)) = last[0].as_ref() {
            if token_matches!(first, punct!("/")) {
                let mut result = "/".to_string();

                if let Some((_, last)) = last[1].as_ref() {
                    println!("First {:?}", first);
                    println!("Last {:?}", last);
                    match &last.value {
                        TokenValue::Keyword(_) => {
                            todo!("String form keyword")
                        }
                        TokenValue::Identifier(ident) => {
                            result.push_str(&ident);
                        }
                        TokenValue::Punct(_) => {
                            todo!("String from punctuator")
                        }
                        TokenValue::Literal(_) => {
                            todo!("String from literal")
                        }
                    }
                }

                self.read_rest_of_regex_literal(&mut result)?;

                let span_end = first.span.start + result.len();
                let span = Span::new(first.span.start, span_end);

                let value = TokenValue::Literal(Literal::Regexp(result));
                return Ok([
                    Some((
                        span_end,
                        Token {
                            span,
                            value,
                            first_on_line: first.first_on_line,
                        },
                    )),
                    None,
                ]);
            }
        }

        Ok(last)
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
