//! The `Cover*` goal symbols are ambiguous between more than one goal symbol. They are stored as
//! raw tokens that later can be parsed when the actual goal symbol is determined.
//!
//! These will never appear in the final AST.
use crate::ast::{Expr, ExprParenthesized, FormalParameters};
use crate::Parser;
use fajt_common::io::PeekReader;
use fajt_lexer::token::{Span, Token};

#[derive(Debug, PartialOrd, PartialEq)]
pub(crate) struct CoverParenthesizedAndArrowParameters {
    pub span: Span,
    pub tokens: Vec<Token>,
}

impl CoverParenthesizedAndArrowParameters {
    pub fn into_arrow_parameters(self) -> crate::error::Result<FormalParameters> {
        let tokens = self.tokens.into_iter();
        let mut reader = PeekReader::new(tokens).unwrap();
        let mut parser = Parser::new(&mut reader).unwrap();
        parser.parse_formal_parameters()
    }

    pub fn into_expression(mut self) -> crate::error::Result<Expr> {
        self.tokens.drain(0..1);
        self.tokens.pop();

        let tokens = self.tokens.into_iter();
        let mut reader = PeekReader::new(tokens).unwrap();
        let mut parser = Parser::new(&mut reader).unwrap();

        let expr = parser.parse_expression().unwrap();

        Ok(ExprParenthesized {
            span: self.span,
            expression: expr.into(),
        }
        .into())
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub(crate) struct CoverCallExpressionAndAsyncArrowHead {
    pub span: Span,
    pub tokens: Vec<Token>,
}

impl CoverCallExpressionAndAsyncArrowHead {
    pub fn into_arrow_parameters(mut self) -> crate::error::Result<FormalParameters> {
        self.tokens.drain(0..1); // Skip 'async' of 'async (...)'

        let tokens = self.tokens.into_iter();
        let mut reader = PeekReader::new(tokens).unwrap();
        let mut parser = Parser::new(&mut reader).unwrap();
        parser.parse_formal_parameters()
    }

    pub fn into_call(self) -> crate::error::Result<Expr> {
        // This is call expressions like: async(), async(parameters) since async is not reserved.
        todo!("CoverCallExpressionAndAsyncArrowHead to call expression")
    }
}
