use crate::ast::Statement::Expression;
use crate::ast::{
    BlockStatement, BreakStatement, CatchClause, ContinueStatement, DebuggerStatement,
    EmptyStatement, IfStatement, ReturnStatement, Statement, ThrowStatement, TryStatement,
    VariableKind, WithStatement,
};
use crate::error::ErrorKind::UnexpectedToken;
use crate::error::Result;
use crate::Parser;
use fajt_common::io::PeekRead;
use fajt_lexer::keyword;
use fajt_lexer::punct;
use fajt_lexer::token::Token;
use fajt_lexer::token_matches;

impl<I> Parser<'_, I>
where
    I: PeekRead<Token, Error = fajt_lexer::error::Error>,
{
    pub(super) fn parse_statement(&mut self) -> Result<Statement> {
        Ok(match self.reader.current()? {
            token_matches!(punct!(";")) => self.parse_empty_statement()?,
            token_matches!(punct!("{")) => self.parse_block_statement()?,
            token_matches!(keyword!("var")) => self.parse_variable_statement(VariableKind::Var)?,
            token_matches!(keyword!("const")) => {
                self.parse_variable_statement(VariableKind::Const)?
            }
            token_matches!(keyword!("let")) => self.parse_variable_statement(VariableKind::Let)?,
            token_matches!(keyword!("if")) => self.parse_if_statement()?,
            token_matches!(keyword!("break")) => self.parse_break_statement()?,
            token_matches!(keyword!("continue")) => self.parse_continue_statement()?,
            token_matches!(keyword!("return")) => self.parse_return_statement()?,
            token_matches!(keyword!("with")) => self.parse_with_statement()?,
            token_matches!(keyword!("throw")) => self.parse_throw_statement()?,
            token_matches!(keyword!("try")) => self.parse_try_statement()?,
            token_matches!(keyword!("debugger")) => self.parse_debugger_statement()?,
            // TODO LabelledStatement
            _ if self.is_expression_statement()? => self.parse_expression_statement()?,

            // Declarations are handles as statements
            token_matches!(keyword!("function")) => self.parse_function_declaration()?,
            token_matches!(keyword!("async"))
                if token_matches!(self.reader.peek(), opt: keyword!("function")) =>
            {
                self.parse_async_function_declaration()?
            }

            t => unimplemented!("Invalid statement error handling {:?}", t),
        })
    }

    /// Check if current position matches the start of an expression statement as specified in the
    /// `ExpressionStatement` goal symbol.
    fn is_expression_statement(&self) -> Result<bool> {
        let token = self.reader.current()?;
        if matches!(
            token.value,
            punct!("{") | keyword!("function") | keyword!("class")
        ) {
            return Ok(false);
        }

        if matches!(token.value, keyword!("let"))
            && token_matches!(self.reader.peek(), opt: punct!("["))
        {
            return Ok(false);
        }

        if matches!(token.value, keyword!("async"))
            && token_matches!(self.reader.peek(), opt: keyword!("function"))
        {
            return Ok(self.followed_by_new_lined());
        }

        Ok(true)
    }

    /// Parses the `BlockStatement` goal symbol.
    fn parse_block_statement(&mut self) -> Result<Statement> {
        let span_start = self.position();
        let token = self.reader.consume()?;
        debug_assert!(token_matches!(token, punct!("{")));

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
                }
            }
        }

        let span = self.span_from(span_start);
        Ok(BlockStatement { span, statements }.into())
    }

    fn parse_expression_statement(&mut self) -> Result<Statement> {
        let expr = self.parse_expression()?;
        Ok(Expression(expr))
    }

    /// Parses the `EmptyStatement` goal symbol.
    fn parse_empty_statement(&mut self) -> Result<Statement> {
        let token = self.reader.consume()?;
        debug_assert!(token_matches!(token, punct!(";")));

        Ok(Statement::Empty(EmptyStatement { span: token.span }))
    }

    /// Parses the `BreakStatement` goal symbol.
    fn parse_break_statement(&mut self) -> Result<Statement> {
        let span_start = self.position();
        let token = self.reader.consume()?;
        debug_assert!(token_matches!(token, keyword!("break")));

        let label = match self.reader.current() {
            token_matches!(ok: punct!(";")) | Err(_) => None,
            Ok(token) if token.first_on_line => None,
            _ => Some(self.parse_identifier()?),
        };

        let span = self.span_from(span_start);
        Ok(BreakStatement { span, label }.into())
    }

    /// Parses the `ContinueStatement` goal symbol.
    fn parse_continue_statement(&mut self) -> Result<Statement> {
        let span_start = self.position();
        let token = self.reader.consume()?;
        debug_assert!(token_matches!(token, keyword!("continue")));

        let label = match self.reader.current() {
            token_matches!(ok: punct!(";")) | Err(_) => None,
            Ok(token) if token.first_on_line => None,
            _ => Some(self.parse_identifier()?),
        };

        let span = self.span_from(span_start);
        Ok(ContinueStatement { span, label }.into())
    }

    /// Parses the `ReturnStatement` goal symbol.
    fn parse_return_statement(&mut self) -> Result<Statement> {
        let span_start = self.position();
        let token = self.reader.consume()?;
        debug_assert!(token_matches!(token, keyword!("return")));

        let argument = match self.reader.current() {
            token_matches!(ok: punct!(";")) | Err(_) => None,
            Ok(token) if token.first_on_line => None,
            _ => Some(self.parse_expression()?),
        };

        let span = self.span_from(span_start);
        Ok(ReturnStatement { span, argument }.into())
    }

    /// Parses the `ThrowStatement` goal symbol.
    fn parse_throw_statement(&mut self) -> Result<Statement> {
        let span_start = self.position();
        let token = self.reader.consume()?;
        debug_assert!(token_matches!(token, keyword!("throw")));

        let argument = match self.reader.current() {
            token_matches!(ok: punct!(";")) | Err(_) => None,
            Ok(token) if token.first_on_line => None,
            _ => Some(self.parse_expression()?),
        };

        let span = self.span_from(span_start);
        Ok(ThrowStatement { span, argument }.into())
    }

    /// Parses the `DebuggerStatement` goal symbol.
    fn parse_debugger_statement(&mut self) -> Result<Statement> {
        let token = self.reader.consume()?;
        debug_assert!(token_matches!(token, keyword!("debugger")));

        Ok(DebuggerStatement { span: token.span }.into())
    }

    /// Parses the `IfStatement` goal symbol.
    fn parse_if_statement(&mut self) -> Result<Statement> {
        let span_start = self.position();
        let token = self.reader.consume()?;
        debug_assert!(token_matches!(token, keyword!("if")));

        let open_paren = self.reader.consume()?;
        if !token_matches!(open_paren, punct!("(")) {
            return err!(UnexpectedToken(open_paren));
        }

        let condition = self.parse_expression()?;

        let close_paren = self.reader.consume()?;
        if !token_matches!(close_paren, punct!(")")) {
            return err!(UnexpectedToken(close_paren));
        }

        let consequent = self.parse_statement()?;

        let alternate = if token_matches!(self.reader.current(), ok: keyword!("else")) {
            self.reader.consume()?;
            Some(self.parse_statement()?)
        } else {
            None
        };

        let span = self.span_from(span_start);
        Ok(IfStatement {
            span,
            condition,
            consequent,
            alternate,
        }
        .into())
    }

    /// Parses the `WithStatement` goal symbol.
    fn parse_with_statement(&mut self) -> Result<Statement> {
        let span_start = self.position();
        let token = self.reader.consume()?;
        debug_assert!(token_matches!(token, keyword!("with")));

        let open_paren = self.reader.consume()?;
        if !token_matches!(open_paren, punct!("(")) {
            return err!(UnexpectedToken(open_paren));
        }

        let object = self.parse_expression()?;

        let close_paren = self.reader.consume()?;
        if !token_matches!(close_paren, punct!(")")) {
            return err!(UnexpectedToken(close_paren));
        }

        let body = self.parse_statement()?;
        let span = self.span_from(span_start);
        Ok(WithStatement { span, object, body }.into())
    }

    /// Parses the `TryStatement` goal symbol.
    fn parse_try_statement(&mut self) -> Result<Statement> {
        let span_start = self.position();
        let token = self.reader.consume()?;
        debug_assert!(token_matches!(token, keyword!("try")));

        let block = self.parse_block_statement()?.unwrap_block_statement();

        let handler = if token_matches!(self.reader.current(), ok: keyword!("catch")) {
            let span_start = self.position();
            self.reader.consume()?;
            let parameter = if token_matches!(self.reader.current(), ok: punct!("(")) {
                self.reader.consume()?;
                let pattern = self.parse_binding_pattern()?;
                let close_paren = self.reader.consume()?;
                if !token_matches!(close_paren, punct!(")")) {
                    return err!(UnexpectedToken(close_paren));
                }
                Some(pattern)
            } else {
                None
            };

            let body = self.parse_block_statement()?.unwrap_block_statement();

            let span = self.span_from(span_start);
            Some(CatchClause {
                span,
                parameter,
                body,
            })
        } else {
            None
        };

        let span = self.span_from(span_start);
        Ok(TryStatement {
            span,
            block,
            handler,
            finalizer: None,
        }
        .into())
    }
}
