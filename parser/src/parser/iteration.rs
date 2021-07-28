use crate::ast::{DoWhileStatement, ForStatement, Statement, WhileStatement};
use crate::error::{Result, ThenTry};
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
    pub(super) fn parse_do_while_statement(&mut self) -> Result<Statement> {
        let span_start = self.position();
        self.consume_assert(keyword!("do"))?;

        let body = self.parse_statement()?;

        self.consume_assert(keyword!("while"))?;
        self.consume_assert(punct!("("))?;

        let test = self.parse_expression()?;

        self.consume_assert(punct!(")"))?;

        let span = self.span_from(span_start);
        Ok(DoWhileStatement { span, body, test }.into())
    }

    pub(super) fn parse_while_statement(&mut self) -> Result<Statement> {
        let span_start = self.position();
        self.consume_assert(keyword!("while"))?;
        self.consume_assert(punct!("("))?;

        let test = self.parse_expression()?;

        self.consume_assert(punct!(")"))?;

        let body = self.parse_statement()?;

        let span = self.span_from(span_start);
        Ok(WhileStatement { span, test, body }.into())
    }

    pub(super) fn parse_for_statement(&mut self) -> Result<Statement> {
        let span_start = self.position();
        self.consume_assert(keyword!("for"))?;

        // TODO await

        self.consume_assert(punct!("("))?;

        match self.reader.current()? {
            token_matches!(keyword!("var")) => todo!("for ( var ..."),
            token_matches!(keyword!("let")) => todo!("for ( let ..."),
            token_matches!(keyword!("const")) => todo!("for ( const ..."),
            _ => {
                // TODO clean up
                let init =
                    (!self.current_matches(punct!(";"))).then_try(|| self.parse_expression())?;
                self.consume_assert(punct!(";"))?;
                let test =
                    (!self.current_matches(punct!(";"))).then_try(|| self.parse_expression())?;
                self.consume_assert(punct!(";"))?;
                let update =
                    (!self.current_matches(punct!(")"))).then_try(|| self.parse_expression())?;
                self.consume_assert(punct!(")"))?;

                let body = self.parse_statement()?;

                let span = self.span_from(span_start);
                return Ok(ForStatement {
                    span,
                    init,
                    test,
                    update,
                    body,
                }
                .into());
            }
        }
    }
}
