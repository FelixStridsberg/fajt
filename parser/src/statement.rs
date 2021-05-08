use crate::ast::Stmt::ExpressionStmt;
use crate::ast::{EmptyStmt, Stmt, VariableKind};
use crate::error::Result;
use crate::Parser;
use fajt_lexer::keyword;
use fajt_lexer::punct;
use fajt_lexer::token_matches;

mod variable;

impl Parser<'_> {
    pub(crate) fn parse_statement(&mut self) -> Result<Stmt> {
        Ok(match self.reader.current()? {
            token_matches!(punct!(";")) => self.parse_empty_statement()?,
            token_matches!(punct!("{")) => unimplemented!("BlockStatement"),
            token_matches!(keyword!("var")) => self.parse_variable_statement(VariableKind::Var)?,
            token_matches!(keyword!("if")) => unimplemented!("IfStatement"),
            token_matches!(keyword!("break")) => unimplemented!("BreakStatement"),
            token_matches!(keyword!("continue")) => unimplemented!("ContinueStatement"),
            token_matches!(keyword!("break")) => unimplemented!("BreakStatement"),
            token_matches!(keyword!("return")) => unimplemented!("ReturnStatement"),
            token_matches!(keyword!("with")) => unimplemented!("WithStatement"),
            token_matches!(keyword!("throw")) => unimplemented!("ThrowStatement"),
            token_matches!(keyword!("try")) => unimplemented!("TryStatement"),
            token_matches!(keyword!("debugger")) => unimplemented!("DebuggerStatement"),
            // TODO ExpressionStatement
            // TODO LabelledStatement
            _ if self.is_expression_statement() => self.parse_expression_statement()?,
            _ => unimplemented!("Invalid statement error handling"),
        })
    }

    fn is_expression_statement(&self) -> bool {
        if let Some(peek) = self.reader.peek() {
            // TODO async [no LineTerminator here} function
            !matches!(
                peek.value,
                punct!("{")
                    | keyword!("function")
                    | keyword!("class")
                    | keyword!("let")
                    | punct!("[")
            )
        } else {
            true
        }
    }

    fn parse_expression_statement(&mut self) -> Result<Stmt> {
        let expr = self.parse_expression()?;
        Ok(ExpressionStmt(expr).into())
    }

    fn parse_empty_statement(&mut self) -> Result<Stmt> {
        let stmt = Stmt::Empty(EmptyStmt::new(self.reader.current()?.span.clone()));
        self.reader.consume()?;
        Ok(stmt)
    }
}
