#[macro_use]
extern crate fajt_lexer;

use fajt_lexer::Lexer;
use fajt_lexer::token;
use fajt_lexer::token::Token;
use crate::ast::{Program, Stmt, EmptyStmt};

pub mod ast;
mod statement;
mod expression;

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

    pub fn next(&mut self) -> &Token {
        self.current = self.lexer.read().unwrap(); // TODO
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

    pub fn parse(&mut self) -> Program {
        let stmt = self.parse_statement();
        Program::from_body(vec![stmt])
    }

    fn parse_statement(&mut self) -> Stmt {
        match self.reader.current.value {
            punct!(";") => Stmt::Empty(EmptyStmt::new(self.reader.current.location.clone())),
            punct!("{") => unimplemented!("BlockStatement"),
            keyword!("var") => self.parse_variable_statement(),
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

macro_rules! parser_test{
    (input: $input:literal, output:[$($output:expr),*]) => {
        let lexer = Lexer::new(&$input).unwrap();
        let reader = Reader::new(lexer);
        let mut parser = Parser::new(reader);
        let program = parser.parse();

        assert_eq!(program, Program::from_body(vec![$($output),*]))
    }
}

#[cfg(test)]
mod tests {
    use crate::{Parser, Reader};
    use fajt_lexer::Lexer;
    use crate::ast::{Program, Stmt, EmptyStmt, VariableDeclaration};
    use crate::ast::VariableType::Var;

    #[test]
    fn parse_empty_statement() {
        parser_test!(
            input: ";",
            output: [Stmt::Empty(EmptyStmt::new((0, 1).into()))]
        );
    }

    #[test]
    fn parse_var_statement() {
        parser_test!(
            input: ";",
            output: [
                Stmt::VariableDeclaration(VariableDeclaration {
                    identifier: "foo".to_owned(),
                    variable_type: Var,
                })
            ]
        );
    }
}
