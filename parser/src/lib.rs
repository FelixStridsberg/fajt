#[macro_use]
extern crate fajt_lexer;

use fajt_lexer::Lexer;
use fajt_lexer::token;
use fajt_lexer::token::Token;
use std::fs::read;

pub mod ast;

pub struct Reader<'a> {
    lexer: Lexer<'a>,
    current: Token
}

impl <'a>Reader<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let current = lexer.read().unwrap();
        Reader {
            lexer,
            current,
        }
    }

    pub fn current(&self) -> &Token {
        &self.current
    }
}

pub struct Parser<'a> {
    reader: Reader<'a>
}

impl <'a>Parser<'a> {
    pub fn new(reader: Reader<'a>) -> Self {
        Parser {
            reader
        }
    }

    pub fn parse(&mut self) {
        // TODO just testing for now
        self.parse_statement();
    }

    fn parse_statement(&mut self) {
        match self.reader.current.value {
            punct!("{") => unimplemented!("BlockStatement"),
            keyword!("var") => unimplemented!("VariableStatement"),
            punct!(";") => unimplemented!("EmptyStatement"),
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
            _ => unimplemented!("Invalid statement error handling")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Parser, Reader};
    use fajt_lexer::Lexer;

    #[test]
    fn it_works() {
        let input = "var a = 1;";
        let mut lexer = Lexer::new(&input).unwrap();
        let mut reader = Reader::new(lexer);
        let mut parser = Parser::new(reader);
        parser.parse();

    }
}
