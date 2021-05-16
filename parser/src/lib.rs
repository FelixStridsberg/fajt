extern crate fajt_lexer;
pub mod ast;
pub mod error;
mod expression;
mod statement;

use crate::ast::{Expression, Ident, Program};
use crate::error::ErrorKind::UnexpectedToken;
use crate::error::{Error, Result};
use fajt_common::io::PeekReader;
use fajt_lexer::punct;
use fajt_lexer::token;
use fajt_lexer::token::{KeywordContext, Token, TokenValue};
use fajt_lexer::token_matches;
use fajt_lexer::Lexer;

pub struct Parser<'a> {
    keyword_context: KeywordContext,
    reader: &'a mut PeekReader<Token, Lexer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(reader: &'a mut PeekReader<Token, Lexer<'a>>) -> Result<Self> {
        Ok(Parser {
            keyword_context: KeywordContext::empty(),
            reader,
        })
    }

    // TODO with parser context and not KeywordContext
    pub fn with_context(&'a mut self, context: KeywordContext) -> Self {
        Parser {
            keyword_context: context,
            reader: &mut self.reader,
        }
    }

    pub fn parse(&mut self) -> Result<Program> {
        let stmt = self.parse_statement()?;
        Ok(Program::from_body(vec![stmt]))
    }

    // TODO probably not appropriate, used for testing parsing expressions currently.
    pub fn parse_expr(&mut self) -> Result<Expression> {
        self.parse_expression()
    }

    fn is_identifier(&self) -> Result<bool> {
        let token = self.reader.current()?;
        Ok(match &token.value {
            TokenValue::Identifier(_) => true,
            TokenValue::Keyword(k) => k.is_allows_as_identifier(self.keyword_context),
            _ => false,
        })
    }

    fn parse_identifier(&mut self) -> Result<Ident> {
        let token = self.reader.consume()?;
        Ok(match token.value {
            TokenValue::Identifier(s) => Ident::new(s, token.span),
            TokenValue::Keyword(k) => {
                // TODO error handling
                let str = k.into_identifier_string(self.keyword_context).unwrap();
                Ident::new(str, token.span)
            }
            _ => return Err(Error::of(UnexpectedToken(token))),
        })
    }

    fn consume_array_delimiter(&mut self) -> Result<()> {
        match self.reader.current()? {
            token_matches!(punct!(",")) => {
                self.reader.consume()?;
                Ok(())
            }
            token_matches!(punct!("]")) => Ok(()),
            _ => Err(Error::of(UnexpectedToken(self.reader.consume()?))),
        }
    }

    fn consume_object_delimiter(&mut self) -> Result<()> {
        match self.reader.current()? {
            token_matches!(punct!(",")) => {
                self.reader.consume()?;
                Ok(())
            }
            token_matches!(punct!("}")) => Ok(()),
            _ => Err(Error::of(UnexpectedToken(self.reader.consume()?))),
        }
    }
}
