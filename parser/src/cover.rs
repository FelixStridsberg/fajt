use crate::error::Result;
use crate::Parser;
use fajt_ast::Expr;
use fajt_common::io::{PeekRead, ReReadWithState};
use fajt_lexer::punct;
use fajt_lexer::token::Token;
use fajt_lexer::LexerState;

impl<I> Parser<'_, I>
where
    I: PeekRead<Token, Error = fajt_lexer::error::Error>,
    I: ReReadWithState<Token, State = LexerState, Error = fajt_lexer::error::Error>,
{
    /// Parses and resolves the `CoverParenthesizedExpressionAndArrowParameterList` production.
    pub(super) fn parse_cover_parenthesized_and_arrow_parameters(&mut self) -> Result<Expr> {
        let start_token = self.current()?.clone();
        match self.parse_parenthesized_expr() {
            Ok(expr) if !self.current_matches(&punct!("=>")) => Ok(expr),
            result => {
                self.reader.rewind_to(&start_token)?;

                let arrow_function = self.parse_arrow_function_expr();
                if result.is_err() && arrow_function.is_err() {
                    // TODO make real error handling
                    println!("Error during cover case:\n{:?}", result.unwrap_err());
                }

                arrow_function
            }
        }
    }

    /// Parses and resolves the `CoverCallExpressionAndAsyncArrowHead` production.
    pub(super) fn parse_cover_call_or_async_arrow_head(&mut self) -> Result<Expr> {
        let start_token = self.current()?.clone();
        match self.parse_covered_call_expression() {
            Ok(expr) if !self.current_matches(&punct!("=>")) => Ok(expr),
            result => {
                self.reader.rewind_to(&start_token)?;
                let arrow_function = self.parse_async_arrow_function_expr();
                if result.is_err() && arrow_function.is_err() {
                    // TODO make real error handling
                    println!("Error during cover case:\n{:?}", result.unwrap_err());
                }

                arrow_function
            }
        }
    }

    /// Parses the `CallExpression` covered by `CoverCallExpressionAndAsyncArrowHead`.
    fn parse_covered_call_expression(&mut self) -> Result<Expr> {
        let span_start = self.position();
        let async_ident = self.parse_identifier()?;
        self.parse_call_expr(span_start, async_ident.into())
    }
}
