use crate::ast::{
    DoWhileStatement, ForInit, ForStatement, Statement, VariableKind, VariableStatement,
    WhileStatement,
};
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

        let init = self.parse_for_first_argument()?;
        self.consume_assert(punct!(";"))?; // TODO of, in

        let test = (!self.current_matches(punct!(";"))).then_try(|| self.parse_expression())?;
        self.consume_assert(punct!(";"))?;
        let update = (!self.current_matches(punct!(")"))).then_try(|| self.parse_expression())?;
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
            let declarations = self.parse_variable_declarations()?;

            let span = self.span_from(span_start);
            return Ok(Some(ForInit::Declaration(VariableStatement {
                span,
                kind,
                declarations,
            })));
        }

        Ok(match self.reader.current()? {
            _ if self.current_matches(punct!(";")) => None,
            _ => Some(ForInit::Expression(self.parse_expression()?)),
        })
    }
}
