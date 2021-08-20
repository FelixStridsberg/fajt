use crate::error::ErrorKind::UnexpectedToken;
use crate::error::Result;
use crate::Parser;
use fajt_ast::{Array, ArrayElement, Expr, ExprLiteral, Literal, Object, PropertyDefinition};
use fajt_common::io::PeekRead;
use fajt_lexer::punct;
use fajt_lexer::token::{Token, TokenValue};
use fajt_lexer::token_matches;

impl<I> Parser<'_, I>
where
    I: PeekRead<Token, Error = fajt_lexer::error::Error>,
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

    /// Parses the `Literal` goal symbol.
    pub(super) fn parse_literal(&mut self) -> Result<Expr> {
        let token = self.consume()?;
        debug_assert!(token_matches!(token, @literal));

        if let TokenValue::Literal(literal) = token.value {
            Ok(ExprLiteral {
                span: token.span,
                literal: literal.into(),
            }
            .into())
        } else {
            unreachable!()
        }
    }

    /// Parses the `ArrayLiteral` goal symbol.
    pub(super) fn parse_array_literal(&mut self) -> Result<Expr> {
        let span_start = self.position();
        let token = self.consume()?;
        debug_assert!(token_matches!(token, punct!("[")));

        let mut elements = Vec::new();
        loop {
            match self.current()? {
                token_matches!(punct!("]")) => {
                    self.consume()?;
                    break;
                }
                token_matches!(punct!(",")) => {
                    self.consume()?;
                    elements.push(ArrayElement::None);
                }
                token_matches!(punct!("...")) => {
                    self.consume()?;
                    let expr = self.parse_assignment_expr()?;
                    elements.push(ArrayElement::Spread(expr));
                    self.consume_array_delimiter()?;
                }
                _ => {
                    let expr = self.parse_assignment_expr()?;
                    elements.push(ArrayElement::Expr(expr));
                    self.consume_array_delimiter()?;
                }
            }
        }

        let span = self.span_from(span_start);
        Ok(ExprLiteral {
            span,
            literal: Literal::Array(Array { elements }),
        }
        .into())
    }

    /// Parses the `ObjectLiteral` goal symbol.
    pub(super) fn parse_object_literal(&mut self) -> Result<Expr> {
        let span_start = self.position();
        let token = self.consume()?;
        debug_assert!(token_matches!(token, punct!("{")));

        let mut props = Vec::new();
        loop {
            match self.current()? {
                token_matches!(punct!("}")) => {
                    self.consume()?;
                    break;
                }
                token_matches!(punct!("...")) => {
                    self.consume()?;
                    let expr = self.parse_assignment_expr()?;
                    props.push(PropertyDefinition::Spread(expr));
                    self.consume_object_delimiter()?;
                }
                _ if self.is_identifier() => {
                    let ident = self.parse_identifier()?;
                    props.push(PropertyDefinition::IdentRef(ident));
                    self.consume_object_delimiter()?;
                }
                _ => return err!(UnexpectedToken(self.consume()?)),
            }
        }

        let span = self.span_from(span_start);
        Ok(ExprLiteral {
            span,
            literal: Literal::Object(Object { props }),
        }
        .into())
    }
}
