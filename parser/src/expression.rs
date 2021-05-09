use crate::ast::{Expr, Ident, Literal, LiteralExpr, ThisExpr};
use crate::error::Result;
use crate::Parser;

use crate::ast::Array;
use crate::ast::Expr::IdentifierReference;
use fajt_lexer::keyword;
use fajt_lexer::punct;
use fajt_lexer::token::TokenValue;
use fajt_lexer::token_matches;
use std::convert::TryInto;

impl Parser<'_> {
    pub(crate) fn parse_expression(&mut self) -> Result<Expr> {
        self.parse_assignment_expression()
    }

    pub(crate) fn parse_assignment_expression(&mut self) -> Result<Expr> {
        // TODO other than primary expressions
        self.parse_primary_expression()
    }

    fn parse_primary_expression(&mut self) -> Result<Expr> {
        Ok(match self.reader.current()? {
            token_matches!(keyword!("this")) => self.parse_this_expression()?,
            token_matches!(keyword!("yield")) => unimplemented!(),
            token_matches!(keyword!("await")) => unimplemented!(),
            token_matches!(keyword!("null")) => self.consume_literal(Literal::Null)?,
            token_matches!(keyword!("true")) => self.consume_literal(Literal::Boolean(true))?,
            token_matches!(keyword!("false")) => self.consume_literal(Literal::Boolean(false))?,
            token_matches!(@literal) => self.parse_literal()?,
            token_matches!(punct!("[")) => self.parse_array_literal()?,
            // TODO ObjectLiteral
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

    fn consume_literal(&mut self, literal: Literal) -> Result<Expr> {
        let token = self.reader.consume()?;
        Ok(LiteralExpr::new(literal, token.span).into())
    }

    fn parse_literal(&mut self) -> Result<Expr> {
        let token = self.reader.consume()?;
        debug_assert!(token_matches!(token, @literal));

        if let TokenValue::Literal(literal) = token.value {
            Ok(LiteralExpr::new(literal.into(), token.span).into())
        } else {
            unreachable!()
        }
    }

    fn parse_array_literal(&mut self) -> Result<Expr> {
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
                    elements.push(None);
                }
                _ => todo!(),
            }
        }

        let span_end = self.reader.position();
        let span = (span_start, span_end);
        Ok(LiteralExpr::new(Literal::Array(Array::new(elements)), span).into())
    }

    fn parse_this_expression(&mut self) -> Result<Expr> {
        let token = self.reader.consume()?;
        Ok(ThisExpr::new(token.span).into())
    }

    fn parse_identifier_reference(&mut self) -> Result<Expr> {
        let token = self.reader.consume()?;
        let ident: Ident = token.try_into().unwrap();

        // Consume potential trailing semi colon TODO how to handle these in general?
        if self.reader.current()?.value == punct!(";") {
            self.reader.consume()?;
        }

        Ok(IdentifierReference(ident.into()))
    }
}
