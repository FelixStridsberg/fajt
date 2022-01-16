extern crate fajt_lexer;
extern crate serde;

#[macro_use]
pub mod error;
mod binary_expr;
mod binding;
mod class;
mod cover;
mod expr;
mod function;
mod iteration;
mod literal;
mod member_access;
mod method;
mod module;
mod stmt;
mod variable;

use std::cell::Cell;
use std::rc::Rc;

use fajt_ast::{Expr, Ident, LitString, Literal, Program, PropertyName, SourceType, Span, Stmt};
use fajt_common::io::{PeekRead, PeekReader, ReRead};
use fajt_lexer::token::{KeywordContext, Token, TokenValue};
use fajt_lexer::{punct, Lexer};
use fajt_lexer::{token_matches, LexerState};

use crate::error::ErrorKind::UnexpectedToken;
use crate::error::Result;

pub fn parse_program(input: &str) -> Result<Program> {
    parse::<Program>(input, SourceType::Unknown)
}

pub fn parse<T>(input: &str, source_type: SourceType) -> Result<T>
where
    T: Parse,
{
    let lexer = Lexer::new(input).unwrap();
    let mut reader = fajt_common::io::PeekReader::new(lexer).unwrap();
    Parser::parse::<T>(&mut reader, source_type)
}

#[derive(Default)]
pub struct ContextModify {
    is_await: Option<bool>,
    is_yield: Option<bool>,
    is_in: Option<bool>,
    is_strict: Option<bool>,
    is_default: Option<bool>,
}

impl ContextModify {
    pub fn new() -> Self {
        Self {
            is_await: None,
            is_yield: None,
            is_in: None,
            is_strict: None,
            is_default: None,
        }
    }

    pub fn set_yield(&mut self, value: bool) -> &mut Self {
        self.is_yield = Some(value);
        self
    }

    pub fn set_await(&mut self, value: bool) -> &mut Self {
        self.is_await = Some(value);
        self
    }

    pub fn set_in(&mut self, value: bool) -> &mut Self {
        self.is_in = Some(value);
        self
    }

    pub fn set_strict(&mut self, value: bool) -> &mut Self {
        self.is_strict = Some(value);
        self
    }

    pub fn set_default(&mut self, value: bool) -> &mut Self {
        self.is_default = Some(value);
        self
    }
}

#[derive(Clone)]
pub struct Context {
    is_await: bool,
    is_yield: bool,
    is_in: bool,
    is_strict: bool,
    is_default: bool,
}

impl Context {
    pub fn modify(&mut self, modify: &ContextModify) -> Self {
        let mut context = self.clone();
        if let Some(is_await) = modify.is_await {
            context.is_await = is_await;
        }

        if let Some(is_yield) = modify.is_yield {
            context.is_yield = is_yield;
        }

        if let Some(is_in) = modify.is_in {
            context.is_in = is_in;
        }

        if let Some(is_strict) = modify.is_strict {
            context.is_strict = is_strict;
        }

        if let Some(is_default) = modify.is_default {
            context.is_default = is_default;
        }

        context
    }

    fn keyword_context(&self) -> KeywordContext {
        let mut keyword_context = KeywordContext::empty();
        if self.is_await {
            keyword_context |= KeywordContext::AWAIT;
        }

        if self.is_yield {
            keyword_context |= KeywordContext::YIELD;
        }

        if self.is_strict {
            keyword_context |= KeywordContext::STRICT;
        }

        keyword_context
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            is_await: false,
            is_yield: false,
            is_in: false,
            is_strict: false,
            is_default: false,
        }
    }
}

pub trait Parse: Sized {
    fn parse<I>(parser: &mut Parser<I>) -> Result<Self>
    where
        I: PeekRead<Token, Error = fajt_lexer::error::Error>,
        I: ReRead<Token, State = LexerState, Error = fajt_lexer::error::Error>;
}

impl Parse for Expr {
    fn parse<I>(parser: &mut Parser<I>) -> Result<Self>
    where
        I: PeekRead<Token, Error = fajt_lexer::error::Error>,
        I: ReRead<Token, State = LexerState, Error = fajt_lexer::error::Error>,
    {
        parser
            .with_context(ContextModify::new().set_in(true))
            .parse_expr()
    }
}

impl Parse for Stmt {
    fn parse<I>(parser: &mut Parser<I>) -> Result<Self>
    where
        I: PeekRead<Token, Error = fajt_lexer::error::Error>,
        I: ReRead<Token, State = LexerState, Error = fajt_lexer::error::Error>,
    {
        parser.parse_stmt()
    }
}

impl Parse for Program {
    fn parse<I>(parser: &mut Parser<I>) -> Result<Self>
    where
        I: PeekRead<Token, Error = fajt_lexer::error::Error>,
        I: ReRead<Token, State = LexerState, Error = fajt_lexer::error::Error>,
    {
        let mut body = Vec::new();
        loop {
            if parser.reader.is_end() {
                break;
            }

            body.push(parser.parse_stmt()?);
        }
        Ok(Program::from_body(parser.source_type.get(), body))
    }
}

pub struct Parser<'a, I>
where
    I: PeekRead<Token, Error = fajt_lexer::error::Error>,
{
    context: Context,
    reader: &'a mut PeekReader<Token, I>,
    source_type: Rc<Cell<SourceType>>,
}

impl<'a, I> Parser<'a, I>
where
    I: PeekRead<Token, Error = fajt_lexer::error::Error>,
    I: ReRead<Token, State = LexerState, Error = fajt_lexer::error::Error>,
{
    pub fn new(reader: &'a mut PeekReader<Token, I>, source_type: SourceType) -> Result<Self> {
        Ok(Parser {
            context: Context::default(),
            reader,
            source_type: Rc::new(Cell::new(source_type)),
        })
    }

    pub fn parse<T>(reader: &'a mut PeekReader<Token, I>, source_type: SourceType) -> Result<T>
    where
        T: Parse,
    {
        let mut parser = Parser::new(reader, source_type)?;
        T::parse(&mut parser)
    }

    fn source_type(&self) -> SourceType {
        self.source_type.get()
    }

    fn set_source_type(&mut self, source_type: SourceType) {
        self.source_type.replace(source_type);
    }

    fn current(&self) -> Result<&Token> {
        Ok(self.reader.current()?)
    }

    fn consume(&mut self) -> Result<Token> {
        Ok(self.reader.consume()?)
    }

    fn peek(&self) -> Option<&Token> {
        self.reader.peek()
    }

    fn is_end(&self) -> bool {
        self.reader.is_end()
    }

    fn position(&self) -> usize {
        self.current()
            .map(|t| t.span.start)
            .unwrap_or_else(|_| self.reader.position())
    }

    fn span_from(&self, start: usize) -> Span {
        Span::new(start, self.reader.position())
    }

    pub fn with_context(&mut self, modify: &ContextModify) -> Parser<'_, I> {
        Parser {
            context: self.context.modify(modify),
            reader: self.reader,
            source_type: self.source_type.clone(),
        }
    }

    fn current_matches(&self, value: TokenValue) -> bool {
        if let Ok(token) = self.current() {
            token.value == value
        } else {
            false
        }
    }

    fn current_matches_string_literal(&self) -> bool {
        matches!(
            self.current(),
            Ok(Token {
                value: TokenValue::Literal(Literal::String(_)),
                ..
            })
        )
    }

    fn peek_matches(&self, value: TokenValue) -> bool {
        if let Some(token) = self.peek() {
            token.value == value
        } else {
            false
        }
    }

    fn consume_assert(&mut self, value: TokenValue) -> Result<Token> {
        let token = self.consume()?;
        if token.value != value {
            return err!(UnexpectedToken(token));
        }
        Ok(token)
    }

    fn maybe_consume(&mut self, value: TokenValue) -> Result<bool> {
        if self.current_matches(value) {
            self.consume()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn followed_by_new_lined(&self) -> bool {
        self.peek().map_or(false, |t| t.first_on_line)
    }

    fn peek_is_identifier(&self) -> bool {
        is_identifier(self.peek(), self.context.keyword_context())
    }

    fn is_identifier(&self) -> bool {
        is_identifier(self.current().ok(), self.context.keyword_context())
    }

    fn parse_identifier(&mut self) -> Result<Ident> {
        let token = self.consume()?;
        Ok(match token.value {
            TokenValue::Identifier(s) => Ident::new(s, token.span),
            TokenValue::Keyword(k) => {
                let str = k.into_identifier_string(self.context.keyword_context())?;
                Ident::new(str, token.span)
            }
            _ => return err!(UnexpectedToken(token)),
        })
    }

    fn parse_optional_identifier(&mut self) -> Result<Option<Ident>> {
        Ok(if self.is_identifier() {
            Some(self.parse_identifier()?)
        } else {
            None
        })
    }

    /// Parses the `PropertyName` goal symbol.
    fn parse_property_name(&mut self) -> Result<PropertyName> {
        match self.current()? {
            token_matches!(@literal) => {
                let token = self.consume()?;
                match token.value {
                    TokenValue::Literal(Literal::String(string)) => {
                        Ok(PropertyName::String(string))
                    }
                    TokenValue::Literal(Literal::Number(number)) => {
                        Ok(PropertyName::Number(number))
                    }
                    _ => err!(UnexpectedToken(token)),
                }
            }
            token_matches!(punct!("[")) => {
                self.consume()?;
                let expr = self.parse_assignment_expr()?;
                self.consume_assert(punct!("]"))?;
                Ok(PropertyName::Computed(expr))
            }
            _ if self.is_identifier() => Ok(PropertyName::Ident(self.parse_identifier()?)),
            _ => err!(UnexpectedToken(self.consume()?)),
        }
    }

    fn parse_directive_prologue(&mut self) -> Result<Vec<LitString>> {
        let mut directives = Vec::new();

        loop {
            if self.current_matches_string_literal() {
                let stmt = self.parse_stmt()?;
                let string = stmt
                    .unwrap_expr_stmt()
                    .expr
                    .unwrap_literal()
                    .literal
                    .unwrap_string();
                directives.push(string);
            } else {
                break;
            }
        }

        Ok(directives)
    }

    fn consume_array_delimiter(&mut self) -> Result<()> {
        self.consume_list_delimiter(punct!("]"))
    }

    fn consume_object_delimiter(&mut self) -> Result<()> {
        self.consume_list_delimiter(punct!("}"))
    }

    fn consume_parameter_delimiter(&mut self) -> Result<()> {
        self.consume_list_delimiter(punct!(")"))
    }

    fn consume_list_delimiter(&mut self, list_end: TokenValue) -> Result<()> {
        if !self.maybe_consume(punct!(","))? && !self.current_matches(list_end) {
            return err!(UnexpectedToken(self.consume()?));
        }

        Ok(())
    }
}

fn is_identifier(token: Option<&Token>, keyword_context: KeywordContext) -> bool {
    match token {
        Some(Token {
            value: TokenValue::Identifier(_),
            ..
        }) => true,
        Some(Token {
            value: TokenValue::Keyword(keyword),
            ..
        }) => keyword.is_allows_as_identifier(keyword_context),
        _ => false,
    }
}
