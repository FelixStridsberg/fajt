use crate::ast::{FormalParameters, FunctionDeclaration, Ident, Statement};
use crate::error::{ErrorKind, Result};
use crate::parser::ContextModify;
use crate::Parser;
use fajt_lexer::keyword;
use fajt_lexer::punct;
use fajt_lexer::token_matches;

impl Parser<'_, '_> {
    /// Parses the `AsyncFunctionDeclaration` goal symbol.
    ///
    /// Example:
    /// ```no_rust
    /// async function fn( ...args ) { return 1 };
    /// ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~^
    /// ```
    pub(super) fn parse_async_function_declaration(&mut self) -> Result<Statement> {
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
    /// function fn( a, ...args ) { return 1 };
    /// ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~^
    /// ```
    pub(super) fn parse_function_declaration(&mut self) -> Result<Statement> {
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
    /// function fn( a, ...args ) { return 1 };
    ///            ^~~~~~~~~~~~~~~~~~~~~~~~~~~^
    ///
    /// async function fn( a, ...args ) { return 1 };
    ///                  ^~~~~~~~~~~~~~~~~~~~~~~~~~~^
    /// ```
    pub(super) fn parse_function_implementation(
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
    pub(super) fn parse_formal_parameters(&mut self) -> Result<Option<FormalParameters>> {
        let span_start = self.position();
        let token = self.reader.consume()?;
        if !token_matches!(token, punct!("(")) {
            return err!(ErrorKind::UnexpectedToken(token));
        }

        if token_matches!(self.reader.current()?, punct!(")")) {
            self.reader.consume()?;
            return Ok(None);
        }

        let mut parameters = Vec::new();
        let mut rest = None;
        loop {
            match self.reader.current()? {
                token_matches!(punct!(")")) => {
                    self.reader.consume()?;
                    break;
                }
                token_matches!(punct!("...")) => {
                    rest = Some(self.parse_binding_rest_element()?);
                    self.consume_parameter_delimiter()?;
                }
                _ => {
                    parameters.push(self.parse_binding_element()?);
                    self.consume_parameter_delimiter()?;
                }
            }
        }

        let span = self.span_from(span_start);
        Ok(Some(FormalParameters {
            span,
            parameters,
            rest,
        }))
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
    pub(super) fn parse_function_body(&mut self) -> Result<Vec<Statement>> {
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
}
