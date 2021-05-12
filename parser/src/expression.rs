use crate::ast::{
    Expression, Ident, Literal, LiteralExpression, Object, PropertyDefinition, ThisExpression,
};
use crate::error::{Error, Result};
use crate::Parser;

use crate::ast::Array;
use crate::ast::Expression::IdentifierReference;
use crate::error::ErrorKind::UnexpectedToken;
use fajt_lexer::keyword;
use fajt_lexer::punct;
use fajt_lexer::token::{Span, TokenValue};
use fajt_lexer::token_matches;
use std::convert::TryInto;

impl Parser<'_> {
    pub(crate) fn parse_expression(&mut self) -> Result<Expression> {
        self.parse_assignment_expression()
    }

    pub(crate) fn parse_assignment_expression(&mut self) -> Result<Expression> {
        // TODO other than primary expressions
        self.parse_primary_expression()
    }

    fn parse_primary_expression(&mut self) -> Result<Expression> {
        Ok(match self.reader.current()? {
            token_matches!(keyword!("this")) => self.parse_this_expression()?,
            token_matches!(keyword!("yield")) => unimplemented!(),
            token_matches!(keyword!("await")) => unimplemented!(),
            token_matches!(keyword!("null")) => self.consume_literal(Literal::Null)?,
            token_matches!(keyword!("true")) => self.consume_literal(Literal::Boolean(true))?,
            token_matches!(keyword!("false")) => self.consume_literal(Literal::Boolean(false))?,
            token_matches!(@literal) => self.parse_literal()?,
            token_matches!(punct!("[")) => self.parse_array_literal()?,
            token_matches!(punct!("{")) => self.parse_object_literal()?,
            // TODO FunctionExpression
            // TODO ClassExpression
            // TODO GeneratorExpression
            // TODO AsyncFunctionExpression
            // TODO AsyncGeneratorExpression
            // TODO RegularExpressionLiteral
            // TODO TemplateLiteral
            // TODO CoverParenthesizedExpressionAndArrowParameterList
            token_matches!(@ident) => self.parse_identifier_reference()?,
            r => unimplemented!("TOKEN: {:?}", r),
        })
    }

    fn consume_literal(&mut self, literal: Literal) -> Result<Expression> {
        let token = self.reader.consume()?;
        Ok(LiteralExpression {
            span: token.span,
            literal,
        }
        .into())
    }

    fn parse_literal(&mut self) -> Result<Expression> {
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

    fn parse_array_literal(&mut self) -> Result<Expression> {
        let token = self.reader.consume()?;
        debug_assert!(token_matches!(token, punct!("[")));

        let span_start = token.span.start;

        let mut elements = Vec::new();
        let mut comma_delimiter = false;
        loop {
            match self.reader.current()? {
                token_matches!(punct!("]")) => {
                    self.reader.consume()?;
                    break;
                }
                token_matches!(punct!(",")) => {
                    self.reader.consume()?;

                    if !comma_delimiter {
                        elements.push(None);
                    }
                    comma_delimiter = false;
                }
                _ => {
                    let expr = self.parse_expression()?;
                    elements.push(Some(expr));

                    comma_delimiter = true;
                }
            }
        }

        let span_end = self.reader.position();
        let span = Span::new(span_start, span_end);
        Ok(LiteralExpression {
            span,
            literal: Literal::Array(Array::new(elements)),
        }
        .into())
    }

    fn parse_object_literal(&mut self) -> Result<Expression> {
        let token = self.reader.consume()?;
        debug_assert!(token_matches!(token, punct!("{")));

        let span_start = token.span.start;

        let mut comma_allowed = false;
        let mut props = Vec::new();
        loop {
            match self.reader.current()? {
                token_matches!(punct!("}")) => {
                    self.reader.consume()?;
                    break;
                }
                token_matches!(punct!(",")) if comma_allowed => {
                    self.reader.consume()?;
                    comma_allowed = false;
                }
                token_matches!(@ident) => {
                    let ident: Ident = self.reader.consume()?.try_into()?;
                    props.push(PropertyDefinition::IdentifierReference(ident.into()));

                    comma_allowed = true;
                }
                _ => return Err(Error::of(UnexpectedToken(self.reader.consume()?))),
            }
        }

        let span_end = self.reader.position();
        let span = Span::new(span_start, span_end);
        Ok(LiteralExpression {
            span,
            literal: Literal::Object(Object { props }),
        }
        .into())
    }

    fn parse_this_expression(&mut self) -> Result<Expression> {
        let token = self.reader.consume()?;
        Ok(ThisExpression::new(token.span).into())
    }

    fn parse_identifier_reference(&mut self) -> Result<Expression> {
        let token = self.reader.consume()?;
        let ident: Ident = token.try_into().unwrap();

        // Consume potential trailing semi colon TODO how to handle these in general?
        if self.reader.current()?.value == punct!(";") {
            self.reader.consume()?;
        }

        Ok(IdentifierReference(ident.into()))
    }
}
