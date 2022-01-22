use crate::error::ErrorKind::{SyntaxError, UnexpectedToken};
use crate::error::Result;
use crate::{Parser, ThenTry};
use fajt_ast::{
    ArrayBinding, BindingElement, BindingPattern, Ident, NamedBinding, ObjectBinding,
    ObjectBindingProp, SingleNameBinding,
};
use fajt_common::io::{PeekRead, ReReadWithState};
use fajt_lexer::token::Punct::{BraceClose, BracketClose};
use fajt_lexer::token::{Punct, Token, TokenValue};
use fajt_lexer::token_matches;
use fajt_lexer::{punct, LexerState};

impl<I> Parser<'_, I>
where
    I: PeekRead<Token, Error = fajt_lexer::error::Error>,
    I: ReReadWithState<Token, State = LexerState, Error = fajt_lexer::error::Error>,
{
    /// Parses the `BindingPattern` production.
    pub(super) fn parse_binding_pattern(&mut self) -> Result<BindingPattern> {
        Ok(match self.current()? {
            token_matches!(punct!("{")) => self.parse_object_binding_pattern()?,
            token_matches!(punct!("[")) => self.parse_array_binding_pattern()?,
            _ => BindingPattern::Ident(self.parse_identifier()?),
        })
    }

    /// Parses the `ObjectBindingPattern` production.
    fn parse_object_binding_pattern(&mut self) -> Result<BindingPattern> {
        let span_start = self.position();
        self.consume_assert(&punct!("{"))?;

        let mut props = Vec::new();

        let mut rest = None;
        loop {
            match self.current()? {
                token_matches!(punct!("}")) => {
                    self.consume()?;
                    break;
                }
                token_matches!(punct!("...")) => {
                    self.consume()?;
                    rest = self.parse_rest_binding_ident(BracketClose)?;
                    break;
                }
                _ if self.peek_matches(&punct!(":")) => {
                    props.push(ObjectBindingProp::Named(self.parse_named_binding()?));

                    self.consume_object_delimiter()?;
                }
                _ if self.is_identifier() => {
                    props.push(ObjectBindingProp::Single(self.parse_single_name_binding()?));
                    self.consume_object_delimiter()?;
                }
                _ => return err!(UnexpectedToken(self.consume()?)),
            }
        }

        let span = self.span_from(span_start);
        Ok(ObjectBinding { span, props, rest }.into())
    }

    /// Parses the `SingleNameBinding` production.
    fn parse_single_name_binding(&mut self) -> Result<SingleNameBinding> {
        let span_start = self.position();

        let ident = self.parse_identifier()?;
        let initializer = self
            .current_matches(&punct!("="))
            .then_try(|| self.parse_initializer())?;

        let span = self.span_from(span_start);
        Ok(SingleNameBinding {
            span,
            ident,
            initializer,
        })
    }

    /// Parses the `PropertyName: BindingElement` case of the `BindingProperty` production.
    fn parse_named_binding(&mut self) -> Result<NamedBinding> {
        let span_start = self.position();
        let property = self.parse_property_name()?;
        self.consume_assert(&punct!(":"))?;

        let binding = self.parse_binding_element()?;

        let span = self.span_from(span_start);
        Ok(NamedBinding {
            span,
            property,
            binding,
        })
    }

    /// Parses the `ArrayBindingPattern` production.
    fn parse_array_binding_pattern(&mut self) -> Result<BindingPattern> {
        let span_start = self.position();
        self.consume_assert(&punct!("["))?;

        let mut elements = Vec::new();

        let mut rest = None;
        loop {
            match self.current()? {
                token_matches!(punct!("]")) => {
                    self.consume()?;
                    break;
                }
                token_matches!(punct!(",")) => {
                    self.consume()?;
                    elements.push(None);
                }
                token_matches!(punct!("...")) => {
                    self.consume()?;
                    rest = self.parse_rest_binding_ident(BraceClose)?;
                    break;
                }
                _ if self.is_binding_element()? => {
                    elements.push(Some(self.parse_binding_element()?));
                    self.consume_array_delimiter()?;
                }
                _ => return err!(UnexpectedToken(self.consume()?)),
            }
        }

        let span = self.span_from(span_start);
        Ok(ArrayBinding {
            span,
            elements,
            rest,
        }
        .into())
    }

    pub(super) fn is_binding_element(&self) -> Result<bool> {
        match self.current()? {
            token_matches!(punct!("{") | punct!("[")) => Ok(true),
            _ if self.is_identifier() => Ok(true),
            _ => Ok(false),
        }
    }

    /// Parses the `BindingElement` production.
    pub(super) fn parse_binding_element(&mut self) -> Result<BindingElement> {
        let span_start = self.position();
        let pattern = self.parse_binding_pattern()?;
        let initializer = self
            .current_matches(&punct!("="))
            .then_try(|| self.parse_initializer())?;

        let span = self.span_from(span_start);
        Ok(BindingElement {
            span,
            pattern,
            initializer,
        })
    }

    /// Parses the `BindingRestElement` production.
    pub(super) fn parse_binding_rest_element(&mut self) -> Result<BindingPattern> {
        self.consume_assert(&punct!("..."))?;
        self.parse_binding_pattern()
    }

    /// Parses the `BindingIdentifier` production.
    /// This also consumes the expected end punctuator.
    fn parse_rest_binding_ident(&mut self, expected_end: Punct) -> Result<Option<Ident>> {
        let ident = self.parse_identifier()?;
        let end_token = self.consume()?;

        if let TokenValue::Punct(p) = end_token.value {
            if p == expected_end {
                return Ok(Some(ident));
            }
        }

        err!(SyntaxError(
            "Rest element must be last element".to_owned(),
            ident.span,
        ))
    }
}
