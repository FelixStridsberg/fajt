use fajt_common::io::{PeekRead, PeekReader};
use fajt_lexer::punct;
use fajt_lexer::token::{KeywordContext, Span, Token, TokenValue};
use fajt_lexer::token_matches;

use crate::ast::{Expression, Ident, Program, PropertyName};
use crate::error::ErrorKind::UnexpectedToken;
use crate::error::Result;

mod binary_expression;
mod binding;
mod class;
mod cover;
mod expression;
mod function;
mod literal;
mod member_access;
mod statement;
mod variable;

#[derive(Default)]
pub struct ContextModify {
    is_await: Option<bool>,
    is_yield: Option<bool>,
}

impl ContextModify {
    pub fn new() -> Self {
        Self {
            is_await: None,
            is_yield: None,
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
}

#[derive(Clone)]
pub struct Context {
    is_await: bool,
    is_yield: bool,
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

        keyword_context
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            is_await: false,
            is_yield: false,
        }
    }
}

pub struct Parser<'a, I>
where
    I: PeekRead<Token, Error = fajt_lexer::error::Error>,
{
    context: Context,
    reader: &'a mut PeekReader<Token, I>,
}

impl<'a, I> Parser<'a, I>
where
    I: PeekRead<Token, Error = fajt_lexer::error::Error>,
{
    pub fn new(reader: &'a mut PeekReader<Token, I>) -> Result<Self> {
        Ok(Parser {
            context: Context::default(),
            reader,
        })
    }

    pub fn parse(&mut self) -> Result<Program> {
        let stmt = self.parse_statement()?;
        Ok(Program::from_body(vec![stmt]))
    }

    // TODO probably not appropriate, used for testing parsing expressions currently.
    pub fn parse_expr(&mut self) -> Result<Expression> {
        self.parse_expression()
    }

    fn position(&self) -> usize {
        self.reader
            .current()
            .map(|t| t.span.start)
            .unwrap_or_else(|_| self.reader.position())
    }

    fn span_from(&self, start: usize) -> Span {
        Span::new(start, self.reader.position())
    }

    pub fn with_context(&mut self, modify: &ContextModify) -> Parser<'_, I> {
        Parser {
            context: self.context.modify(modify),
            reader: &mut self.reader,
        }
    }

    fn current_matches(&self, value: TokenValue) -> bool {
        if let Ok(token) = self.reader.current() {
            token.value == value
        } else {
            false
        }
    }

    fn peek_matches(&self, value: TokenValue) -> bool {
        if let Some(token) = self.reader.peek() {
            token.value == value
        } else {
            false
        }
    }

    fn consume_known(&mut self, value: TokenValue) -> Result<()> {
        let token = self.reader.consume()?;
        if token.value != value {
            return err!(UnexpectedToken(token));
        }
        Ok(())
    }

    fn followed_by_new_lined(&self) -> bool {
        self.reader.peek().map_or(false, |t| t.first_on_line)
    }

    fn peek_is_identifier(&self) -> bool {
        is_identifier(self.reader.peek(), self.context.keyword_context())
    }

    fn is_identifier(&self) -> bool {
        is_identifier(self.reader.current().ok(), self.context.keyword_context())
    }

    fn parse_identifier(&mut self) -> Result<Ident> {
        let token = self.reader.consume()?;
        Ok(match token.value {
            TokenValue::Identifier(s) => Ident::new(s, token.span),
            TokenValue::Keyword(k) => {
                // TODO error handling
                let str = k
                    .into_identifier_string(self.context.keyword_context())
                    .unwrap();
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
        match self.reader.current()? {
            token_matches!(@literal) => todo!(),
            token_matches!(punct!("[")) => todo!(),
            _ if self.is_identifier() => Ok(PropertyName::Ident(self.parse_identifier()?)),
            _ => return err!(UnexpectedToken(self.reader.consume()?)),
        }
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
        if self.current_matches(punct!(",")) {
            self.reader.consume()?;
        }
        else if !self.current_matches(list_end) {
            return err!(UnexpectedToken(self.reader.consume()?))
        }

        return Ok(())
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
