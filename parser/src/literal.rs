use crate::error::Result;
use crate::{Error, Parser};
use fajt_ast::{
    ArrayElement, Expr, ExprLiteral, LitArray, LitObject, LitTemplate, Literal, MethodKind,
    NamedProperty, PropertyDefinition, PropertyName, TemplatePart,
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
        if let TokenValue::Literal(literal) = token.value {
            Ok(ExprLiteral {
                span: token.span,
                literal,
            }
            .into())
        } else {
            Err(Error::unexpected_token(token))
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

    /// Parses the `Template` production.
    pub(super) fn parse_template_literal(&mut self) -> Result<LitTemplate> {
        if token_matches!(self.current()?, @literal) {
            return self.parse_non_substitution_template();
        }

        let mut parts = Vec::new();

        let head_str = self.parse_template_literal_head_string()?;
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

    /// Parses the `NonSubstitutionTemplate` production.
    fn parse_non_substitution_template(&mut self) -> Result<LitTemplate> {
        let template = self.consume()?;
        if let TokenValue::Literal(Literal::Template(template)) = template.value {
            Ok(template)
        } else {
            Err(Error::unexpected_token(template))
        }
    }

    /// Parses the `TemplateHead` production.
    fn parse_template_literal_head_string(&mut self) -> Result<String> {
        let head = self.consume()?;
        match head.value {
            TokenValue::TemplateHead(string) => Ok(string),
            _ => Err(Error::unexpected_token(head)),
        }
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

            let element = self.parse_array_element()?;
            if element != ArrayElement::Elision {
                self.consume_list_delimiter(&punct!("]"))?;
            }

            elements.push(element);
        }

        let span = self.span_from(span_start);
        Ok(ExprLiteral {
            span,
            literal: Literal::Array(LitArray { elements }),
        }
        .into())
    }

    fn parse_array_element(&mut self) -> Result<ArrayElement> {
        match self.current()? {
            token_matches!(punct!(",")) => {
                self.consume()?;
                Ok(ArrayElement::Elision)
            }
            token_matches!(punct!("...")) => {
                self.consume()?;
                let expr = self.parse_assignment_expr()?;
                Ok(ArrayElement::Spread(expr))
            }
            _ => {
                let expr = self.parse_assignment_expr()?;
                Ok(ArrayElement::Expr(expr))
            }
        }
    }

    /// Parses the `ObjectLiteral` production.
    pub(super) fn parse_object_literal(&mut self) -> Result<Expr> {
        let span_start = self.position();
        let start_token = self.consume_assert(&punct!("{"))?;

        let mut props = Vec::new();
        loop {
            if self.maybe_consume(&punct!("}"))? {
                break;
            }

            let prop = self.parse_property_definition()?;

            // If we hit an `ident` followed by a `=` in this context, we are either parsing the left
            // side of an object assignment, or this is illegal syntax. It's covered by the
            // `CoverInitializedName` production. Here we assume we are in an `ObjectAssignmentPattern`
            // since that is the only legal case.
            if self.current_matches(&punct!("="))
                && matches!(&prop, PropertyDefinition::IdentRef(_))
            {
                self.reader.rewind_to(&start_token)?;
                return Ok(Expr::ObjectBinding(self.parse_object_binding_pattern()?));
            }

            props.push(prop);

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
            _ if self.peek_matches(&punct!(":")) => {
                let span_start = self.position();
                let name = self.parse_property_name()?;
                self.parse_named_property_definition(span_start, name)
            }
            token_matches!(punct!("[")) => self.parse_property_definition_with_computed_name(),
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
            _ => {
                let ident = self.parse_identifier()?;
                Ok(PropertyDefinition::IdentRef(ident))
            }
        }
    }

    fn parse_property_definition_with_computed_name(&mut self) -> Result<PropertyDefinition> {
        let span_start = self.position();
        let name = self.parse_property_name()?;

        if self.current_matches(&punct!(":")) {
            self.parse_named_property_definition(span_start, name)
        } else {
            self.parse_method(span_start, name, MethodKind::Method)
                .map(PropertyDefinition::Method)
        }
    }

    fn parse_named_property_definition(
        &mut self,
        span_start: usize,
        name: PropertyName,
    ) -> Result<PropertyDefinition> {
        self.consume_assert(&punct!(":"))?;
        let value = self.parse_assignment_expr()?;

        let span = self.span_from(span_start);
        Ok(PropertyDefinition::Named(NamedProperty {
            span,
            name,
            value,
        }))
    }

    pub fn is_object_method_definition(&self) -> bool {
        match self.current() {
            token_matches!(ok: punct!("*") | punct!("[")) => true,
            token_matches!(ok: keyword!("async")) if !self.peek_matches(&punct!(":")) => true,
            token_matches!(ok: keyword!("get") | keyword!("set")) => true,
            _ => self.peek_matches(&punct!("(")),
        }
    }
}
