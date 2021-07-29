use crate::ast::{
    ArrowFunctionBody, BindingElement, DeclFunction, Expr, ExprArrowFunction, ExprFunction,
    FormalParameters, Ident, Stmt,
};
use crate::error::Result;
use crate::parser::ContextModify;
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
    /// Parses the `ArrowFunction` goal symbol, but expects the parameters as input since that may
    /// be a non terminal before we know if it is an arrow function or parenthesized expression.
    pub(super) fn parse_arrow_function_expression(
        &mut self,
        span_start: usize,
        binding_parameter: bool,
        asynchronous: bool,
        parameters: FormalParameters,
    ) -> Result<Expr> {
        self.consume_assert(punct!("=>"))?;

        let body = if self.current_matches(punct!("{")) {
            ArrowFunctionBody::Block(self.parse_function_body()?)
        } else {
            ArrowFunctionBody::Expression(self.parse_assignment_expression()?)
        };

        let span = self.span_from(span_start);
        Ok(ExprArrowFunction {
            span,
            asynchronous,
            binding_parameter,
            parameters,
            body,
        }
        .into())
    }

    /// Parses the `ArrowParameters` goal symbol.
    /// Returns true in second tuple element if the parameters are a binding identifier without
    /// parentheses.
    pub(crate) fn parse_arrow_identifier_argument(&mut self) -> Result<FormalParameters> {
        let span_start = self.position();
        let identifier = self.parse_identifier()?;
        let span = self.span_from(span_start);
        Ok(FormalParameters {
            span: span.clone(),
            bindings: vec![BindingElement {
                span,
                pattern: identifier.into(),
                initializer: None,
            }],
            rest: None,
        })
    }

    /// Parses the `FunctionExpression` goal symbol.
    pub(super) fn parse_function_expression(&mut self) -> Result<Expr> {
        let span_start = self.position();
        self.consume_assert(keyword!("function"))?;

        let generator = self.consume_generator_token();
        let identifier = self.parse_optional_identifier()?;
        let parameters = self.parse_formal_parameters()?;
        let body = self.parse_function_body()?;

        let span = self.span_from(span_start);
        Ok(ExprFunction {
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
    pub(super) fn parse_async_function_expression(&mut self) -> Result<Expr> {
        let span_start = self.position();
        self.consume_assert(keyword!("async"))?;

        let function_token = self.consume_assert(keyword!("function"))?;
        debug_assert_eq!(function_token.first_on_line, false);

        let generator = self.consume_generator_token();
        let identifier = self.parse_optional_identifier()?;
        let parameters = self.parse_formal_parameters()?;
        let body = self.parse_function_body()?;

        let span = self.span_from(span_start);
        Ok(ExprFunction {
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
    pub(super) fn parse_function_declaration(&mut self) -> Result<Stmt> {
        let span_start = self.position();
        self.consume_assert(keyword!("function"))?;

        let generator = self.consume_generator_token();
        let ident = self.parse_identifier()?;

        self.with_context(ContextModify::new().set_yield(false).set_await(false))
            .parse_function_implementation(span_start, ident, generator, false)
    }

    /// Parses the `AsyncFunctionDeclaration` goal symbol.
    pub(super) fn parse_async_function_declaration(&mut self) -> Result<Stmt> {
        let span_start = self.position();
        self.consume_assert(keyword!("async"))?;

        let function_token = self.consume_assert(keyword!("function"))?;
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
    ) -> Result<Stmt> {
        let parameters = self.parse_formal_parameters()?;
        let body = self.parse_function_body()?;

        let span = self.span_from(span_start);
        Ok(DeclFunction {
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
    pub(crate) fn parse_formal_parameters(&mut self) -> Result<FormalParameters> {
        let span_start = self.position();

        self.consume_assert(punct!("("))?;

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
        Ok(FormalParameters {
            span,
            bindings: parameters,
            rest,
        })
    }

    /// Parses the `FunctionBody` or `AsyncFunctionBody` goal symbol.
    pub(super) fn parse_function_body(&mut self) -> Result<Vec<Stmt>> {
        self.consume_assert(punct!("{"))?;

        let mut body = Vec::new();
        loop {
            if self.current_matches(punct!("}")) {
                self.reader.consume()?;
                break;
            }

            body.push(self.parse_statement()?);
        }
        Ok(body)
    }

    /// If the current token is '*', it is consumed and true is returned, otherwise false.
    pub(super) fn consume_generator_token(&mut self) -> bool {
        let generator = self.current_matches(punct!("*"));
        if generator {
            self.reader.consume().unwrap();
            true
        } else {
            false
        }
    }
}
