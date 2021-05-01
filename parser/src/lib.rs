extern crate fajt_lexer;

use crate::ast::{EmptyStmt, Program, Stmt, VariableKind};
use crate::reader::Reader;
use crate::error::Result;
use fajt_lexer::keyword;
use fajt_lexer::punct;
use fajt_lexer::token;
use fajt_lexer::Lexer;

pub mod error;
pub mod ast;
mod expression;
mod reader;
mod statement;

pub struct Parser<'a> {
    reader: Reader<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Result<Self> {
        let reader = Reader::new(lexer)?;
        Ok(Parser { reader })
    }

    pub fn parse(&mut self) -> Program {
        let stmt = self.parse_statement();
        Program::from_body(vec![stmt])
    }

    fn parse_statement(&mut self) -> Stmt {
        match self.reader.current().value {
            punct!(";") => Stmt::Empty(EmptyStmt::new(self.reader.current().location.clone())),
            punct!("{") => unimplemented!("BlockStatement"),
            keyword!("var") => self.parse_variable_statement(VariableKind::Var),
            keyword!("if") => unimplemented!("IfStatement"),
            keyword!("break") => unimplemented!("BreakStatement"),
            keyword!("continue") => unimplemented!("ContinueStatement"),
            keyword!("break") => unimplemented!("BreakStatement"),
            keyword!("return") => unimplemented!("ReturnStatement"),
            keyword!("with") => unimplemented!("WithStatement"),
            keyword!("throw") => unimplemented!("ThrowStatement"),
            keyword!("try") => unimplemented!("TryStatement"),
            keyword!("debugger") => unimplemented!("DebuggerStatement"),
            // TODO ExpressionStatement
            // TODO LabelledStatement
            _ => unimplemented!("Invalid statement error handling"),
        }
    }
}
