#[macro_use]
extern crate bitflags;

extern crate fajt_macros;

mod code_point;
pub mod error;

#[macro_use]
pub mod token;

use crate::code_point::CodePoint;
use crate::error::Error;
use crate::error::ErrorKind::{EndOfStream, InvalidOrUnexpectedToken};
use crate::token::Token;
use crate::token::TokenValue;
use fajt_ast::Base::{Binary, Hex, Octal};
use fajt_ast::{LitString, LitTemplate, Literal, Span, TemplatePart};
use fajt_common::io::{PeekRead, PeekReader, ReReadWithState};
use std::io::{Seek, SeekFrom};
use std::mem;
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
    data: &'a str,
    state: LexerState,
    reader: PeekReader<char, CharIndices<'a>>,
    override_first_on_line: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(data: &'a str) -> Result<Self> {
        let reader = PeekReader::new(data.char_indices())?;
        Ok(Lexer {
            data,
            state: LexerState::default(),
            reader,
            override_first_on_line: false,
        })
    }

    pub fn read_all(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();

        loop {
            match self.read() {
                Ok(token) => tokens.push(token),
                Err(e) => {
                    if *e.kind() != EndOfStream {
                        return Err(e);
                    }
                    break;
                }
            }
        }

        Ok(tokens)
    }

    pub fn read(&mut self) -> Result<Token> {
        if self.state.inside_template {
            return self.read_template_literal_middle_or_tail();
        }

        let new_line = self.skip_comments_and_white_spaces()? | self.override_first_on_line;
        let current = self.reader.current()?;

        self.override_first_on_line = false;

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
                        return Err(Error::invalid_or_unexpected_token(error_token));
                    }
                } else {
                    produce!(self, 1, punct!("."))
                }
            }
            '/' if self.state.regex_allowed => self.read_regexp_literal(),
            '/' => produce!(self, 1, punct!("/")),
            '0'..='9' => self.read_number_literal(),
            '"' | '\'' => self.read_string_literal(),
            '`' => self.read_template_literal_head(),
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

    fn read_template_literal_head(&mut self) -> Result<TokenValue> {
        let delimiter = self.reader.consume()?;
        debug_assert_eq!(delimiter, '`');

        let (value, ending) = self.read_until_end_of_template_literal_part()?;

        if ending == "${" {
            Ok(TokenValue::TemplateHead(value))
        } else {
            // Non substitution template literal
            Ok(TokenValue::Literal(Literal::Template(LitTemplate {
                parts: vec![TemplatePart::String(value)],
            })))
        }
    }

    fn read_template_literal_middle_or_tail(&mut self) -> Result<Token> {
        let span_start = self.reader.position();
        let start = self.reader.consume()?;
        debug_assert_eq!(start, '}');

        let (value, ending) = self.read_until_end_of_template_literal_part()?;

        let value = if ending == "${" {
            TokenValue::TemplateMiddle(value)
        } else {
            TokenValue::TemplateTail(value)
        };

        let span = Span::new(span_start, self.reader.position());
        Ok(Token {
            span,
            first_on_line: false,
            value,
        })
    }

    /// Returns literal string and what ended it.
    fn read_until_end_of_template_literal_part(&mut self) -> Result<(String, &'static str)> {
        let mut result = String::new();

        let mut escape = false;
        loop {
            let c = self.reader.consume()?;
            if !escape && c == '`' {
                return Ok((result, "`"));
            }

            if !escape && c == '$' && self.reader.current()? == &'{' {
                self.reader.consume()?;
                return Ok((result, "${"));
            }

            escape = c == '\\' && !escape;
            if !escape {
                result.push(c);
            }
        }
    }

    fn read_regexp_literal(&mut self) -> Result<TokenValue> {
        let mut result = String::new();
        let regexp_start = self.reader.consume()?;
        debug_assert_eq!(regexp_start, '/');
        result.push(regexp_start);

        self.read_until_not_escaped('/', &mut result)?;
        result.push('/');

        let flags = self.reader.read_while(char::is_part_of_identifier)?;
        result.push_str(&flags);

        Ok(TokenValue::Literal(Literal::Regexp(result)))
    }

    /// Consumes from reader and push to `result` until an unescaped `delimiter` is reached.
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

#[derive(Default)]
pub struct LexerState {
    regex_allowed: bool,
    inside_template: bool,
}

impl LexerState {
    pub fn regex_allowed() -> Self {
        LexerState {
            regex_allowed: true,
            ..Self::default()
        }
    }

    pub fn inside_template() -> Self {
        LexerState {
            inside_template: true,
            ..Self::default()
        }
    }
}

impl Seek for Lexer<'_> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        let pos = match pos {
            SeekFrom::Start(offset) => offset,
            SeekFrom::End(offset) => (self.data.len() as i64 + offset) as u64,
            SeekFrom::Current(offset) => (self.reader.position() as i64 + offset) as u64,
        };

        let offset = pos as usize;
        self.reader = PeekReader::with_offset(self.data[offset..].char_indices(), offset).unwrap();

        Ok(pos)
    }
}

impl ReReadWithState<Token> for Lexer<'_> {
    type Error = error::Error;
    type State = LexerState;

    /// Rewind reader to before `token`, `token` must have been previously read from this lexer.
    fn rewind_before(&mut self, token: &Token) {
        self.seek(SeekFrom::Start(token.span.start as u64)).unwrap();
        self.override_first_on_line = token.first_on_line;
    }

    /// Read one token with a different lexer state.
    ///
    /// The ECMAScript lexer have different contexts that cannot be determined from the lexical
    /// grammar alone.
    ///
    /// # Example:
    /// ```js
    /// a = b
    /// /c/g
    /// ```
    ///
    /// Would by default result in:
    /// ```txt
    /// Identifier("a")
    /// Punctuator("=")
    /// Identifier("b")
    /// Punctuator("/")
    /// Identifier("c")
    /// Punctuator("/")
    /// Punctuator("g")
    /// ```
    ///
    /// but when the parser encounters a `/` in a context where regexp is allowed it will rewind
    /// and re-read with regexp state set in the lexer, with the following result:
    /// ```txt
    /// Identifier("a")
    /// Punctuator("=")
    /// Identifier("b")
    /// Literal(Regexp("/c/g"))
    /// ```
    fn read_with_state(
        &mut self,
        mut state: LexerState,
    ) -> std::result::Result<Option<(usize, Token)>, Self::Error> {
        mem::swap(&mut state, &mut self.state);
        let result = self.next()?;
        mem::swap(&mut self.state, &mut state);

        Ok(result)
    }
}

impl PeekRead<Token> for Lexer<'_> {
    type Error = error::Error;

    fn next(&mut self) -> std::result::Result<Option<(usize, Token)>, Error> {
        match self.read() {
            Ok(t) => Ok(Some((t.span.end, t))),
            Err(e) => {
                if *e.kind() == EndOfStream {
                    return Ok(None);
                }
                Err(e)
            }
        }
    }
}
