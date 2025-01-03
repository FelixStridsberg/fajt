use crate::error::ErrorKind::{InitializedNameNotAllowed, UnexpectedToken};
use crate::error::{Error, Result};
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
    /// If an arrow function is parsed, it is returned as an error because it is only valid
    /// higher up in the tree. The error must be caught higher up.
    pub(super) fn parse_cover_parenthesized_and_arrow_parameters(&mut self) -> Result<Expr> {
        let start_token = self.current()?.clone();

        // A parenthesized expression cannot be empty, it must be arrow function.
        if self.peek_matches(&punct!(")")) {
            return Err(Error::arrow_function_not_allowed(
                self.parse_arrow_function_expr()?,
            ));
        }

        match self.parse_parenthesized_expr() {
            Ok(expr) if !self.current_matches(&punct!("=>")) => Ok(expr),
            Ok(_) => {
                self.reader.rewind_to(&start_token)?;
                Err(Error::arrow_function_not_allowed(
                    self.parse_arrow_function_expr()?,
                ))
            }
            Err(error) => match error.kind() {
                &UnexpectedToken(punct!("..."), _) | InitializedNameNotAllowed => {
                    self.reader.rewind_to(&start_token)?;
                    Err(Error::arrow_function_not_allowed(
                        self.parse_arrow_function_expr()?,
                    ))
                }
                _ => Err(error),
            },
        }
    }

    /// Parses and resolves the `CoverCallExpressionAndAsyncArrowHead` production.
    pub(super) fn parse_cover_call_or_async_arrow_head(&mut self) -> Result<Expr> {
        let start_token = self.current()?.clone();
        match self.parse_covered_call_expression() {
            Ok(expr) if !self.current_matches(&punct!("=>")) => Ok(expr),
            Ok(_) => {
                self.reader.rewind_to(&start_token)?;
                self.parse_async_arrow_function_expr()
            }
            error => error,
        }
    }

    /// Parses the `CallExpression` covered by `CoverCallExpressionAndAsyncArrowHead`.
    fn parse_covered_call_expression(&mut self) -> Result<Expr> {
        let span_start = self.position();
        let async_ident = self.parse_identifier()?;
        self.parse_call_expr(span_start, async_ident.into())
    }
}
