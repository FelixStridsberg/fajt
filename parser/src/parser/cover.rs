use crate::ast::{CoverCallExprAndAsyncArrowHead, CoverParenthesizedAndArrowParameters};
use crate::error::Result;
use crate::Parser;
use fajt_common::io::PeekRead;
use fajt_lexer::keyword;
use fajt_lexer::punct;
use fajt_lexer::token::Token;
use fajt_lexer::token_matches;

impl<I> Parser<'_, I>
where
    I: PeekRead<Token, Error = fajt_lexer::error::Error>,
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
            let token = self.reader.consume()?;
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
