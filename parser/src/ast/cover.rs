//! The `Cover*` goal symbols as unambiguous between more than one goal symbol. They are stored as
//! raw tokens that later can be parsed when the actual goal symbol is determined.
//!
//! These will never appear in the final AST.
use crate::ast::{Expression, FormalParameters, ParenthesizedExpression};
use crate::Parser;
use fajt_common::io::PeekReader;
use fajt_lexer::token::{Span, Token};

#[derive(Debug, PartialOrd, PartialEq)]
pub(crate) struct CoverParenthesizedAndArrowParameters {
    pub span: Span,
    pub tokens: Vec<Token>,
}

impl CoverParenthesizedAndArrowParameters {
    pub fn into_arrow_parameters(self) -> crate::error::Result<Option<FormalParameters>> {
        let tokens = self.tokens.into_iter();
        let mut reader = PeekReader::new(tokens).unwrap();
        let mut parser = Parser::new(&mut reader).unwrap();
        Ok(parser.parse_formal_parameters()?)
    }

    pub fn into_expression(mut self) -> crate::error::Result<Expression> {
        self.tokens.drain(0..1);
        self.tokens.pop();

        let tokens = self.tokens.into_iter();
        let mut reader = PeekReader::new(tokens).unwrap();
        let mut parser = Parser::new(&mut reader).unwrap();

        let expression = parser.parse_expression().unwrap();

        Ok(ParenthesizedExpression {
            span: self.span,
            expression,
        }
        .into())
    }
}
