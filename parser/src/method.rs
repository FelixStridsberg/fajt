use crate::error::Result;
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
        match self.current()? {
            token_matches!(punct!("*")) => self.parse_generator_method(),
            token_matches!(keyword!("get")) => self.parse_getter_or_setter(MethodKind::Get),
            token_matches!(keyword!("set")) => self.parse_getter_or_setter(MethodKind::Set),
            token_matches!(keyword!("async")) if !self.followed_by_new_line() => {
                self.parse_async_method()
            }
            _ => {
                let span_start = self.position();
                let name = self.parse_property_name()?;
                self.parse_method(span_start, name, MethodKind::Method)
            }
        }
    }

    /// Parses the `GeneratorMethod` production.
    fn parse_generator_method(&mut self) -> Result<MethodDefinition> {
        let span_start = self.position();
        self.consume_assert(&punct!("*"))?;

        let name = self.parse_property_name()?;
        self.with_context(self.context.with_yield(true).with_await(false))
            .parse_method(span_start, name, MethodKind::Method)
    }

    fn parse_getter_or_setter(&mut self, kind: MethodKind) -> Result<MethodDefinition> {
        let span_start = self.position();
        self.consume()?;

        let name = self.parse_property_name()?;
        self.parse_method(span_start, name, kind)
    }

    /// Parses the `AsyncMethod` and `AsyncGeneratorMethod` production.
    fn parse_async_method(&mut self) -> Result<MethodDefinition> {
        let span_start = self.position();
        self.consume_assert(&keyword!("async"))?;

        let generator = self.maybe_consume(&punct!("*"))?;
        let name = self.parse_property_name()?;
        self.with_context(self.context.with_yield(generator).with_await(true))
            .parse_method(span_start, name, MethodKind::Method)
    }

    pub(super) fn parse_method(
        &mut self,
        span_start: usize,
        name: PropertyName,
        kind: MethodKind,
    ) -> Result<MethodDefinition> {
        // TODO this should be `UniqueFormalParameters` or `PropertySetParameterList` depending on kind.
        let parameters = self.parse_formal_parameters()?;
        let body = self.parse_function_body()?;

        let span = self.span_from(span_start);
        Ok(MethodDefinition {
            span,
            name,
            kind,
            parameters,
            body,
            generator: self.context.is_yield,
            asynchronous: self.context.is_await,
        })
    }
}
