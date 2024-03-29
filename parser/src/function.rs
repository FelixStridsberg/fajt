use crate::error::Result;
use crate::{DirectivePrologueSemantics, Error, Parser, ThenTry};
use fajt_ast::{
    ArrowFunctionBody, BindingElement, Body, DeclFunction, Expr, ExprArrowFunction, ExprFunction,
    FormalParameters, Ident, Stmt,
};
use fajt_common::io::{PeekRead, ReReadWithState};
use fajt_lexer::punct;
use fajt_lexer::token::Token;
use fajt_lexer::token_matches;
use fajt_lexer::{keyword, LexerState};

impl<I> Parser<'_, I>
where
    I: PeekRead<Token, Error = fajt_lexer::error::Error>,
    I: ReReadWithState<Token, State = LexerState, Error = fajt_lexer::error::Error>,
{
    /// Parses the `ArrowFunction` production.
    pub(super) fn parse_arrow_function_expr(&mut self) -> Result<Expr> {
        let asynchronous = false;
        let span_start = self.position();
        self.parse_arrow_function(span_start, asynchronous)
    }

    /// Parses the async version of `ArrowFunction` production.
    pub(super) fn parse_async_arrow_function_expr(&mut self) -> Result<Expr> {
        let asynchronous = true;
        let span_start = self.position();
        self.consume_assert(&keyword!("async"))?;
        self.parse_arrow_function(span_start, asynchronous)
    }

    pub(super) fn parse_arrow_function(
        &mut self,
        span_start: usize,
        asynchronous: bool,
    ) -> Result<Expr> {
        let (binding_parameter, parameters) = self.parse_arrow_function_parameters()?;

        let arrow = self.consume_assert(&punct!("=>"))?;
        if arrow.first_on_line {
            return Err(Error::unexpected_token(arrow));
        }

        let body = self
            .with_context(
                self.context
                    .reset_parameters()
                    .with_in(self.context.is_in)
                    .with_await(asynchronous),
            )
            .parse_concise_body()?;

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

    /// Parses the `ArrowParameters` production. First item in the tuple is `true` if parameters are
    /// a `BindingIdentifier`.
    fn parse_arrow_function_parameters(&mut self) -> Result<(bool, FormalParameters)> {
        let (binding_parameter, parameters) = if self.is_identifier() {
            (true, self.parse_arrow_identifier_argument()?)
        } else {
            (false, self.parse_formal_parameters()?)
        };

        Ok((binding_parameter, parameters))
    }

    /// Parses the `ConciseBody` production.
    fn parse_concise_body(&mut self) -> Result<ArrowFunctionBody> {
        if self.current_matches(&punct!("{")) {
            Ok(ArrowFunctionBody::Body(self.parse_function_body()?))
        } else {
            Ok(ArrowFunctionBody::Expr(
                self.parse_assignment_expr()?.into(),
            ))
        }
    }

    /// Parses the `ArrowParameters` production.
    pub(super) fn parse_arrow_identifier_argument(&mut self) -> Result<FormalParameters> {
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

    /// Parses the `FunctionExpression` production.
    pub(super) fn parse_function_expr(&mut self) -> Result<Expr> {
        let span_start = self.position();
        self.consume_assert(&keyword!("function"))?;

        let generator = self.maybe_consume(&punct!("*"))?;
        self.with_context(
            self.context
                .with_yield(generator)
                .with_await(false)
                .with_in_method(false)
                .with_super_call_allowed(false),
        )
        .parse_function_expr_content(span_start)
    }

    /// Parses the `AsyncFunctionExpression` production.
    pub(super) fn parse_async_function_expr(&mut self) -> Result<Expr> {
        let span_start = self.position();
        self.consume_assert(&keyword!("async"))?;

        let function_token = self.consume_assert(&keyword!("function"))?;
        debug_assert!(!function_token.first_on_line);

        let generator = self.maybe_consume(&punct!("*"))?;

        self.with_context(
            self.context
                .with_yield(generator)
                .with_await(true)
                .with_in_method(false)
                .with_super_call_allowed(false),
        )
        .parse_function_expr_content(span_start)
    }

    /// Parses the parts from the optional identifier and forward for async/non-async
    /// function/generator expressions, assumes context is set correctly.
    fn parse_function_expr_content(&mut self, span_start: usize) -> Result<Expr> {
        let identifier = self.is_identifier().then_try(|| self.parse_identifier())?;
        let parameters = self.parse_formal_parameters()?;
        let body = self.parse_function_body()?;

        let span = self.span_from(span_start);
        Ok(ExprFunction {
            span,
            asynchronous: self.context.is_await,
            generator: self.context.is_yield,
            identifier,
            parameters,
            body,
        }
        .into())
    }

    /// Parses the `FunctionDeclaration` production.
    pub(super) fn parse_function_declaration(&mut self) -> Result<Stmt> {
        let span_start = self.position();
        self.consume_assert(&keyword!("function"))?;

        let generator = self.maybe_consume(&punct!("*"))?;
        let ident = self.parse_function_identifier()?;

        self.with_context(
            self.context
                .with_yield(generator)
                .with_await(false)
                .with_in_method(false)
                .with_super_call_allowed(false),
        )
        .parse_function_decl_content(span_start, ident)
    }

    /// Parses the `AsyncFunctionDeclaration` production.
    pub(super) fn parse_async_function_declaration(&mut self) -> Result<Stmt> {
        let span_start = self.position();
        self.consume_assert(&keyword!("async"))?;

        let function_token = self.consume_assert(&keyword!("function"))?;
        debug_assert!(!function_token.first_on_line);

        let generator = self.maybe_consume(&punct!("*"))?;
        let ident = self.parse_function_identifier()?;

        self.with_context(
            self.context
                .with_yield(generator)
                .with_await(true)
                .with_in_method(false)
                .with_super_call_allowed(false),
        )
        .parse_function_decl_content(span_start, ident)
    }

    /// Parses the parts after the identifier and forward async/non-async function/generator
    /// declarations, assumes context is set correctly.
    pub(super) fn parse_function_decl_content(
        &mut self,
        span_start: usize,
        identifier: Ident,
    ) -> Result<Stmt> {
        let parameters = self.parse_formal_parameters()?;
        let body = self.parse_function_body()?;

        let span = self.span_from(span_start);
        Ok(DeclFunction {
            span,
            asynchronous: self.context.is_await,
            generator: self.context.is_yield,
            identifier,
            parameters,
            body,
        }
        .into())
    }

    /// Parses the name of a function. If in `default` (export default) context, the ident may be
    /// empty/non existent in which case a dummy identifier is returned.
    fn parse_function_identifier(&mut self) -> Result<Ident> {
        // In `default` context the identifier is optional.
        if self.context.is_default && self.current_matches(&punct!("(")) {
            let current = self.current().unwrap();
            Ok(Ident::dummy(current.span.start))
        } else {
            self.parse_identifier()
        }
    }

    /// Parses the `FormalParameters` production.
    pub(super) fn parse_formal_parameters(&mut self) -> Result<FormalParameters> {
        let span_start = self.position();

        self.consume_assert(&punct!("("))?;

        let mut parameters = Vec::new();
        let mut rest = None;
        loop {
            match self.current()? {
                token_matches!(punct!(")")) => {
                    self.consume()?;
                    break;
                }
                token_matches!(punct!("...")) => {
                    rest = Some(self.parse_binding_rest_element()?);
                    self.consume_assert(&punct!(")"))?;
                    break;
                }
                _ => {
                    parameters.push(self.parse_binding_element()?);
                    self.consume_list_delimiter(&punct!(")"))?;
                }
            }
        }

        let span = self.span_from(span_start);
        Ok(FormalParameters {
            span,
            bindings: parameters,
            rest: rest.map(Box::new),
        })
    }

    /// Parses the `FunctionBody` or `AsyncFunctionBody` production.
    pub(super) fn parse_function_body(&mut self) -> Result<Body> {
        let span_start = self.position();
        self.consume_assert(&punct!("{"))?;

        let directives = self.parse_directive_prologue()?;
        let is_strict = self.context.is_strict || directives.as_slice().contains_strict();

        let statements = self
            .with_context(self.context.with_strict(is_strict).with_return(true))
            .parse_function_body_stmt_list()?;

        let span = self.span_from(span_start);
        Ok(Body {
            span,
            directives,
            statements,
        })
    }

    fn parse_function_body_stmt_list(&mut self) -> Result<Vec<Stmt>> {
        let mut statements = Vec::new();
        loop {
            if self.maybe_consume(&punct!("}"))? {
                break;
            }

            statements.push(self.parse_declaration_or_statement()?);
        }
        Ok(statements)
    }
}
