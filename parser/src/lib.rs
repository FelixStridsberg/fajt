extern crate fajt_lexer;
pub mod ast;
pub mod error;
mod expression;
mod statement;

use crate::ast::{Expression, Ident, Program};
use crate::error::ErrorKind::UnexpectedToken;
use crate::error::{Error, Result};
use fajt_common::io::PeekReader;
use fajt_lexer::keyword;
use fajt_lexer::punct;
use fajt_lexer::token;
use fajt_lexer::token::Token;
use fajt_lexer::token_matches;
use fajt_lexer::Lexer;
use std::convert::TryInto;

pub struct Parser<'a> {
    reader: PeekReader<Token, Lexer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Result<Self> {
        let reader = PeekReader::new(lexer)?;
        Ok(Parser { reader })
    }

    pub fn parse(&mut self) -> Result<Program> {
        let stmt = self.parse_statement()?;
        Ok(Program::from_body(vec![stmt]))
    }

    // TODO probably not appropriate, used for testing parsing expressions currently.
    pub fn parse_expr(&mut self) -> Result<Expression> {
        self.parse_expression()
    }

    fn is_binding_identifier(&self) -> Result<bool> {
        let token = self.reader.current()?;
        Ok(token_matches!(token, @ident)
            || token_matches!(token, keyword!("await"))
            || token_matches!(token, keyword!("yield")))
    }

    fn parse_binding_identifier(&mut self) -> Result<Ident> {
        match self.reader.current()? {
            token_matches!(@ident) => self.reader.consume()?.try_into(),
            token_matches!(keyword!("await")) | token_matches!(keyword!("yield")) => {
                // TODO fail depending on context
                self.reader.consume()?.try_into()
            }
            _ => return Err(Error::of(UnexpectedToken(self.reader.consume()?))),
        }
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
