use crate::ast::{
    DoWhileStatement, ForInStatement, ForInit, ForOfStatement, ForStatement, Stmt, VariableKind,
    VariableStatement, WhileStatement,
};
use crate::error::ErrorKind::UnexpectedToken;
use crate::error::{Result, ThenTry};
use crate::{ContextModify, Parser};
use fajt_common::io::PeekRead;
use fajt_lexer::keyword;
use fajt_lexer::punct;
use fajt_lexer::token::Token;
use fajt_lexer::token_matches;

impl<I> Parser<'_, I>
where
    I: PeekRead<Token, Error = fajt_lexer::error::Error>,
{
    pub(super) fn parse_do_while_statement(&mut self) -> Result<Stmt> {
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

    pub(super) fn parse_while_statement(&mut self) -> Result<Stmt> {
        let span_start = self.position();
        self.consume_assert(keyword!("while"))?;
        self.consume_assert(punct!("("))?;

        let test = self.parse_expression()?;

        self.consume_assert(punct!(")"))?;

        let body = self.parse_statement()?;

        let span = self.span_from(span_start);
        Ok(WhileStatement { span, test, body }.into())
    }

    pub(super) fn parse_for_statement(&mut self) -> Result<Stmt> {
        let span_start = self.position();
        self.consume_assert(keyword!("for"))?;

        if self.context.is_await && self.current_matches(keyword!("await")) {
            return self.parse_for_await_of(span_start);
        }

        self.consume_assert(punct!("("))?;

        let init = self.parse_for_first_argument()?;
        match self.reader.current() {
            token_matches!(ok: punct!(";")) => self.parse_plain_for(span_start, init),
            token_matches!(ok: keyword!("of")) if init.is_some() => {
                self.parse_for_of(span_start, init.unwrap(), false)
            }
            token_matches!(ok: keyword!("in")) if init.is_some() => {
                self.parse_for_in(span_start, init.unwrap())
            }
            _ => return err!(UnexpectedToken(self.reader.consume()?)),
        }
    }

    fn parse_plain_for(&mut self, span_start: usize, init: Option<ForInit>) -> Result<Stmt> {
        self.consume_assert(punct!(";"))?;

        let test = (!self.current_matches(punct!(";"))).then_try(|| self.parse_expression())?;
        self.consume_assert(punct!(";"))?;
        let update = (!self.current_matches(punct!(")"))).then_try(|| self.parse_expression())?;
        self.consume_assert(punct!(")"))?;

        let body = self.parse_statement()?;
        let span = self.span_from(span_start);
        Ok(ForStatement {
            span,
            init,
            test,
            update,
            body,
        }
        .into())
    }

    fn parse_for_in(&mut self, span_start: usize, left: ForInit) -> Result<Stmt> {
        // TODO verify that `left` is a valid LeftHandSideExpression if not declaration.

        self.consume_assert(keyword!("in"))?;

        let right = self
            .with_context(ContextModify::new().set_in(true))
            .parse_expression()?;

        self.consume_assert(punct!(")"))?;

        let body = self.parse_statement()?;
        let span = self.span_from(span_start);
        Ok(ForInStatement {
            span,
            left,
            right,
            body,
        }
        .into())
    }

    fn parse_for_await_of(&mut self, span_start: usize) -> Result<Stmt> {
        self.reader.consume()?;
        self.consume_assert(punct!("("))?;

        let init = self.parse_for_first_argument()?;
        if init.is_none() {
            return err!(UnexpectedToken(self.reader.consume()?));
        }

        self.parse_for_of(span_start, init.unwrap(), true)
    }

    fn parse_for_of(&mut self, span_start: usize, left: ForInit, wait: bool) -> Result<Stmt> {
        // TODO verify that `left` is a valid LeftHandSideExpression if not declaration.

        self.consume_assert(keyword!("of"))?;

        let right = self
            .with_context(ContextModify::new().set_in(true))
            .parse_expression()?;

        self.consume_assert(punct!(")"))?;

        let body = self.parse_statement()?;
        let span = self.span_from(span_start);
        Ok(ForOfStatement {
            span,
            left,
            right,
            body,
            wait,
        }
        .into())
    }

    pub(super) fn parse_for_first_argument(&mut self) -> Result<Option<ForInit>> {
        let span_start = self.position();
        let variable_kind = match self.reader.current()? {
            token_matches!(keyword!("var")) => Some(VariableKind::Var),
            token_matches!(keyword!("let")) => Some(VariableKind::Let),
            token_matches!(keyword!("const")) => Some(VariableKind::Const),
            _ => None,
        };

        if let Some(kind) = variable_kind {
            self.reader.consume()?; // var, let, const
            let declarations = self
                .with_context(ContextModify::new().set_in(false))
                .parse_variable_declarations()?;

            let span = self.span_from(span_start);
            return Ok(Some(ForInit::Declaration(VariableStatement {
                span,
                kind,
                declarations,
            })));
        }

        Ok(match self.reader.current()? {
            _ if self.current_matches(punct!(";")) => None,
            _ => Some(ForInit::Expression(
                self.with_context(ContextModify::new().set_in(false))
                    .parse_expression()?,
            )),
        })
    }
}
