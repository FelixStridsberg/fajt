use crate::ast::Statement::Expression;
use crate::ast::{
    BlockStatement, EmptyStatement, FormalParameters, FunctionDeclaration, Ident, Statement,
    VariableKind,
};
use crate::error::{Error, ErrorKind, Result};
use crate::{ContextModify, Parser};
use fajt_lexer::keyword;
use fajt_lexer::punct;
use fajt_lexer::token_matches;

mod variable;

impl Parser<'_, '_> {
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
            return Ok(self.reader.peek().unwrap().first_on_line);
        }

        Ok(true)
    }

    /// Parses the `AsyncFunctionDeclaration` goal symbol.
    ///
    /// Example:
    /// ```no_rust
    /// async function fn( ...args ) { return 1 };
    /// ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~^
    /// ```
    fn parse_async_function_declaration(&mut self) -> Result<Statement> {
        let span_start = self.position();
        let token = self.reader.consume()?;
        debug_assert!(token_matches!(token, keyword!("async")));

        let function_token = self.reader.consume()?;
        debug_assert!(token_matches!(function_token, keyword!("function")));
        debug_assert_eq!(function_token.first_on_line, false);

        let ident = self.parse_identifier()?;

        self.with_context(ContextModify::new().set_yield(false).set_await(true))
            .parse_function_implementation(span_start, ident, true)
    }

    /// Parses the `FunctionDeclaration` goal symbol.
    ///
    /// Example:
    /// ```no_rust
    /// function fn( ...args ) { return 1 };
    /// ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~^
    /// ```
    fn parse_function_declaration(&mut self) -> Result<Statement> {
        let span_start = self.position();
        let token = self.reader.consume()?;
        debug_assert!(token_matches!(token, keyword!("function")));

        let ident = self.parse_identifier()?;

        self.with_context(ContextModify::new().set_yield(false).set_await(false))
            .parse_function_implementation(span_start, ident, false)
    }

    /// Parses the part after the identifier of a function declaration.
    ///
    /// Example:
    /// ```no_rust
    /// function fn( ...args ) { return 1 };
    ///            ^~~~~~~~~~~~~~~~~~~~~~~~^
    ///
    /// async function fn( ...args ) { return 1 };
    ///                  ^~~~~~~~~~~~~~~~~~~~~~~~^
    /// ```
    fn parse_function_implementation(
        &mut self,
        span_start: usize,
        ident: Ident,
        asynchronous: bool,
    ) -> Result<Statement> {
        let parameters = self.parse_formal_parameters()?;
        let body = self.parse_function_body()?;

        let span = self.span_from(span_start);
        Ok(FunctionDeclaration {
            span,
            asynchronous,
            ident,
            parameters,
            body,
        }
        .into())
    }

    /// Parses the `FormalParameters` goal symbol.
    ///
    /// Example:
    /// ```no_rust
    /// function fn(a, b, ...c) {}
    ///             ^~~~~~~~~^
    /// ```
    fn parse_formal_parameters(&mut self) -> Result<Option<FormalParameters>> {
        let span_start = self.position();
        let token = self.reader.consume()?;
        if !token_matches!(token, punct!("(")) {
            return Err(Error::of(ErrorKind::UnexpectedToken(token)));
        }

        if token_matches!(self.reader.current()?, punct!(")")) {
            self.reader.consume()?;
            return Ok(None);
        }

        let mut rest = None;
        loop {
            match self.reader.current()? {
                token_matches!(punct!(")")) => {
                    self.reader.consume()?;
                    break;
                }
                token_matches!(punct!("...")) => {
                    rest = Some(self.parse_binding_rest_element()?);
                }
                _ => {
                    return Err(Error::of(ErrorKind::UnexpectedToken(
                        self.reader.consume()?,
                    )))
                }
            }
        }

        let span = self.span_from(span_start);
        Ok(Some(FormalParameters { span, rest }))
    }

    /// Parses the `FunctionBody` or `AsyncFunctionBody` goal symbol.
    /// Note: Only the +Await and ~Yield is different between async an non async, that is implicit
    /// by the context.
    ///
    /// Example:
    /// ```no_rust
    /// function fn() { var a = 1; }
    ///               ^~~~~~~~~~~~~^
    /// ```
    fn parse_function_body(&mut self) -> Result<Vec<Statement>> {
        let token = self.reader.consume()?;
        if !token_matches!(token, punct!("{")) {
            todo!("Error handling")
        }

        let mut body = Vec::new();
        loop {
            if token_matches!(self.reader.current()?, punct!("}")) {
                self.reader.consume()?;
                break;
            }

            body.push(self.parse_statement()?);
        }
        Ok(body)
    }

    /// Parses the `BlockStatement` goal symbol.
    ///
    /// Example:
    /// ```no_rust
    /// if (true) { return "success"; } else { return "fail"; }
    ///           ^~~~~~~~~~~~~~~~~~~~^      ^~~~~~~~~~~~~~~~~^
    /// ```
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
