use crate::ast::{
    Array, ArrayElement, Expression, Literal, LiteralExpression, Object, PropertyDefinition,
};
use crate::error::ErrorKind::UnexpectedToken;
use crate::error::Result;
use crate::Parser;
use fajt_lexer::punct;
use fajt_lexer::token::TokenValue;
use fajt_lexer::token_matches;

impl Parser<'_, '_> {
    /// Consumes a know literal token from the reader and returns an expression of it.
    pub(super) fn consume_literal(&mut self, literal: Literal) -> Result<Expression> {
        let token = self.reader.consume()?;
        Ok(LiteralExpression {
            span: token.span,
            literal,
        }
        .into())
    }

    /// Parses the `Literal` goal symbol.
    pub(super) fn parse_literal(&mut self) -> Result<Expression> {
        let token = self.reader.consume()?;
        debug_assert!(token_matches!(token, @literal));

        if let TokenValue::Literal(literal) = token.value {
            Ok(LiteralExpression {
                span: token.span,
                literal: literal.into(),
            }
            .into())
        } else {
            unreachable!()
        }
    }

    /// Parses the `ArrayLiteral` goal symbol.
    pub(super) fn parse_array_literal(&mut self) -> Result<Expression> {
        let span_start = self.position();
        let token = self.reader.consume()?;
        debug_assert!(token_matches!(token, punct!("[")));

        let mut elements = Vec::new();
        loop {
            match self.reader.current()? {
                token_matches!(punct!("]")) => {
                    self.reader.consume()?;
                    break;
                }
                token_matches!(punct!(",")) => {
                    self.reader.consume()?;
                    elements.push(ArrayElement::None);
                }
                token_matches!(punct!("...")) => {
                    self.reader.consume()?;
                    let expr = self.parse_expression()?;
                    elements.push(ArrayElement::Spread(expr));
                    self.consume_array_delimiter()?;
                }
                _ => {
                    let expr = self.parse_expression()?;
                    elements.push(ArrayElement::Expr(expr));
                    self.consume_array_delimiter()?;
                }
            }
        }

        let span = self.span_from(span_start);
        Ok(LiteralExpression {
            span,
            literal: Literal::Array(Array { elements }),
        }
        .into())
    }

    /// Parses the `ObjectLiteral` goal symbol.
    pub(super) fn parse_object_literal(&mut self) -> Result<Expression> {
        let span_start = self.position();
        let token = self.reader.consume()?;
        debug_assert!(token_matches!(token, punct!("{")));

        let mut props = Vec::new();
        loop {
            match self.reader.current()? {
                token_matches!(punct!("}")) => {
                    self.reader.consume()?;
                    break;
                }
                token_matches!(punct!("...")) => {
                    self.reader.consume()?;
                    let expr = self.parse_expression()?;
                    props.push(PropertyDefinition::Spread(expr));
                    self.consume_object_delimiter()?;
                }
                _ if self.is_identifier()? => {
                    let ident = self.parse_identifier()?;
                    props.push(PropertyDefinition::IdentifierReference(ident.into()));
                    self.consume_object_delimiter()?;
                }
                _ => return err!(UnexpectedToken(self.reader.consume()?)),
            }
        }

        let span = self.span_from(span_start);
        Ok(LiteralExpression {
            span,
            literal: Literal::Object(Object { props }),
        }
        .into())
    }
}
