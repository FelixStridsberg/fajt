use crate::ast::{DoWhileStatement, Statement};
use crate::error::Result;
use crate::Parser;
use fajt_common::io::PeekRead;
use fajt_lexer::keyword;
use fajt_lexer::punct;
use fajt_lexer::token::Token;

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
}
