use crate::error::Result;
use crate::Parser;
use fajt_ast::{Callee, Expr, ExprCall, ExprParenthesized, FormalParameters, SourceType, Span};
use fajt_common::io::{PeekRead, PeekReader, ReRead};
use fajt_lexer::punct;
use fajt_lexer::token::Token;
use fajt_lexer::token_matches;
use fajt_lexer::{keyword, LexerState};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct CoverParenthesizedAndArrowParameters {
    pub span: Span,
    #[serde(skip)]
    pub tokens: Vec<Token>,
}

impl CoverParenthesizedAndArrowParameters {
    pub fn into_arrow_parameters(self) -> Result<FormalParameters> {
        let tokens = self.tokens.into_iter();
        let mut reader = PeekReader::new(tokens).unwrap();
        let mut parser = Parser::new(&mut reader, SourceType::Unknown).unwrap();
        parser.parse_formal_parameters()
    }

    pub fn into_expr(mut self) -> Result<Expr> {
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
    }
}

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct CoverCallExprAndAsyncArrowHead {
    pub span: Span,
    #[serde(skip)]
    pub tokens: Vec<Token>,
}

impl CoverCallExprAndAsyncArrowHead {
    pub fn into_arrow_parameters(mut self) -> Result<FormalParameters> {
        self.tokens.drain(0..1); // Skip 'async' of 'async (...)'

        let tokens = self.tokens.into_iter();
        let mut reader = PeekReader::new(tokens).unwrap();
        let mut parser = Parser::new(&mut reader, SourceType::Unknown).unwrap();
        parser.parse_formal_parameters()
    }

    pub fn into_call(self) -> Result<Expr> {
        let tokens = self.tokens.into_iter();
        let mut reader = PeekReader::new(tokens).unwrap();
        let mut parser = Parser::new(&mut reader, SourceType::Unknown).unwrap();

        let async_ident = parser.parse_identifier()?;
        let (arguments_span, arguments) = parser.parse_arguments()?;
        Ok(ExprCall {
            span: self.span,
            callee: Callee::Expr(async_ident.into()),
            arguments_span,
            arguments,
        }
        .into())
    }
}

impl<I> Parser<'_, I>
where
    I: PeekRead<Token, Error = fajt_lexer::error::Error>,
    I: ReRead<Token, State = LexerState, Error = fajt_lexer::error::Error>,
{
    /// Parses the `CoverParenthesizedExpressionAndArrowParameterList` goal symbol.
    pub(super) fn parse_cover_parenthesized_and_arrow_parameters(
        &mut self,
    ) -> Result<CoverParenthesizedAndArrowParameters> {
        let span_start = self.position();
        let mut tokens = Vec::new();

        self.collect_parenthesized_tokens(&mut tokens)?;

        let span = self.span_from(span_start);
        Ok(CoverParenthesizedAndArrowParameters { span, tokens })
    }

    /// Parses the `CoverCallExpressionAndAsyncArrowHead` goal symbol.
    pub(super) fn parse_cover_call_and_async_arrow_head(
        &mut self,
    ) -> Result<CoverCallExprAndAsyncArrowHead> {
        let async_token = self.consume_assert(keyword!("async"))?;
        let span_start = self.position();
        let mut tokens = vec![async_token];

        self.collect_parenthesized_tokens(&mut tokens)?;

        let span = self.span_from(span_start);
        Ok(CoverCallExprAndAsyncArrowHead { span, tokens })
    }

    fn collect_parenthesized_tokens(&mut self, tokens: &mut Vec<Token>) -> Result<()> {
        let token = self.consume_assert(punct!("("))?;
        tokens.push(token);

        let mut depth = 1;
        loop {
            let token = self.consume()?;
            match &token {
                token_matches!(punct!("(")) => depth += 1,
                token_matches!(punct!(")")) => depth -= 1,
                _ => {}
            }

            tokens.push(token);

            if depth == 0 {
                break;
            }
        }

        Ok(())
    }
}
