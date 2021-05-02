use crate::ast::{EmptyStmt, Stmt, VariableKind};
use crate::error::Result;
use crate::Parser;
use fajt_lexer::keyword;
use fajt_lexer::punct;

mod variable;

impl Parser<'_> {
    pub(crate) fn parse_statement(&mut self) -> Result<Stmt> {
        Ok(match self.reader.current().value {
            punct!(";") => Stmt::Empty(EmptyStmt::new(self.reader.current().location.clone())),
            punct!("{") => unimplemented!("BlockStatement"),
            keyword!("var") => self.parse_variable_statement(VariableKind::Var)?,
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
        })
    }
}
