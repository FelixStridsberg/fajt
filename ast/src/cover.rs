//! The `Cover*` goal symbols are ambiguous between more than one goal symbol. They are stored as
//! raw tokens that later can be parsed when the actual goal symbol is determined.
//!
//! These will never appear in the final AST.
use crate::{Expr, ExprParenthesized, FormalParameters};
//use fajt_parser::parser::SourceType;
//use fajt_parser::parser::Parser;
use fajt_common::io::PeekReader;
use fajt_lexer::token::{Span, Token};

ast_struct! {
    pub struct CoverParenthesizedAndArrowParameters {
        pub span: Span,
        #[serde(skip)]
        pub tokens: Vec<Token>,
    }
}

impl CoverParenthesizedAndArrowParameters {
    pub fn into_arrow_parameters(self) /* -> fajt_parser::error::Result<FormalParameters> */
    {
        todo!()
        /*
        let tokens = self.tokens.into_iter();
        let mut reader = PeekReader::new(tokens).unwrap();
        let mut parser = Parser::new(&mut reader, SourceType::Unknown).unwrap();
        parser.parse_formal_parameters()
         */
    }

    pub fn into_expr(mut self) /* -> fajt_parser::error::Result<Expr> */
    {
        todo!()
        /*
        self.tokens.drain(0..1);
        self.tokens.pop();

        let tokens = self.tokens.into_iter();
        let mut reader = PeekReader::new(tokens).unwrap();
        let mut parser = Parser::new(&mut reader, SourceType::Unknown).unwrap();

        let expr = parser.parse_expr().unwrap();

        Ok(ExprParenthesized {
            span: self.span,
            expression: expr.into(),
        }
        .into())
         */
    }
}

ast_struct! {
    pub struct CoverCallExprAndAsyncArrowHead {
        pub span: Span,
        #[serde(skip)]
        pub tokens: Vec<Token>,
    }
}

impl CoverCallExprAndAsyncArrowHead {
    pub fn into_arrow_parameters(mut self) /* -> fajt_parser::error::Result<FormalParameters>*/
    {
        todo!()
        /*
        self.tokens.drain(0..1); // Skip 'async' of 'async (...)'

        let tokens = self.tokens.into_iter();
        let mut reader = PeekReader::new(tokens).unwrap();
        let mut parser = Parser::new(&mut reader, SourceType::Unknown).unwrap();
        parser.parse_formal_parameters()
         */
    }

    pub fn into_call(self) /*-> fajt_parser::error::Result<Expr> */
    {
        // This is call expressions like: async(), async(parameters) since async is not reserved.
        todo!("CoverCallExpressionAndAsyncArrowHead to call expression")
    }
}
