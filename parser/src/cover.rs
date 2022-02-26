use crate::error::Result;
use crate::Parser;
use fajt_ast::{Callee, Expr, ExprCall, Ident};
use fajt_common::io::{PeekRead, ReReadWithState};
use fajt_lexer::punct;
use fajt_lexer::token::Token;
use fajt_lexer::token_matches;
use fajt_lexer::LexerState;

impl<I> Parser<'_, I>
where
    I: PeekRead<Token, Error = fajt_lexer::error::Error>,
    I: ReReadWithState<Token, State = LexerState, Error = fajt_lexer::error::Error>,
{
    /// Parses and resolves the `CoverParenthesizedExpressionAndArrowParameterList` production.
    pub(super) fn parse_cover_parenthesized_and_arrow_parameters(&mut self) -> Result<Expr> {
        let span_start = self.position();

        let after_token = self.token_after_parenthesis()?;
        if token_matches!(after_token, opt: punct!("=>")) && !after_token.unwrap().first_on_line {
            let parameters = self.parse_formal_parameters()?;
            self.parse_arrow_function_expr(span_start, false, parameters)
        } else {
            self.parse_parenthesized_expr()
        }
    }

    /// Parses and resolves the `CoverCallExpressionAndAsyncArrowHead` production.
    pub(super) fn parse_cover_call_or_async_arrow_head(
        &mut self,
        async_ident: Ident,
    ) -> Result<Expr> {
        let span_start = async_ident.span.start;
        let after_token = self.token_after_parenthesis()?;
        if token_matches!(after_token, opt: punct!("=>")) {
            let parameters = self.parse_formal_parameters()?;
            self.parse_async_arrow_function_expr(span_start, false, parameters)
        } else {
            let (arguments_span, arguments) = self.parse_arguments()?;
            let span = self.span_from(span_start);
            Ok(ExprCall {
                span,
                callee: Callee::Expr(Box::new(async_ident.into())),
                arguments_span,
                arguments,
            }
            .into())
        }
    }

    /// Assumes next token is start parenthesis, skips past matching closing parenthesis, reads
    /// token, then rewinds so next token is start parenthesis again.
    ///
    /// We do this to figure out the "cover" productions that requires reading past parenthesis.
    fn token_after_parenthesis(&mut self) -> Result<Option<Token>> {
        let start = self.consume_assert(&punct!("("))?;
        self.skip_until_closing_parenthesis()?;

        let token = self.consume().ok();

        self.reader.rewind_to(&start)?;
        Ok(token)
    }

    fn skip_until_closing_parenthesis(&mut self) -> Result<()> {
        let mut depth = 1;
        loop {
            let token = self.consume()?;
            match &token {
                token_matches!(punct!("(")) => depth += 1,
                token_matches!(punct!(")")) => depth -= 1,
                _ => {}
            }

            if depth == 0 {
                break;
            }
        }

        Ok(())
    }
}
