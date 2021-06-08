use crate::ast::{
    ArrowFunctionExpression, BindingElement, Expression, FormalParameters, FunctionDeclaration,
    FunctionExpression, Ident, Statement,
};
use crate::error::{ErrorKind, Result};
use crate::parser::ContextModify;
use crate::Parser;
use fajt_lexer::keyword;
use fajt_lexer::punct;
use fajt_lexer::token_matches;

impl Parser<'_, '_> {
    /// Parses the `ArrowFunction` goal symbol.
    pub(super) fn parse_arrow_function_expression(&mut self) -> Result<Expression> {
        let span_start = self.position();
        let (parameters, binding_parameter) = self.parse_arrow_function_parameters()?;

        let arrow = self.reader.consume()?;
        if !token_matches!(arrow, punct!("=>")) {
            return err!(ErrorKind::UnexpectedToken(arrow));
        }

        // TODO this can also be a plain expression
        let _body = self.parse_function_body()?;

        let span = self.span_from(span_start);
        Ok(ArrowFunctionExpression {
            span,
            binding_parameter,
            parameters,
        }
        .into())
    }

    /// Parses the `ArrowParameters` goal symbol.
    /// Returns true in second tuple element if the parameters are a binding identifier without
    /// parentheses.
    fn parse_arrow_function_parameters(&mut self) -> Result<(Option<FormalParameters>, bool)> {
        let span_start = self.position();
        if self.is_identifier() {
            let identifier = self.parse_identifier()?;
            let span = self.span_from(span_start);
            return Ok((
                Some(FormalParameters {
                    span: span.clone(),
                    parameters: vec![BindingElement {
                        span,
                        pattern: identifier.into(),
                        initializer: None,
                    }],
                    rest: None,
                }),
                true,
            ));
        }

        todo!()
    }

    /// Parses the `FunctionExpression` goal symbol.
    pub(super) fn parse_function_expression(&mut self) -> Result<Expression> {
        let span_start = self.position();
        let token = self.reader.consume()?;
        debug_assert!(token_matches!(token, keyword!("function")));

        let generator = self.consume_generator_token();
        let identifier = self.parse_optional_identifier()?;
        let parameters = self.parse_formal_parameters()?;
        let body = self.parse_function_body()?;

        let span = self.span_from(span_start);
        Ok(FunctionExpression {
            span,
            asynchronous: false,
            generator,
            identifier,
            parameters,
            body,
        }
        .into())
    }

    /// Parses the `AsyncFunctionExpression` goal symbol.
    pub(super) fn parse_async_function_expression(&mut self) -> Result<Expression> {
        let span_start = self.position();
        let token = self.reader.consume()?;
        debug_assert!(token_matches!(token, keyword!("async")));

        let function_token = self.reader.consume()?;
        debug_assert!(token_matches!(function_token, keyword!("function")));
        debug_assert_eq!(function_token.first_on_line, false);

        let generator = self.consume_generator_token();
        let identifier = self.parse_optional_identifier()?;
        let parameters = self.parse_formal_parameters()?;
        let body = self.parse_function_body()?;

        let span = self.span_from(span_start);
        Ok(FunctionExpression {
            span,
            asynchronous: true,
            generator,
            identifier,
            parameters,
            body,
        }
        .into())
    }

    /// Parses the `FunctionDeclaration` goal symbol.
    pub(super) fn parse_function_declaration(&mut self) -> Result<Statement> {
        let span_start = self.position();
        let token = self.reader.consume()?;
        debug_assert!(token_matches!(token, keyword!("function")));

        let generator = self.consume_generator_token();
        let ident = self.parse_identifier()?;

        self.with_context(ContextModify::new().set_yield(false).set_await(false))
            .parse_function_implementation(span_start, ident, generator, false)
    }

    /// Parses the `AsyncFunctionDeclaration` goal symbol.
    pub(super) fn parse_async_function_declaration(&mut self) -> Result<Statement> {
        let span_start = self.position();
        let token = self.reader.consume()?;
        debug_assert!(token_matches!(token, keyword!("async")));

        let function_token = self.reader.consume()?;
        debug_assert!(token_matches!(function_token, keyword!("function")));
        debug_assert_eq!(function_token.first_on_line, false);

        let generator = self.consume_generator_token();
        let ident = self.parse_identifier()?;

        self.with_context(ContextModify::new().set_yield(false).set_await(true))
            .parse_function_implementation(span_start, ident, generator, true)
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
        generator: bool,
        asynchronous: bool,
    ) -> Result<Statement> {
        let parameters = self.parse_formal_parameters()?;
        let body = self.parse_function_body()?;

        let span = self.span_from(span_start);
        Ok(FunctionDeclaration {
            span,
            asynchronous,
            generator,
            identifier: ident,
            parameters,
            body,
        }
        .into())
    }

    /// Parses the `FormalParameters` goal symbol.
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
    pub(super) fn parse_function_body(&mut self) -> Result<Vec<Statement>> {
        let token = self.reader.consume()?;
        debug_assert!(token_matches!(token, punct!("{")));

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

    /// If the current token is '*', it is consumed and true is returned, otherwise false.
    pub(super) fn consume_generator_token(&mut self) -> bool {
        let generator = token_matches!(self.reader.current(), ok: punct!("*"));
        if generator {
            self.reader.consume().unwrap();
            true
        } else {
            false
        }
    }
}
