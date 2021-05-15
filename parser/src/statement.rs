use crate::ast::Statement::Expression;
use crate::ast::{BlockStatement, EmptyStatement, Statement, VariableKind};
use crate::error::Result;
use crate::Parser;
use fajt_lexer::keyword;
use fajt_lexer::punct;
use fajt_lexer::token::Span;
use fajt_lexer::token_matches;

mod variable;

impl Parser<'_> {
    pub(crate) fn parse_statement(&mut self) -> Result<Statement> {
        Ok(match self.reader.current()? {
            token_matches!(punct!(";")) => self.parse_empty_statement()?,
            token_matches!(punct!("{")) => self.parse_block_statement()?,
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
            // TODO LabelledStatement
            _ if self.is_expression_statement()? => self.parse_expression_statement()?,
            t => unimplemented!("Invalid statement error handling {:?}", t),
        })
    }

    fn is_expression_statement(&self) -> Result<bool> {
        let token = self.reader.current()?;
        if matches!(
            token.value,
            punct!("{") | keyword!("function") | keyword!("class")
        ) {
            return Ok(false);
        }

        // TODO async [no LineTerminator here] function, "async function" as a single token from lexer?

        if matches!(token.value, keyword!("let"))
            && token_matches!(self.reader.peek(), opt: punct!("["))
        {
            return Ok(false);
        }

        Ok(true)
    }

    /// Parses the `BlockStatement` goal symbol.
    ///
    /// Example:
    /// ```no_rust
    /// if (true) { return "success"; } else { return "fail"; }
    ///           ^~~~~~~~~~~~~~~~~~~~^      ^~~~~~~~~~~~~~~~~^
    /// ```
    fn parse_block_statement(&mut self) -> Result<Statement> {
        let token = self.reader.consume()?;
        debug_assert!(token_matches!(token, punct!("{")));

        let span_start = token.span.start;

        let mut statements = Vec::new();
        loop {
            match self.reader.current()? {
                token_matches!(punct!("}")) => {
                    self.reader.consume()?;
                    break;
                }
                _ => {
                    let statement = self.parse_statement()?;
                    statements.push(statement)
                },
            }
        }

        let span_end = self.reader.position();
        let span = Span::new(span_start, span_end);
        Ok(BlockStatement { span, statements }.into())
    }

    fn parse_expression_statement(&mut self) -> Result<Statement> {
        let expr = self.parse_expression()?;
        Ok(Expression(expr))
    }

    /// Parses the `EmptyStatement` goal symbol.
    ///
    /// Example:
    /// ```no_rust
    /// ;
    /// ^
    /// ```
    fn parse_empty_statement(&mut self) -> Result<Statement> {
        let token = self.reader.consume()?;
        debug_assert!(token_matches!(token, punct!(";")));

        Ok(Statement::Empty(EmptyStatement { span: token.span }))
    }
}
