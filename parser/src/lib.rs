#[macro_use]
extern crate fajt_lexer;

use crate::ast::{EmptyStmt, Program, Stmt, VariableType};
use fajt_lexer::token;
use fajt_lexer::token::Token;
use fajt_lexer::Lexer;

#[cfg(test)]
macro_rules! parser_test{
    (input: $input:literal, output:[$($output:expr),*]) => {
        let lexer = Lexer::new(&$input).unwrap();
        let reader = Reader::new(lexer);
        let mut parser = Parser::new(reader);
        let program = parser.parse();

        assert_eq!(program, Program::from_body(vec![$($output),*]))
    }
}

pub mod ast;
mod expression;
mod statement;

pub struct Reader<'a> {
    lexer: Lexer<'a>,
    current: Token,
}

impl<'a> Reader<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let current = lexer.read().unwrap();
        Reader { lexer, current }
    }

    pub fn current(&self) -> &Token {
        &self.current
    }

    pub fn next(&mut self) -> &Token {
        self.current = self.lexer.read().unwrap(); // TODO
        &self.current
    }
}

pub struct Parser<'a> {
    reader: Reader<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(reader: Reader<'a>) -> Self {
        Parser { reader }
    }

    pub fn parse(&mut self) -> Program {
        let stmt = self.parse_statement();
        Program::from_body(vec![stmt])
    }

    fn parse_statement(&mut self) -> Stmt {
        match self.reader.current.value {
            punct!(";") => Stmt::Empty(EmptyStmt::new(self.reader.current.location.clone())),
            punct!("{") => unimplemented!("BlockStatement"),
            keyword!("var") => self.parse_variable_statement(VariableType::Var),
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
