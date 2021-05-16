use crate::ast::{
    ArrayElement, Expression, Ident, Literal, LiteralExpression, Object, PropertyDefinition,
    ThisExpression,
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

impl Parser<'_, '_> {
    pub(crate) fn parse_expression(&mut self) -> Result<Expression> {
        self.parse_assignment_expression()
    }

    pub(crate) fn parse_assignment_expression(&mut self) -> Result<Expression> {
        // TODO other than primary expressions
        self.parse_primary_expression()
    }

    /// Parses the `PrimaryExpression` goal symbol.
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

    /// Parses the `Literal` goal symbol, which is part of `PrimaryExpression`.
    ///
    /// Example:
    /// ```no_rust
    /// return ["success", 200, 0x6A, true];
    ///         ^~~~~~~~^  ^~^  ^~~^  ^~~^
    /// ```
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

    /// Parses the `ArrayLiteral` goal symbol, which is part of `PrimaryExpression`.
    ///
    /// Example:
    /// ```no_rust
    /// var a = [ a, 1, ...spread ];
    ///         ^~~~~~~~~~~~~~~~~~^
    /// ```
    fn parse_array_literal(&mut self) -> Result<Expression> {
        let token = self.reader.consume()?;
        debug_assert!(token_matches!(token, punct!("[")));

        let span_start = token.span.start;

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

        let span_end = self.reader.position();
        let span = Span::new(span_start, span_end);
        Ok(LiteralExpression {
            span,
            literal: Literal::Array(Array { elements }),
        }
        .into())
    }

    /// Parses the `ObjectLiteral` goal symbol, which is part of `PrimaryExpression`.
    ///
    /// Example:
    /// ```no_rust
    /// var a = { a, b: 1, ...spread };
    ///         ^~~~~~~~~~~~~~~~~~~~~^
    /// ```
    fn parse_object_literal(&mut self) -> Result<Expression> {
        let token = self.reader.consume()?;
        debug_assert!(token_matches!(token, punct!("{")));

        let span_start = token.span.start;

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

    /// Parses the `this` expression which is part of the `PrimaryExpression` goal symbol.
    ///
    /// Example:
    /// ```no_rust
    /// this
    /// ^~~^
    /// ```
    fn parse_this_expression(&mut self) -> Result<Expression> {
        let token = self.reader.consume()?;
        debug_assert!(token_matches!(token, keyword!("this")));

        Ok(ThisExpression::new(token.span).into())
    }

    /// Parses the `IdentifierReference` goal symbol, which is part of the `PrimaryExpression`.
    ///
    /// Example:
    /// ```no_rust
    /// var foo = bar;
    ///           ^~^
    /// ```
    fn parse_identifier_reference(&mut self) -> Result<Expression> {
        let ident: Ident = self.parse_identifier()?;

        // Consume potential trailing semi colon TODO how to handle these in general?
        if self.reader.current()?.value == punct!(";") {
            self.reader.consume()?;
        }

        Ok(IdentifierReference(ident.into()))
    }
}
