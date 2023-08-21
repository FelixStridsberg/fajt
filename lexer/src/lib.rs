#[macro_use]
extern crate bitflags;
extern crate fajt_macros;

mod code_point;
pub mod error;

#[macro_use]
pub mod token;
mod regexp;
mod unicode_escape_sequence;

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
        if $self.reader.peek().ok() == Some(&$peek) {
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
    first_on_line: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(data: &'a str) -> Result<Self> {
        let reader = PeekReader::new(data.char_indices())?;
        Ok(Lexer {
            data,
            state: LexerState::default(),
            reader,
            first_on_line: true,
        })
    }

    pub fn set_state(&mut self, state: LexerState) {
        self.state = state;
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

        self.skip_comments_and_white_spaces()?;

        if self.is_end() {
            return Err(Error::end_of_stream());
        }

        let current = self.reader.current()?;

        let start = self.reader.position();
        let value = match current {
            '/' if self.state.regex_allowed => self.read_regexp_literal(),
            // <op>=
            '/' | '*' | '%' | '+' | '-' | '|' | '^' | '&' | '<' | '>' | '='
                if self.reader.peek().ok() == Some(&'=') =>
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
            '<' if self.reader.peek().ok() == Some(&'<') => {
                self.reader.consume()?;
                produce!(self, peek: '=' ? punct!("<<=") ; punct!("<<"))
            }
            '>' if self.reader.peek().ok() == Some(&'>') => {
                self.reader.consume()?;
                match self.reader.peek() {
                    Ok(&'>') => {
                        self.reader.consume()?;
                        produce!(self, peek: '=' ? punct!(">>>=") ; punct!(">>>"))
                    }
                    Ok(&'=') => produce!(self, 2, punct!(">>=")),
                    _ => produce!(self, 1, punct!(">>")),
                }
            }
            '!' if self.reader.peek().ok() == Some(&'=') => {
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
                    Ok(&'.') if !matches!(self.reader.peek(), Ok('0'..='9')) => {
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
                if self.reader.peek().ok() == Some(&'*') {
                    self.reader.consume()?;
                    produce!(self, peek: '=' ? punct!("**=") ; punct!("**"))
                } else {
                    produce!(self, 1, punct!("*"))
                }
            }
            '.' => match self.reader.peek() {
                Ok('.') => {
                    self.reader.consume()?;
                    if self.reader.peek().ok() == Some(&'.') {
                        produce!(self, 2, punct!("..."))
                    } else {
                        let end = self.reader.position();
                        let error_token = Token::new(punct!("."), self.first_on_line, (start, end));
                        return Err(Error::invalid_or_unexpected_token(error_token));
                    }
                }
                Ok('0'..='9') => self.read_number_literal(),
                _ => produce!(self, 1, punct!(".")),
            },
            '/' => produce!(self, 1, punct!("/")),
            '0'..='9' => self.read_number_literal(),
            '"' | '\'' => self.read_string_literal(),
            '`' => self.read_template_literal_head(),
            c if c.is_start_of_identifier() => self.read_identifier_or_keyword(),
            // Handles identifiers and keywords that start with unicode escape sequence.
            '\\' => self.read_identifier_or_keyword(),
            _ => {
                let c = self.reader.consume()?;
                return Err(Error::unrecognized_code_point(
                    c,
                    (start, start + c.len_utf8()),
                ));
            }
        }?;
        let end = self.reader.position();

        // Support for legacy html end comment: `-->`
        if self.first_on_line && value == punct!("--") && self.reader.current().ok() == Some(&'>') {
            if !self.state.html_comment_allowed {
                return Err(Error::syntax_error(
                    "HTML comments are not allowed in this context".to_owned(),
                    (start, start + 3),
                ));
            }

            self.skip_rest_of_line();
            self.first_on_line = true;
            return self.read();
        }

        let token = Token::new(value, self.first_on_line, (start, end));
        self.first_on_line = false;

        Ok(token)
    }

    fn skip_comments_and_white_spaces(&mut self) -> Result<()> {
        loop {
            self.skip_whitespaces()?;

            match self.reader.current() {
                Ok('/') if self.reader.peek().ok() == Some(&'/') => {
                    self.skip_single_line_comment();
                    self.first_on_line = true;
                }
                Ok('<') if self.reader.peek().ok() == Some(&'!') => {
                    if !self.state.html_comment_allowed {
                        let position = self.reader.position();
                        return Err(Error::syntax_error(
                            "HTML comments are not allowed in this context".to_owned(),
                            (position, position + 2),
                        ));
                    }

                    self.skip_single_line_comment();
                    self.first_on_line = true;
                }
                Ok('/') if self.reader.peek().ok() == Some(&'*') => {
                    self.skip_multi_line_comment()?;
                }
                _ => break,
            }
        }

        self.skip_whitespaces()?;
        Ok(())
    }

    fn skip_single_line_comment(&mut self) {
        self.reader.consume().unwrap();
        self.reader.consume().unwrap();

        self.skip_rest_of_line();
    }

    fn skip_rest_of_line(&mut self) {
        self.reader
            .read_while(|c| !c.is_ecma_line_terminator())
            .unwrap();

        if self.reader.current().is_ok() {
            self.reader.consume().unwrap(); // Consume trailing new line
        }
    }

    /// Skips multi line comment, returns true if a new line was skipped.
    fn skip_multi_line_comment(&mut self) -> Result<()> {
        let span_start = self.reader.position();
        self.reader.consume()?;
        self.reader.consume()?;

        let mut content = String::new();
        loop {
            if matches!(self.reader.current(), Ok('*')) && self.reader.peek().ok() == Some(&'/') {
                self.reader.consume()?;
                self.reader.consume()?;
                break;
            }

            if self.reader.current().is_err() {
                return Err(Error::syntax_error(
                    "Unterminated comment".to_owned(),
                    (span_start, span_start + 2),
                ));
            }

            let char = self.reader.consume()?;
            if char.is_ecma_line_terminator() {
                self.first_on_line = true;
            }
            content.push(char);
        }

        Ok(())
    }

    fn read_identifier_or_keyword(&mut self) -> Result<TokenValue> {
        let word = self.read_identifier_or_keyword_expand_unicode()?;
        let value = if let Ok(keyword) = word.parse() {
            TokenValue::Keyword(keyword)
        } else {
            TokenValue::Identifier(word)
        };

        Ok(value)
    }

    fn read_identifier_or_keyword_expand_unicode(&mut self) -> Result<String> {
        let mut word = String::new();

        loop {
            match self.reader.current() {
                Ok(c) if c.is_part_of_identifier() => word.push(self.reader.consume()?),
                Ok('\\') => {
                    let is_start = word.is_empty();
                    word.push(self.read_and_expand_unicode_escape_sequence(is_start)?);
                }
                _ => break,
            }
        }

        Ok(word)
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
            Ok(&'x' | &'X') if current == &'0' => {
                (Hex, self.read_number(16, char::is_ascii_hexdigit)?)
            }
            Ok(&'o' | &'O') if current == &'0' => {
                (Octal, self.read_number(8, |c| ('0'..='7').contains(c))?)
            }
            Ok(&'b' | &'B') if current == &'0' => {
                (Binary, self.read_number(2, |c| c == &'0' || c == &'1')?)
            }
            _ => {
                return self.read_integer_or_decimal();
            }
        };

        Ok(literal!(number, base, number))
    }

    fn read_integer_or_decimal(&mut self) -> Result<TokenValue> {
        let integral = if self.reader.current()? == &'.' {
            // Decimals without zero: .5
            0
        } else {
            self.read_number(10, |c| c.is_numeric())?
        };

        if let Ok(&'.') = self.reader.current() {
            self.reader.consume()?;
            if !matches!(self.reader.current(), Ok('0'..='9')) {
                // Decimals without decimal part: 1.
                return Ok(literal!(decimal, integral as f64));
            }

            let fraction = self.read_number(10, |c| c.is_numeric())?;
            let digits = (fraction as f64).log10().floor() + 1.0;
            let float = integral as f64 + (fraction as f64 / 10_i32.pow(digits as u32) as f64);

            if let Some(exponent) = self.read_number_exponent()? {
                Ok(literal!(scientific, float, exponent))
            } else {
                Ok(literal!(decimal, float))
            }
        } else if let Some(exponent) = self.read_number_exponent()? {
            Ok(literal!(scientific, integral as f64, exponent))
        } else {
            Ok(literal!(integer, integral))
        }
    }

    fn read_number_exponent(&mut self) -> Result<Option<i32>> {
        if matches!(self.reader.current(), Ok(&'e' | &'E')) {
            self.reader.consume()?;

            let sign = if matches!(self.reader.current(), Ok('-')) {
                self.reader.consume()?;
                -1
            } else {
                if self.reader.current()? == &'+' {
                    self.reader.consume()?;
                }

                1
            };

            let exponent = self.read_number(10, |c| c.is_ascii_digit())?;
            Ok(Some(sign * exponent as i32))
        } else {
            Ok(None)
        }
    }

    fn read_number(&mut self, base: u32, check: fn(&char) -> bool) -> Result<i64> {
        let span_start = self.reader.position();

        // All but base 10 have 2 char prefix: 0b, 0o, 0x
        if base != 10 {
            self.reader.consume()?;
            self.reader.consume()?;
        }

        let number_str = self.reader.read_while(|c| check(c) || c == &'_')?;
        if number_str.contains("__") {
            let span_end = self.reader.position();
            return Err(Error::syntax_error(
                "number cannot contain multiple adjacent underscores".to_owned(),
                (span_start, span_end),
            ));
        }

        if number_str.ends_with('_') {
            let span_end = self.reader.position();
            return Err(Error::syntax_error(
                "number cannot end with underscore".to_owned(),
                (span_start, span_end),
            ));
        }

        let number_str = number_str.replace('_', "");

        if number_str.is_empty() {
            let position = self.reader.position();
            return Err(Error::syntax_error(
                "expected number".to_owned(),
                (position, position),
            ));
        }

        // The check must be strict enough for a safe unwrap here
        Ok(i64::from_str_radix(&number_str, base).unwrap())
    }

    fn skip_whitespaces(&mut self) -> Result<()> {
        loop {
            if self.is_end() {
                break;
            }

            if self.reader.current()?.is_ecma_line_terminator() {
                self.first_on_line = true;
                self.reader.consume()?;
                continue;
            }

            if self.reader.current()?.is_ecma_whitespace() {
                self.reader.consume()?;
                continue;
            }

            break;
        }

        Ok(())
    }

    fn is_end(&self) -> bool {
        use fajt_common::io::char_reader::Error;
        matches!(self.reader.current(), Err(Error::EndOfStream))
    }
}

pub struct LexerState {
    html_comment_allowed: bool,
    regex_allowed: bool,
    inside_template: bool,
}

impl LexerState {
    pub fn with_html_comments_allowed(&self, allowed: bool) -> Self {
        LexerState {
            html_comment_allowed: allowed,
            ..*self
        }
    }

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

impl Default for LexerState {
    fn default() -> Self {
        Self {
            html_comment_allowed: true,
            regex_allowed: false,
            inside_template: false,
        }
    }
}

impl Seek for Lexer<'_> {
    /// Seeks to `pos`.
    /// The first_on_line must be manually corrected by caller.
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        let pos = match pos {
            SeekFrom::Start(offset) => offset,
            SeekFrom::End(offset) => (self.data.len() as i64 + offset) as u64,
            SeekFrom::Current(offset) => (self.reader.position() as i64 + offset) as u64,
        };

        let offset = pos as usize;
        self.reader = PeekReader::with_offset(self.data[offset..].char_indices(), offset).unwrap();
        self.first_on_line = true;

        Ok(pos)
    }
}

impl ReReadWithState<Token> for Lexer<'_> {
    type Error = Error;
    type State = LexerState;

    /// Rewind reader to before `token`, `token` must have been previously read from this lexer.
    fn rewind_before(&mut self, token: &Token) {
        self.seek(SeekFrom::Start(token.span.start as u64)).unwrap();
        self.first_on_line = token.first_on_line;
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
    ) -> std::result::Result<(usize, Token), Self::Error> {
        mem::swap(&mut state, &mut self.state);
        let result = self.next()?;
        mem::swap(&mut self.state, &mut state);

        Ok(result)
    }
}

impl PeekRead<Token> for Lexer<'_> {
    type Error = Error;

    fn next(&mut self) -> std::result::Result<(usize, Token), Error> {
        let token = self.read()?;
        Ok((token.span.end, token))
    }
}
