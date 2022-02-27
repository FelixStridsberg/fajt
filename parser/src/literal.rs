use crate::error::Result;
use crate::{Error, Parser};
use fajt_ast::{
    ArrayElement, Expr, ExprLiteral, LitArray, LitObject, LitTemplate, Literal, MethodKind,
    NamedProperty, PropertyDefinition, TemplatePart,
};
use fajt_common::io::{PeekRead, ReReadWithState};
use fajt_lexer::punct;
use fajt_lexer::token::{Token, TokenValue};
use fajt_lexer::token_matches;
use fajt_lexer::{keyword, LexerState};

impl<I> Parser<'_, I>
where
    I: PeekRead<Token, Error = fajt_lexer::error::Error>,
    I: ReReadWithState<Token, State = LexerState, Error = fajt_lexer::error::Error>,
{
    /// Consumes a know literal token from the reader and returns an expression of it.
    pub(super) fn consume_literal(&mut self, literal: Literal) -> Result<Expr> {
        let token = self.consume()?;
        Ok(ExprLiteral {
            span: token.span,
            literal,
        }
        .into())
    }

    /// Parses the `Literal` production.
    pub(super) fn parse_literal(&mut self) -> Result<Expr> {
        let token = self.consume()?;
        debug_assert!(token_matches!(token, @literal));

        if let TokenValue::Literal(literal) = token.value {
            Ok(ExprLiteral {
                span: token.span,
                literal,
            }
            .into())
        } else {
            unreachable!()
        }
    }

    /// Parses the `TemplateLiteral` production.
    pub(super) fn parse_template_literal_expr(&mut self) -> Result<Expr> {
        let span_start = self.position();
        let template = self.parse_template_literal()?;

        let span = self.span_from(span_start);
        Ok(ExprLiteral {
            span,
            literal: Literal::Template(template),
        }
        .into())
    }

    pub(super) fn parse_template_literal(&mut self) -> Result<LitTemplate> {
        let head = self.consume()?;
        let head_str = match head.value {
            TokenValue::Literal(Literal::Template(template)) => return Ok(template),
            TokenValue::TemplateHead(string) => string,
            _ => return Err(Error::unexpected_token(head)),
        };

        let mut parts = vec![];

        if !head_str.is_empty() {
            parts.push(TemplatePart::String(head_str));
        }

        loop {
            parts.push(TemplatePart::Expr(Box::new(self.parse_expr()?)));
            self.reader
                .reread_with_state(LexerState::inside_template())?;

            let token = self.consume()?;
            match token.value {
                TokenValue::TemplateMiddle(middle) => {
                    if !middle.is_empty() {
                        parts.push(TemplatePart::String(middle));
                    }
                }
                TokenValue::TemplateTail(tail) => {
                    if !tail.is_empty() {
                        parts.push(TemplatePart::String(tail));
                    }
                    break;
                }
                _ => return Err(Error::unexpected_token(token)),
            }
        }

        Ok(LitTemplate { parts })
    }

    /// Parses the `ArrayLiteral` production.
    pub(super) fn parse_array_literal(&mut self) -> Result<Expr> {
        let span_start = self.position();
        self.consume_assert(&punct!("["))?;

        let mut elements = Vec::new();
        loop {
            if self.maybe_consume(&punct!("]"))? {
                break;
            }

            match self.current()? {
                token_matches!(punct!(",")) => {
                    self.consume()?;
                    elements.push(ArrayElement::Elision);
                }
                token_matches!(punct!("...")) => {
                    self.consume()?;
                    let expr = self.parse_assignment_expr()?;
                    elements.push(ArrayElement::Spread(expr));
                    self.consume_list_delimiter(&punct!("]"))?;
                }
                _ => {
                    let expr = self.parse_assignment_expr()?;
                    elements.push(ArrayElement::Expr(expr));
                    self.consume_list_delimiter(&punct!("]"))?;
                }
            }
        }

        let span = self.span_from(span_start);
        Ok(ExprLiteral {
            span,
            literal: Literal::Array(LitArray { elements }),
        }
        .into())
    }

    /// Parses the `ObjectLiteral` production.
    pub(super) fn parse_object_literal(&mut self) -> Result<Expr> {
        let span_start = self.position();
        self.consume_assert(&punct!("{"))?;

        let mut props = Vec::new();
        loop {
            if self.maybe_consume(&punct!("}"))? {
                break;
            }

            props.push(self.parse_property_definition()?);
            self.consume_list_delimiter(&punct!("}"))?;
        }

        let span = self.span_from(span_start);
        Ok(ExprLiteral {
            span,
            literal: Literal::Object(LitObject { props }),
        }
        .into())
    }

    /// Parses the `PropertyDefinition` production.
    fn parse_property_definition(&mut self) -> Result<PropertyDefinition> {
        match self.current()? {
            token_matches!(punct!("...")) => {
                self.consume()?;
                let expr = self.parse_assignment_expr()?;
                Ok(PropertyDefinition::Spread(expr))
            }
            _ if self.peek_matches(&punct!(":")) => Ok(self.parse_named_property_definition()?),
            token_matches!(punct!("[")) => {
                let span_start = self.position();
                let name = self.parse_property_name()?;

                if self.maybe_consume(&punct!(":"))? {
                    let value = self.parse_assignment_expr()?;
                    let span = self.span_from(span_start);
                    return Ok(PropertyDefinition::Named(NamedProperty {
                        span,
                        name,
                        value,
                    }));
                }

                Ok(PropertyDefinition::Method(self.parse_method(
                    span_start,
                    name,
                    MethodKind::Method,
                )?))
            }
            _ if self.is_object_method_definition() => {
                let method = self
                    .with_context(
                        self.context
                            .with_in_method(true)
                            .with_super_call_allowed(false),
                    )
                    .parse_method_definition()?;
                Ok(PropertyDefinition::Method(method))
            }
            // TODO CoverInitializedName
            _ => {
                let ident = self.parse_identifier()?;
                Ok(PropertyDefinition::IdentRef(ident))
            }
        }
    }

    pub fn is_object_method_definition(&self) -> bool {
        match self.current() {
            token_matches!(ok: punct!("*") | punct!("[")) => true,
            token_matches!(ok: keyword!("async")) if !self.peek_matches(&punct!(":")) => true,
            token_matches!(ok: keyword!("get") | keyword!("set")) => true,
            _ => self.peek_matches(&punct!("(")),
        }
    }

    fn parse_named_property_definition(&mut self) -> Result<PropertyDefinition> {
        let span_start = self.position();

        let name = self.parse_property_name()?;
        self.consume_assert(&punct!(":"))?;
        let value = self.parse_assignment_expr()?;

        let span = self.span_from(span_start);
        Ok(PropertyDefinition::Named(NamedProperty {
            span,
            name,
            value,
        }))
    }
}
