use crate::error::Result;
use crate::static_semantics::FormalParametersSemantics;
use crate::Parser;
use fajt_ast::{MethodDefinition, MethodKind, PropertyName};
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
    /// Parses the `MethodDefinition` production.
    pub(super) fn parse_method_definition(&mut self) -> Result<MethodDefinition> {
        let is_static = self.context.static_method_allowed
            && !self.peek_matches(&punct!("("))
            && self.maybe_consume(&keyword!("static"))?;

        match self.current()? {
            token_matches!(punct!("*")) => self.parse_generator_method(is_static),
            token_matches!(keyword!("get")) if !self.peek_matches(&punct!("(")) => {
                self.parse_getter_or_setter(is_static, MethodKind::Get)
            }
            token_matches!(keyword!("set")) if !self.peek_matches(&punct!("(")) => {
                self.parse_getter_or_setter(is_static, MethodKind::Set)
            }
            token_matches!(keyword!("async")) if !self.followed_by_new_line() => {
                self.parse_async_method(is_static)
            }
            _ => {
                let span_start = self.position();
                let name = self.parse_property_name()?;
                self.parse_method(span_start, is_static, name, MethodKind::Method)
            }
        }
    }

    /// Parses the `GeneratorMethod` production.
    fn parse_generator_method(&mut self, is_static: bool) -> Result<MethodDefinition> {
        let span_start = self.position();
        self.consume_assert(&punct!("*"))?;

        let name = self.parse_property_name()?;
        self.with_context(self.context.with_yield(true).with_await(false))
            .parse_method(span_start, is_static, name, MethodKind::Method)
    }

    fn parse_getter_or_setter(
        &mut self,
        is_static: bool,
        kind: MethodKind,
    ) -> Result<MethodDefinition> {
        let span_start = self.position();
        self.consume()?;

        let name = self.parse_property_name()?;
        self.parse_method(span_start, is_static, name, kind)
    }

    /// Parses the `AsyncMethod` and `AsyncGeneratorMethod` production.
    fn parse_async_method(&mut self, is_static: bool) -> Result<MethodDefinition> {
        let span_start = self.position();
        self.consume_assert(&keyword!("async"))?;

        let generator = self.maybe_consume(&punct!("*"))?;
        let name = self.parse_property_name()?;
        self.with_context(self.context.with_yield(generator).with_await(true))
            .parse_method(span_start, is_static, name, MethodKind::Method)
    }

    pub(super) fn parse_method(
        &mut self,
        span_start: usize,
        is_static: bool,
        name: PropertyName,
        kind: MethodKind,
    ) -> Result<MethodDefinition> {
        let parameters = self
            .with_context(self.context.with_yield(false))
            .parse_formal_parameters()?;
        let body = self.parse_function_body()?;

        match kind {
            MethodKind::Method => parameters.early_errors_method(&body.directives)?,
            MethodKind::Get => parameters.early_errors_getter()?,
            MethodKind::Set => {
                parameters.early_errors_setter(&body.directives)?
                // TODO validate if bound names of set list is in `LexicallyDeclaredNames` of body
            }
        }

        let span = self.span_from(span_start);
        Ok(MethodDefinition {
            span,
            name,
            kind,
            parameters,
            body,
            is_static,
            generator: self.context.is_yield,
            asynchronous: self.context.is_await,
        })
    }
}
