extern crate fajt_lexer;
pub mod ast;
pub mod error;
mod expression;
mod reader;
mod statement;

use crate::ast::Program;
use crate::error::Result;
use crate::reader::Reader;
use fajt_lexer::token;
use fajt_lexer::Lexer;

pub struct Parser<'a> {
    reader: Reader<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Result<Self> {
        let reader = Reader::new(lexer)?;
        Ok(Parser { reader })
    }

    pub fn parse(&mut self) -> Result<Program> {
        let stmt = self.parse_statement()?;
        Ok(Program::from_body(vec![stmt]))
    }
}
