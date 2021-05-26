use fajt_common::io::PeekReader;
use fajt_lexer::punct;
use fajt_lexer::token::{KeywordContext, Span, Token, TokenValue};
use fajt_lexer::token_matches;
use fajt_lexer::Lexer;

use crate::ast::{Expression, Ident, Program, PropertyName};
use crate::error::ErrorKind::UnexpectedToken;
use crate::error::Result;

mod binding;
mod expression;
mod function;
mod literal;
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

pub struct Parser<'a, 'b> {
    context: Context,
    reader: &'a mut PeekReader<Token, Lexer<'b>>,
}

impl<'a, 'b> Parser<'a, 'b> {
    pub fn new(reader: &'a mut PeekReader<Token, Lexer<'b>>) -> Result<Self> {
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

    fn with_context(&mut self, modify: &ContextModify) -> Parser<'_, 'b> {
        Parser {
            context: self.context.modify(modify),
            reader: &mut self.reader,
        }
    }

    fn is_identifier(&self) -> Result<bool> {
        let token = self.reader.current()?;
        Ok(match &token.value {
            TokenValue::Identifier(_) => true,
            TokenValue::Keyword(k) => k.is_allows_as_identifier(self.context.keyword_context()),
            _ => false,
        })
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

    /// Parses the `PropertyName` goal symbol.
    fn parse_property_name(&mut self) -> Result<PropertyName> {
        match self.reader.current()? {
            token_matches!(@ident) => Ok(PropertyName::Ident(self.parse_identifier()?)),
            token_matches!(@literal) => todo!(),
            token_matches!(punct!("[")) => todo!(),
            _ => return err!(UnexpectedToken(self.reader.consume()?)),
        }
    }

    /// TODO 3 similar functions, merge them to one (consume_*_delimiter)
    fn consume_array_delimiter(&mut self) -> Result<()> {
        match self.reader.current()? {
            token_matches!(punct!(",")) => {
                self.reader.consume()?;
                Ok(())
            }
            token_matches!(punct!("]")) => Ok(()),
            _ => err!(UnexpectedToken(self.reader.consume()?)),
        }
    }

    fn consume_object_delimiter(&mut self) -> Result<()> {
        match self.reader.current()? {
            token_matches!(punct!(",")) => {
                self.reader.consume()?;
                Ok(())
            }
            token_matches!(punct!("}")) => Ok(()),
            _ => err!(UnexpectedToken(self.reader.consume()?)),
        }
    }

    fn consume_parameter_delimiter(&mut self) -> Result<()> {
        match self.reader.current()? {
            token_matches!(punct!(",")) => {
                self.reader.consume()?;
                Ok(())
            }
            token_matches!(punct!(")")) => Ok(()),
            _ => err!(UnexpectedToken(self.reader.consume()?)),
        }
    }
}
