use crate::ast::CoverParenthesizedAndArrowParameters;
use crate::error::Result;
use crate::Parser;
use fajt_common::io::PeekRead;
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
        let token = self.reader.consume()?;
        debug_assert!(token_matches!(token, punct!("(")));

        let mut tokens = Vec::new();
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

        let span = self.span_from(span_start);
        Ok(CoverParenthesizedAndArrowParameters { span, tokens })
    }
}
