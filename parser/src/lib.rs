extern crate fajt_lexer;
pub mod ast;
pub mod error;
mod expression;
mod statement;

use crate::ast::{Program, Expr};
use crate::error::Result;
use fajt_common::io::PeekReader;
use fajt_lexer::token;
use fajt_lexer::token::Token;
use fajt_lexer::Lexer;

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
    pub fn parse_expr(&mut self) -> Result<Expr> {
        self.parse_expression()
    }
}
