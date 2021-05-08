use crate::ast::{Expr, Ident, Literal, LiteralExpr, ThisExpr};
use crate::error::Result;
use crate::Parser;

use crate::ast::Expr::IdentifierReference;
use fajt_lexer::keyword;
use fajt_lexer::punct;
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
            // TODO Other literals:
            token_matches!(keyword!("null")) => self.consume_literal(Literal::Null)?,

            // TODO ArrayLiteral
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
